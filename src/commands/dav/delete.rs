use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_delete(api: &CloudreveAPI, id: String) -> Result<()> {
    info!("Deleting WebDAV account {}...", id);

    api.delete_dav_account(&id).await?;

    info!("WebDAV account deleted successfully!");

    Ok(())
}
