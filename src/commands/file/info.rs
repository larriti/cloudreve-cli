use cloudreve_api::api::v4::models::GetFileInfoRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::to_string_pretty;

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
    println!("File Information:");
    println!("  Type: {:?}", response.r#type);
    println!("  ID: {}", response.id);
    println!("  Name: {}", response.name);
    if let Some(permission) = &response.permission {
        println!("  Permission: {}", permission);
    }
    println!("  Created: {}", response.created_at);
    println!("  Updated: {}", response.updated_at);
    println!("  Size: {} bytes", response.size);
    println!("  Path: {}", response.path);

    if extended {
        println!("\nMetadata:");
        println!("  {}", to_string_pretty(&response.metadata).unwrap_or_default());
    }

    Ok(())
}
