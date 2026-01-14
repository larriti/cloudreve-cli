use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_mkdir(client: &CloudreveClient, path: String) -> Result<()> {
    info!("Creating directory: {}", path);

    client.create_directory(&path).await?;

    info!("Directory created successfully");
    Ok(())
}
