use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_mkdir(api: &CloudreveAPI, path: String) -> Result<()> {
    info!("Creating directory: {}", path);

    api.create_directory(&path).await?;

    info!("Directory created successfully");
    Ok(())
}
