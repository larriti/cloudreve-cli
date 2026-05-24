use cloudreve_api::api::v4::models::UpdateMetadataRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::Value;

pub async fn handle_update(
    client: &CloudreveClient,
    uri: String,
    metadata: String,
    clear: bool,
) -> Result<()> {
    info!("Updating metadata for: {}", uri);

    let metadata_value: Value = serde_json::from_str(&metadata)?;

    let request = UpdateMetadataRequest {
        metadata: Some(metadata_value),
        clear_metadata: Some(clear),
    };

    client.patch_metadata(&uri, &request).await?;

    info!("Metadata updated successfully");
    Ok(())
}
