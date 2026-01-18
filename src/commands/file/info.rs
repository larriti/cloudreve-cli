use crate::utils::format_bytes;
use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_info(api: &CloudreveAPI, uri: String, extended: bool) -> Result<()> {
    info!("Getting file info for: {}", uri);
    info!("API Version: {}", api.api_version());

    let file_info = api.get_file_info(&uri).await?;

    info!("");
    info!("📄 File Information:");
    info!("  Name: {}", file_info.name());
    info!("  Path: {}", file_info.path());
    info!(
        "  Type: {}",
        if file_info.is_folder() {
            "Folder"
        } else {
            "File"
        }
    );
    info!("  Size: {}", format_bytes(file_info.size()));
    info!("  Created: {}", file_info.created_at());
    info!("  Updated: {}", file_info.updated_at());

    if extended {
        info!("");
        info!("📋 Extended Info:");
        info!("  Extended information not yet fully implemented");
        info!("  Raw data available via version-specific clients");
    }

    Ok(())
}
