use cloudreve_api::api::v4::models::{FileType, ListFilesRequest};
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use crate::utils::format_bytes;

pub async fn handle_list(
    client: &CloudreveClient,
    path: String,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<()> {
    info!("Listing files in path: {}", path);
    let request = ListFilesRequest {
        path: &path,
        page,
        page_size,
        ..Default::default()
    };

    match client.list_files(&request).await {
        Ok(response) => {
            // Display parent directory information
            info!("📂 Parent: {}", response.parent.name);

            // Display pagination info
            if let Some(total) = response.pagination.total_items {
                let total_pages = ((total as f64 / response.pagination.page_size as f64).ceil() as i32).max(1);
                info!("📄 Page {}/{} (showing {} of {} items)",
                      response.pagination.page + 1,
                      total_pages,
                      response.files.len(),
                      total);
            } else {
                info!("📄 Page {} (showing {} items)",
                      response.pagination.page + 1,
                      response.files.len());
            }

            // Display storage policy if present
            if let Some(policy) = &response.storage_policy {
                info!("💾 Storage Policy: {} ({})", policy.name, policy.id);
            }

            // Display capability hints
            info!("🔧 Capabilities: {} (max page size: {})",
                  response.props.capability,
                  response.props.max_page_size);

            info!("");
            info!("📁 Files:");
            for file in response.files {
                match file.r#type {
                    FileType::Folder => info!("  📁 {}/", file.name),
                    FileType::File =>   info!("  📄 {} ({})", file.name, format_bytes(file.size)),
                }
            }

            // Display next page hint
            if let Some(token) = response.pagination.next_token {
                info!("");
                info!("📌 Next page token available: {}", token);
            }
        }
        Err(e) => {
            error!("Error listing files: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
