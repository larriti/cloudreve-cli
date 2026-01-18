use crate::utils::format_bytes;
use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_list(
    api: &CloudreveAPI,
    path: String,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<()> {
    info!("Listing files in path: {}", path);

    // Default page_size to 100 if not specified
    let page_size = page_size.unwrap_or(100);
    let file_list = api.list_files(&path, page, Some(page_size)).await?;

    // Display parent directory information
    info!("📂 Parent: {} (ID: {})", file_list.parent_name(), file_list.parent_id());
    let parent_path = file_list.parent_path();
    if !parent_path.is_empty() {
        info!("   URI: {}", parent_path);
    }

    // Display storage policy if available (V4 only)
    if let Some(policy_name) = file_list.storage_policy_name() {
        let policy_id = file_list.storage_policy_id().unwrap_or_default();
        info!("   Storage Policy: {} (ID: {})", policy_name, policy_id);
    }

    // Display files and folders
    info!("");
    info!("📁 Files:");
    for item in file_list.items() {
        if item.is_folder {
            info!("  📁 {}/", item.name);
        } else {
            info!("  📄 {} ({})", item.name, format_bytes(item.size));
        }
    }

    info!("");
    info!("Total: {} items", file_list.total_count());
    info!("API Version: {}", api.api_version());

    Ok(())
}
