use cloudreve_api::api::v4::models::CreateDavAccountRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_update(
    client: &CloudreveClient,
    id: String,
    uri: Option<String>,
    name: Option<String>,
    readonly: Option<bool>,
    proxy: Option<bool>,
) -> Result<()> {
    info!("Updating WebDAV account {}...", id);

    // Get current account info first to fill in missing fields
    let list_response = client.list_dav_accounts(100, None).await?;
    let current_account = list_response
        .accounts
        .iter()
        .find(|a| a.id == id)
        .ok_or_else(|| cloudreve_api::Error::Api {
            code: 404,
            message: format!("WebDAV account '{}' not found", id),
        })?;

    let request = CreateDavAccountRequest {
        uri: uri.unwrap_or_else(|| {
            // Extract path from URI (remove cloudreve:// prefix)
            current_account
                .uri
                .strip_prefix("cloudreve://")
                .unwrap_or(&current_account.uri)
                .to_string()
        }),
        name: name.unwrap_or_else(|| current_account.name.clone()),
        readonly,
        proxy,
        disable_sys_files: None,
    };

    let account = client.update_dav_account(&id, &request).await?;

    info!("WebDAV account updated successfully!");
    info!("  ID:       {}", account.id);
    info!("  Name:     {}", account.name);
    info!("  URI:      {}", account.uri);

    Ok(())
}
