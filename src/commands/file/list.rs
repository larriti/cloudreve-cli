use cloudreve_api::{CloudreveAPI, Result};
use log::info;
use crate::utils::format_bytes;

pub async fn handle_list(
    api: &CloudreveAPI,
    path: String,
    _page: Option<u32>,
    _page_size: Option<u32>,
) -> Result<()> {
    info!("Listing files in path: {}", path);

    let file_list = api.list_files(&path).await?;

    // Display parent directory information
    info!("📂 Parent: {}", file_list.parent_name());

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
