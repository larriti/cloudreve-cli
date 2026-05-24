use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_delete(client: &CloudreveClient, uri: String) -> Result<()> {
    info!("Deleting permissions for: {}", uri);

    client.delete_file_permission(&uri).await?;

    info!("Permissions deleted successfully");
    Ok(())
}
