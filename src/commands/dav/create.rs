use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_create(
    api: &CloudreveAPI,
    uri: String,
    name: String,
    readonly: bool,
    proxy: bool,
) -> Result<()> {
    info!("Creating WebDAV account '{}'...", name);

    api.create_dav_account(&uri, &name, readonly, proxy).await?;

    info!("WebDAV account created successfully!");
    info!("  Name:     {}", name);
    info!("  URI:      {}", uri);

    Ok(())
}
