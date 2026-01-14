use cloudreve_api::api::v4::models::RenameFileRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_rename(
    client: &CloudreveClient,
    src: String,
    name: String,
) -> Result<()> {
    info!("Renaming: {} -> {}", src, name);

    let request = RenameFileRequest { name: &name };
    client.rename_file(&src, &request).await?;

    info!("Rename completed successfully");
    Ok(())
}
