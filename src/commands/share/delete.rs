use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_delete(client: &CloudreveClient, id: String) -> Result<()> {
    info!("Deleting share link: {}", id);

    client.delete_share_link(&id).await?;

    info!("Share link deleted successfully");
    Ok(())
}
