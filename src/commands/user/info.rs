use cloudreve_api::{CloudreveAPI, Result};
use crate::context::TokenManager;
use log::info;

pub async fn handle_info(api: &CloudreveAPI, _token_manager: &TokenManager) -> Result<()> {
    info!("Getting user information...");

    // For V3, we can get user info directly from CloudreveAPI
    // For V4, we may need user_id from cache
    let user = api.get_user_info().await?;

    info!("User information:");
    info!("  ID: {}", user.id);
    info!("  Email: {}", user.email);
    info!("  Nickname: {}", user.nickname);
    if let Some(status) = &user.status {
        info!("  Status: {}", status);
    }
    if let Some(group) = &user.group {
        info!("  Group: {}", group);
    }

    Ok(())
}
