use cloudreve_api::api::v4::models::{CreateUploadSessionRequest, ListFilesRequest};
use cloudreve_api::{CloudreveClient, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info, warn};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::time::{sleep, Duration};

/// Resolves the storage policy ID to use for upload
/// Priority: directory policy > user specified > first available
async fn resolve_storage_policy(
    client: &CloudreveClient,
    upload_path: &str,
    user_policy: Option<String>,
) -> Result<String> {
    // Step 1: Try to get directory's preferred policy
    let dir_path = Path::new(upload_path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("/");

    info!("Checking storage policy for directory: {}", dir_path);

    let list_request = ListFilesRequest {
        path: dir_path,
        page: Some(0),
        page_size: Some(1), // Minimal request, we only need policy info
        ..Default::default()
    };

    match client.list_files(&list_request).await {
        Ok(list_response) => {
            if let Some(policy) = list_response.storage_policy {
                info!("Using directory storage policy: {} ({})", policy.name, policy.id);
                return Ok(policy.id);
            }
            info!("Directory has no preferred storage policy");
        }
        Err(e) => {
            warn!("Could not fetch directory storage policy: {}", e);
        }
    }

    // Step 2: Use user-specified policy
    if let Some(pid) = user_policy {
        info!("Using user-specified storage policy: {}", pid);
        return Ok(pid);
    }

    // Step 3: Fetch and use first available policy
    info!("Fetching available storage policies...");
    match client.get_storage_policies().await {
        Ok(policies) if !policies.is_empty() => {
            let first_policy = &policies[0];
            info!(
                "Using first available storage policy: {} ({})",
                first_policy.name, first_policy.id
            );
            Ok(first_policy.id.clone())
        }
        Ok(_) => {
            error!("No storage policies available");
            Err(cloudreve_api::Error::Api {
                code: 400,
                message: "No storage policies available. Use --policy to specify one.".to_string(),
            })
        }
        Err(e) => {
            error!("Could not fetch storage policies: {}", e);
            Err(e)
        }
    }
}

pub async fn handle_upload(
    client: &CloudreveClient,
    file: String,
    path: String,
    overwrite: bool,
    policy_id: Option<String>,
) -> Result<()> {
    info!("Uploading file: {} to path: {}", file, path);

    // 1. Verify local file
    let file_path = Path::new(&file);
    if !file_path.exists() {
        error!("Error: File '{}' does not exist", file);
        return Err(cloudreve_api::Error::Api {
            code: 404,
            message: format!("File '{}' does not exist", file),
        });
    }

    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let file_size = file_path.metadata()?.len();
    info!("File size: {} bytes", file_size);

    // 2. Resolve storage policy with directory preference
    let policy_id = resolve_storage_policy(client, &path, policy_id).await?;

    // 3. Build path for upload (API layer handles URI conversion)
    let upload_path = if path.ends_with('/') || path.is_empty() {
        format!("{}{}", path, file_name)
    } else {
        format!("{}/{}", path, file_name)
    };

    // 4. Create upload session
    let session_request = CreateUploadSessionRequest {
        uri: &upload_path,
        size: file_size,
        policy_id: &policy_id,
        last_modified: None,
        mime_type: None,
        metadata: None,
        entity_type: if overwrite { Some("version") } else { None },
    };

    let session = client.create_upload_session(&session_request).await?;
    info!("Upload session created: {}", session.session_id);

    // Handle chunk_size being 0 (upload entire file at once)
    let chunk_size = if session.chunk_size == 0 {
        file_size as usize
    } else {
        session.chunk_size as usize
    };

    let total_chunks = if session.chunk_size == 0 {
        1
    } else {
        file_size.div_ceil(session.chunk_size)
    } as u32;

    info!(
        "Chunk size: {} bytes, Total chunks: {}",
        chunk_size, total_chunks
    );

    // 5. Read file and upload in chunks
    let mut file_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut file_content)?;

    // Create progress bar
    let pb = ProgressBar::new(total_chunks as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} chunks ({eta})")
        .unwrap()
        .progress_chars("=>-"));
    pb.set_message("Uploading");

    for (index, chunk) in file_content.chunks(chunk_size).enumerate() {
        // API requires chunk index to start from 0
        let chunk_index = index as u32;

        debug!(
            "Uploading chunk {}/{} (size: {} bytes)",
            chunk_index + 1,
            total_chunks,
            chunk.len()
        );

        // Retry mechanism
        let mut retries = 3;
        loop {
            match client
                .upload_file_chunk(&session.session_id, chunk_index, chunk)
                .await
            {
                Ok(_) => {
                    pb.inc(1);
                    break;
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        error!(
                            "Failed to upload chunk {} after 3 retries: {}",
                            chunk_index + 1, e
                        );
                        // Clean up upload session
                        let _ = client.delete_upload_session(&upload_path, &session.session_id).await;
                        return Err(e);
                    }
                    warn!(
                        "Retry {} uploading chunk {}: {}",
                        4 - retries,
                        chunk_index + 1,
                        e
                    );
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pb.finish_with_message("Upload completed!");
    info!("{} chunks uploaded.", total_chunks);
    Ok(())
}
