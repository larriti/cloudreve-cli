use cloudreve_api::api::v4::models::CreateDavAccountRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_create(
    client: &CloudreveClient,
    uri: String,
    name: String,
    readonly: bool,
    proxy: bool,
) -> Result<()> {
    info!("Creating WebDAV account '{}'...", name);

    let request = CreateDavAccountRequest {
        uri,
        name,
        readonly: Some(readonly),
        proxy: Some(proxy),
        disable_sys_files: None,
    };

    let account = client.create_dav_account(&request).await?;

    info!("WebDAV account created successfully!");
    info!("  ID:       {}", account.id);
    info!("  Name:     {}", account.name);
    info!("  URI:      {}", account.uri);
    info!("  Password: {}", account.password);

    Ok(())
}
