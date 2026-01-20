use crate::utils::format_bytes;
use cloudreve_api::{CloudreveAPI, FileListAll, Result};
use log::info;

pub async fn handle_list(
    api: &CloudreveAPI,
    path: String,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<()> {
    info!("Listing files in path: {}", path);

    // If page is specified, use single page listing; otherwise fetch all pages
    let file_list_all = if page.is_some() {
        // Single page request
        let page_size = page_size.unwrap_or(100);
        let single_page = api.list_files(&path, page, Some(page_size)).await?;

        // Convert FileList to FileListAll for uniform handling
        match single_page {
            cloudreve_api::FileList::V3(d) => FileListAll::V3(d),
            cloudreve_api::FileList::V4(r) => FileListAll::V4(r),
        }
    } else {
        // Fetch all pages with automatic pagination
        api.list_files_all(&path, page_size).await?
    };

    // Display parent directory information
    info!(
        "📂 Parent: {} (ID: {})",
        file_list_all.parent_name(),
        file_list_all.parent_id()
    );
    let parent_path = file_list_all.parent_path();
    if !parent_path.is_empty() {
        info!("   URI: {}", parent_path);
    }

    // Display storage policy if available (V4 only)
    if let Some(policy_name) = file_list_all.storage_policy_name() {
        let policy_id = file_list_all.storage_policy_id().unwrap_or_default();
        info!("   Storage Policy: {} (ID: {})", policy_name, policy_id);
    }

    // Display files and folders
    info!("");
    info!("📁 Files:");
    for item in file_list_all.items() {
        if item.is_folder {
            info!("  📁 {}/", item.name);
        } else {
            info!("  📄 {} ({})", item.name, format_bytes(item.size));
        }
    }

    info!("");
    let total_count = file_list_all.total_count();
    info!("Total: {} items", total_count);

    // Show if this is a paginated result
    if let Some(total_items) = file_list_all.total_items()
        && total_items as usize > total_count
    {
        info!(
            "(Showing {} out of {} total items)",
            total_count, total_items
        );
    }

    info!("API Version: {}", api.api_version());

    Ok(())
}
