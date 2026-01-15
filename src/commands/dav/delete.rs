use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_delete(client: &CloudreveClient, id: String) -> Result<()> {
    info!("Deleting WebDAV account {}...", id);

    client.delete_dav_account(&id).await?;

    info!("WebDAV account deleted successfully!");

    Ok(())
}
