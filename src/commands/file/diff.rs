use crate::utils::format_bytes;
use chrono::{DateTime, Utc};
use cloudreve_api::api::v4::models::GetFileInfoRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use std::fs;

pub async fn handle_diff(
    client: &CloudreveClient,
    local_path: String,
    remote_uri: String,
) -> Result<()> {
    info!("Comparing: {} <-> {}", local_path, remote_uri);

    // Get local file info
    let local_metadata = fs::metadata(&local_path)?;
    let local_size = local_metadata.len();
    let local_modified: DateTime<Utc> = local_metadata.modified()?.into();

    // Get remote file info
    let request = GetFileInfoRequest {
        uri: &remote_uri,
        include_extended_info: Some(false),
    };
    let remote_info = client.get_file_info_extended(&request).await?;

    // Display comparison
    info!("");
    info!("File Comparison:");
    info!("  Local:  {}", local_path);
    info!("  Remote: {} ({})", remote_info.name, remote_uri);
    info!("");

    // Size comparison
    info!("Size comparison:");
    info!("  Local:  {}", format_bytes(local_size as i64));
    info!("  Remote: {}", format_bytes(remote_info.size));

    let size_diff = (local_size as i64 - remote_info.size).abs();
    if local_size as i64 != remote_info.size {
        info!(
            "  ⚠ Difference: {} ({} bytes)",
            format_bytes(size_diff),
            size_diff
        );
    } else {
        info!("  ✓ Sizes match");
    }
    info!("");

    // Modification time comparison
    info!("Modification time:");
    info!(
        "  Local:  {}",
        local_modified.format("%Y-%m-%d %H:%M:%S UTC")
    );
    let remote_time = parse_remote_time(&remote_info.updated_at);
    info!("  Remote: {}", remote_time.format("%Y-%m-%d %H:%M:%S UTC"));

    let time_diff = (local_modified - remote_time).num_seconds().abs();
    if time_diff > 1 {
        info!("  ⚠ Difference: {} seconds", time_diff);
    } else {
        info!("  ✓ Times match");
    }
    info!("");

    // Summary
    info!("Summary:");
    let files_match = local_size as i64 == remote_info.size && time_diff <= 1;
    if files_match {
        info!("  ✓ Files appear to be identical");
    } else {
        info!("  ⚠ Files differ");
    }

    Ok(())
}

fn parse_remote_time(time_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(time_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
