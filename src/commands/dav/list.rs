use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_list(client: &CloudreveClient, page_size: u32) -> Result<()> {
    info!("Listing WebDAV accounts...");

    let response = client.list_dav_accounts(page_size, None).await?;

    if response.accounts.is_empty() {
        info!("No WebDAV accounts found.");
        return Ok(());
    }

    info!("Found {} WebDAV account(s):", response.accounts.len());
    for account in response.accounts {
        info!("");
        info!("  ID:          {}", account.id);
        info!("  Name:        {}", account.name);
        info!("  URI:         {}", account.uri);
        info!("  Password:    {}", account.password);
        info!("  Created:     {}", account.created_at);
    }

    Ok(())
}
