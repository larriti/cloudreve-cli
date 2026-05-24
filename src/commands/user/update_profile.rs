use cloudreve_api::api::v4::models::UpdateProfileRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_update_profile(
    client: &CloudreveClient,
    nickname: Option<String>,
    avatar: Option<String>,
) -> Result<()> {
    info!("Updating user profile...");

    let request = UpdateProfileRequest {
        nickname: nickname.as_deref(),
        email: None,
        avatar: avatar.as_deref(),
    };

    client.update_profile(&request).await?;

    info!("Profile updated successfully");
    Ok(())
}
