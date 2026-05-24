use cloudreve_api::Result;
use cloudreve_api::api::v3::{ApiV3Client, models::FileSourceRequest};
use cloudreve_api::api::v4::{ApiV4Client, models::CreateDirectLinkRequest};
use log::{error, info};

/// Create direct links for files (V3)
/// Supports both file IDs and file paths
pub async fn handle_create_link_v3(client: &ApiV3Client, inputs: Vec<String>) -> Result<()> {
    info!(
        "Creating direct links for {} file(s) (V3 API)",
        inputs.len()
    );

    // Convert paths to IDs if needed
    let mut ids = Vec::new();
    for input in &inputs {
        // If input looks like an ID (short alphanumeric), use it directly
        if input.len() < 20 && !input.contains('/') {
            ids.push(input.clone());
        } else {
            // Otherwise, treat it as a path and resolve to ID
            let file_id = resolve_path_to_id(client, input).await?;
            ids.push(file_id);
        }
    }

    let request = FileSourceRequest { items: ids };

    match client.get_file_source(&request).await {
        Ok(sources) => {
            info!("Direct links created successfully!");
            for item in sources {
                info!("  Name: {}", item.name);
                // Build full URL if the returned URL is a relative path
                let full_url = if item.url.starts_with('/') {
                    format!("{}{}", client.base_url.trim_end_matches('/'), item.url)
                } else {
                    item.url.clone()
                };
                info!("  Link: {}", full_url);
            }
        }
        Err(e) => {
            error!("Error creating direct links: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Resolve a file path to a file ID in V3 API
async fn resolve_path_to_id(client: &ApiV3Client, path: &str) -> Result<String> {
    // Split path into directory and filename
    let (dir_path, file_name) = if let Some(last_slash) = path.rfind('/') {
        let dir = if last_slash == 0 {
            "/"
        } else {
            &path[..last_slash]
        };
        let file = &path[last_slash + 1..];
        (dir, file)
    } else {
        // No directory, use root
        ("/", path)
    };

    // List directory contents
    let dir_list = client.list_directory(dir_path).await?;

    // Find the file by name
    for obj in dir_list.objects {
        if obj.name == file_name {
            return Ok(obj.id);
        }
    }

    Err(cloudreve_api::Error::InvalidResponse(format!(
        "File not found: {} in directory {}",
        file_name, dir_path
    )))
}

/// Create direct links for files (V4)
pub async fn handle_create_link_v4(client: &ApiV4Client, paths: Vec<String>) -> Result<()> {
    info!("Creating direct links for {} file(s) (V4 API)", paths.len());

    let uris: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    let request = CreateDirectLinkRequest { uris };

    match client.create_direct_link(&request).await {
        Ok(links) => {
            info!("Direct links created successfully!");
            for item in links {
                info!("  File: {}", item.file_url);
                info!("  Link: {}", item.link);
            }
        }
        Err(e) => {
            error!("Error creating direct links: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Delete a direct link (V4 only - V3 doesn't support deleting)
pub async fn handle_delete_link_v4(client: &ApiV4Client, id: String) -> Result<()> {
    info!("Deleting direct link: {}", id);

    match client.delete_direct_link(&id).await {
        Ok(()) => {
            info!("Direct link deleted successfully!");
        }
        Err(e) => {
            error!("Error deleting direct link: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
