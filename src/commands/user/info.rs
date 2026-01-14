use cloudreve_api::{CloudreveClient, Result};
use crate::context::TokenManager;
use log::{debug, info};

pub async fn handle_info(client: &CloudreveClient, token_manager: &TokenManager) -> Result<()> {
    info!("Getting user information...");

    // Get user_id from cache
    let token_info = token_manager.get_default_token()?
        .ok_or_else(|| cloudreve_api::Error::Api {
            code: 401,
            message: "No cached token found. Please authenticate first.".to_string(),
        })?;

    let user_id = token_info.user_id;
    debug!("Using user_id from cache: {}", user_id);

    // Use correct API endpoint: /user/info/{user_id}
    let user = client.get_user_info(&user_id).await?;

    info!("User information:");
    info!("  ID: {}", user.id);
    info!("  Email: {}", user.email);
    info!("  Nickname: {}", user.nickname);
    if let Some(status) = &user.status {
        info!("  Status: {}", status);
    }
    if let Some(avatar) = &user.avatar {
        info!("  Avatar: {}", avatar);
    }
    info!("  Created At: {}", user.created_at);
    if let Some(group) = &user.group {
        info!("  Group: {} ({})", group.name, group.id);
    }

    Ok(())
}
