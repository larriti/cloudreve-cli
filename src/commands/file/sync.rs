use cloudreve_api::api::v4::models::ListFilesRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use crate::utils::format_bytes;
use chrono::{DateTime, Utc};

pub enum SyncDirection {
    Up,    // Local -> Remote
    Down,  // Remote -> Local
    Both,  // Bidirectional
}

pub async fn handle_sync(
    client: &CloudreveClient,
    local_path: String,
    remote_path: String,
    direction: String,
    dry_run: bool,
) -> Result<()> {
    info!("Syncing: {} <-> {}", local_path, remote_path);

    let sync_dir = match direction.as_str() {
        "up" => SyncDirection::Up,
        "down" => SyncDirection::Down,
        "both" => SyncDirection::Both,
        _ => {
            error!("Invalid direction: {}. Use 'up', 'down', or 'both'", direction);
            return Err(cloudreve_api::Error::Api {
                code: 400,
                message: format!("Invalid direction: {}", direction),
            });
        }
    };

    if dry_run {
        info!("DRY RUN MODE - No actual changes will be made");
    }

    match sync_dir {
        SyncDirection::Up => sync_up(client, &local_path, &remote_path, dry_run).await?,
        SyncDirection::Down => sync_down(client, &local_path, &remote_path, dry_run).await?,
        SyncDirection::Both => sync_both(client, &local_path, &remote_path, dry_run).await?,
    }

    Ok(())
}

async fn sync_up(
    client: &CloudreveClient,
    local_path: &str,
    remote_path: &str,
    dry_run: bool,
) -> Result<()> {
    info!("Sync UP: {} -> {}", local_path, remote_path);

    // Get local files
    let local_files = scan_local_directory(local_path)
        .map_err(|e| cloudreve_api::Error::Api {
            code: 500,
            message: format!("Failed to scan local directory: {}", e),
        })?;

    // Get remote files
    let remote_files = list_remote_files(client, remote_path).await?;
    let remote_map: HashMap<_, _> = remote_files
        .into_iter()
        .map(|f| (f.name.clone(), f))
        .collect();

    let mut upload_count = 0;
    let mut skip_count = 0;
    let mut total_size = 0u64;

    for local_file in local_files {
        let name = &local_file.name;
        if let Some(remote_file) = remote_map.get(name) {
            // Compare modification times
            let local_time = local_file.modified;
            let remote_time = parse_remote_time(&remote_file.updated_at);

            if local_time > remote_time {
                if dry_run {
                    info!("[DRY RUN] Would upload: {} (newer: {} > {})",
                        name, local_time, remote_time);
                } else {
                    info!("Uploading: {} (newer)", name);
                    // TODO: Implement actual upload
                }
                upload_count += 1;
                total_size += local_file.size;
            } else {
                info!("Skipping: {} (remote is up to date)", name);
                skip_count += 1;
            }
        } else {
            if dry_run {
                info!("[DRY RUN] Would upload: {} (new file)", name);
            } else {
                info!("Uploading: {} (new file)", name);
                // TODO: Implement actual upload
            }
            upload_count += 1;
            total_size += local_file.size;
        }
    }

    info!("");
    info!("Sync UP summary:");
    info!("  Would upload: {} files", upload_count);
    info!("  Skipped: {} files", skip_count);
    info!("  Total size: {}", format_bytes(total_size as i64));

    Ok(())
}

async fn sync_down(
    client: &CloudreveClient,
    local_path: &str,
    remote_path: &str,
    dry_run: bool,
) -> Result<()> {
    info!("Sync DOWN: {} -> {}", remote_path, local_path);

    // Get remote files
    let remote_files = list_remote_files(client, remote_path).await?;

    // Get local files map
    let local_map = scan_local_directory_to_map(local_path)
        .map_err(|e| cloudreve_api::Error::Api {
            code: 500,
            message: format!("Failed to scan local directory: {}", e),
        })?;

    let mut download_count = 0;
    let mut skip_count = 0;
    let mut total_size = 0u64;

    for remote_file in remote_files {
        let name = &remote_file.name;

        if let Some(local_file) = local_map.get(name) {
            // Compare modification times
            let local_time = local_file.modified;
            let remote_time = parse_remote_time(&remote_file.updated_at);

            if remote_time > local_time {
                if dry_run {
                    info!("[DRY RUN] Would download: {} (newer: {} > {})",
                        name, remote_time, local_time);
                } else {
                    info!("Downloading: {} (newer)", name);
                    // TODO: Implement actual download
                }
                download_count += 1;
                total_size += remote_file.size as u64;
            } else {
                info!("Skipping: {} (local is up to date)", name);
                skip_count += 1;
            }
        } else {
            if dry_run {
                info!("[DRY RUN] Would download: {} (new file)", name);
            } else {
                info!("Downloading: {} (new file)", name);
                // TODO: Implement actual download
            }
            download_count += 1;
            total_size += remote_file.size as u64;
        }
    }

    info!("");
    info!("Sync DOWN summary:");
    info!("  Would download: {} files", download_count);
    info!("  Skipped: {} files", skip_count);
    info!("  Total size: {}", format_bytes(total_size as i64));

    Ok(())
}

async fn sync_both(
    _client: &CloudreveClient,
    _local_path: &str,
    _remote_path: &str,
    _dry_run: bool,
) -> Result<()> {
    info!("Sync BOTH (bidirectional)");

    // This would implement more sophisticated bidirectional sync
    // For now, show a message
    warn!("Bidirectional sync is not fully implemented yet");
    info!("Consider using 'up' or 'down' direction for one-way sync");

    Ok(())
}

#[derive(Debug)]
struct LocalFileInfo {
    name: String,
    size: u64,
    modified: DateTime<Utc>,
}

fn scan_local_directory(path: &str) -> std::result::Result<Vec<LocalFileInfo>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            let modified: DateTime<Utc> = metadata.modified()?.into();
            let size = metadata.len();

            files.push(LocalFileInfo {
                name,
                size,
                modified,
            });
        }
    }

    Ok(files)
}

fn scan_local_directory_to_map(path: &str) -> std::result::Result<HashMap<String, LocalFileInfo>, Box<dyn std::error::Error>> {
    let files = scan_local_directory(path)?;
    Ok(files.into_iter().map(|f| (f.name.clone(), f)).collect())
}

async fn list_remote_files(
    client: &CloudreveClient,
    path: &str,
) -> Result<Vec<cloudreve_api::api::v4::models::File>> {
    let request = ListFilesRequest {
        path,
        page: Some(0),
        page_size: Some(1000),
        ..Default::default()
    };

    let response = client.list_files(&request).await?;
    Ok(response.files)
}

fn parse_remote_time(time_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(time_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
