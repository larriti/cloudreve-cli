use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_change_password(
    client: &CloudreveClient,
    old_password: String,
    new_password: String,
) -> Result<()> {
    info!("Changing password...");

    client.change_password(&old_password, &new_password).await?;

    info!("Password changed successfully");
    Ok(())
}
