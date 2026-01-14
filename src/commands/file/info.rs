use cloudreve_api::api::v4::models::GetFileInfoRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::to_string_pretty;
use crate::utils::format_bytes;

pub async fn handle_info(
    client: &CloudreveClient,
    uri: String,
    extended: bool,
) -> Result<()> {
    info!("Getting file info for: {}", uri);

    let request = GetFileInfoRequest {
        uri: &uri,
        include_extended_info: Some(extended),
    };

    let response = client.get_file_info_extended(&request).await?;

    // Display the file information
    info!("File Information:");
    info!("  Type: {:?}", response.r#type);
    info!("  ID: {}", response.id);
    info!("  Name: {}", response.name);
    if let Some(permission) = &response.permission {
        info!("  Permission: {}", permission);
    }
    info!("  Created: {}", response.created_at);
    info!("  Updated: {}", response.updated_at);
    info!("  Size: {} ({})", format_bytes(response.size), response.size);
    info!("  Path: {}", response.path);

    if extended {
        info!("\nMetadata:");
        info!("  {}", to_string_pretty(&response.metadata).unwrap_or_default());
    }

    Ok(())
}
