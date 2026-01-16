use cloudreve_api::{CloudreveAPI, Result};
use log::info;

pub async fn handle_rename(
    api: &CloudreveAPI,
    src: String,
    name: String,
) -> Result<()> {
    info!("Renaming: {} -> {}", src, name);

    api.rename(&src, &name).await?;

    info!("Rename completed successfully");
    Ok(())
}
