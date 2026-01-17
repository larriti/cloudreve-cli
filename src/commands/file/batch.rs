use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};
use log::{error, info, warn};
use std::fs;
use std::path::Path;
use crate::utils::format_bytes;
use super::upload::handle_upload;
use super::download::handle_download;

/// Batch upload files
pub async fn handle_batch_upload(
    api: &CloudreveAPI,
    paths: Vec<String>,
    dest_path: String,
    overwrite: bool,
    policy_id: Option<String>,
    recursive: bool,
) -> Result<()> {
    // Batch upload is only supported in V4
    match api.inner() {
        UnifiedClient::V3(_) => {
            return Err(cloudreve_api::Error::InvalidResponse(
                "Batch upload not available in V3 API".to_string()
            ));
        }
        UnifiedClient::V4(_) => {}
    }

    info!("Starting batch upload of {} items", paths.len());

    let mut uploaded = 0usize;
    let mut failed = 0usize;
    let mut total_size = 0u64;

    for path in paths {
        let path_obj = Path::new(&path);

        if path_obj.is_file() {
            match handle_upload(api, path.clone(), dest_path.clone(), overwrite, policy_id.clone()).await {
                Ok(_) => {
                    uploaded += 1;
                    // Get file size
                    if let Ok(metadata) = path_obj.metadata() {
                        total_size += metadata.len();
                    }
                    info!("Uploaded: {}", path);
                }
                Err(e) => {
                    failed += 1;
                    error!("Failed: {} - {}", path, e);
                }
            }
        } else if path_obj.is_dir() && recursive {
            match upload_directory(api, path_obj, &dest_path, overwrite, policy_id.as_deref(), &mut total_size).await {
                Ok(count) => {
                    uploaded += count;
                }
                Err(e) => {
                    error!("Failed directory {}: {}", path, e);
                    failed += 1;
                }
            }
        } else {
            warn!("Skipping: {} (not a file or recursive not enabled)", path);
        }
    }

    info!("");
    info!("Batch upload summary:");
    info!("  Uploaded: {} files", uploaded);
    info!("  Failed: {} files", failed);
    info!("  Total size: {}", format_bytes(total_size as i64));

    Ok(())
}

async fn upload_directory(
    api: &CloudreveAPI,
    dir_path: &Path,
    dest_path: &str,
    overwrite: bool,
    policy_id: Option<&str>,
    total_size: &mut u64,
) -> Result<usize> {
    let mut count = 0;

    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_str = path.to_str().ok_or_else(||
                cloudreve_api::Error::Api {
                    code: 400,
                    message: "Invalid file path".to_string()
                }
            )?;

            match handle_upload(api, file_str.to_string(), dest_path.to_string(), overwrite, policy_id.map(|s| s.to_string())).await {
                Ok(_) => {
                    count += 1;
                    if let Ok(metadata) = path.metadata() {
                        *total_size += metadata.len();
                    }
                }
                Err(e) => {
                    error!("Failed to upload {}: {}", file_str, e);
                }
            }
        } else if path.is_dir() {
            // Recursively upload subdirectories
            let dir_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let new_dest = format!("{}/{}/", dest_path.trim_end_matches('/'), dir_name);
            let upload_future = Box::pin(upload_directory(
                api,
                &path,
                &new_dest,
                overwrite,
                policy_id,
                total_size,
            ));
            if let Ok(sub_count) = upload_future.await {
                count += sub_count;
            }
        }
    }

    Ok(count)
}

/// Batch download files
pub async fn handle_batch_download(
    api: &CloudreveAPI,
    uris: Vec<String>,
    output_dir: String,
    expires_in: Option<u32>,
) -> Result<()> {
    // Batch download is only supported in V4
    match api.inner() {
        UnifiedClient::V3(_) => {
            return Err(cloudreve_api::Error::InvalidResponse(
                "Batch download not available in V3 API".to_string()
            ));
        }
        UnifiedClient::V4(_) => {}
    }

    info!("Starting batch download of {} items", uris.len());

    let mut downloaded = 0usize;
    let mut failed = 0usize;
    let mut total_size = 0u64;

    for uri in uris {
        match handle_download(api, uri.clone(), output_dir.clone(), expires_in).await {
            Ok(_) => {
                downloaded += 1;
                // Try to get downloaded file size
                let filename = uri.split('/').next_back().unwrap_or("file");
                let output_path = Path::new(&output_dir).join(filename);
                if output_path.exists() {
                    if let Ok(metadata) = output_path.metadata() {
                        total_size += metadata.len();
                    }
                }
                info!("Downloaded: {}", uri);
            }
            Err(e) => {
                failed += 1;
                error!("Failed: {} - {}", uri, e);
            }
        }
    }

    info!("");
    info!("Batch download summary:");
    info!("  Downloaded: {} items", downloaded);
    info!("  Failed: {} items", failed);
    info!("  Total size: {}", format_bytes(total_size as i64));

    Ok(())
}
