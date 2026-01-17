use cloudreve_api::{CloudreveAPI, Result};
use cloudreve_api::api::v4::uri::uri_to_path;
use log::info;

pub async fn handle_list(api: &CloudreveAPI, page_size: u32) -> Result<()> {
    info!("Listing WebDAV accounts...");

    let response = api.list_dav_accounts(page_size).await?;

    if response.accounts.is_empty() {
        info!("No WebDAV accounts found.");
        return Ok(());
    }

    info!("Found {} WebDAV account(s):", response.accounts.len());
    for account in response.accounts {
        info!("");
        info!("  ID:          {}", account.id);
        info!("  Name:        {}", account.name);
        if let Some(uri) = &account.uri {
            if let Ok(path) = uri_to_path(uri) {
                info!("  Path:        {}", path);
            }
            else {
                info!("  Path:        {}", uri);
            }
        }
        if let Some(server) = &account.server {
            info!("  Server:      {}", server);
        }
        if let Some(password) = &account.password {
            info!("  Password:    {}", password);
        }
        info!("  Created:     {}", account.created_at);
    }

    Ok(())
}
