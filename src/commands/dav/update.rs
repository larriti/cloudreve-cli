use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_update(
    api: &CloudreveAPI,
    id: String,
    uri: Option<String>,
    name: Option<String>,
    readonly: Option<bool>,
    proxy: Option<bool>,
) -> Result<()> {
    info!("Updating WebDAV account {}...", id);

    api.update_dav_account(
        &id,
        uri.as_deref(),
        name.as_deref(),
        readonly,
        proxy,
    ).await?;

    info!("WebDAV account updated successfully!");
    info!("  ID:       {}", id);

    Ok(())
}
