use cloudreve_api::{CloudreveAPI, Result, UnifiedClient, UserInfo};
use log::info;

pub async fn handle_info(api: &CloudreveAPI, token_manager: &crate::context::TokenManager) -> Result<()> {
    info!("Getting user information...");

    let user = match api.inner() {
        UnifiedClient::V3(_) => {
            // V3: Get user info from API
            api.get_user_info().await?
        }
        UnifiedClient::V4(client) => {
            // V4: Get user_id from token and call user info endpoint
            let token_info = token_manager
                .get_token_by_url(api.base_url())?
                .ok_or_else(|| cloudreve_api::Error::InvalidResponse(
                    "Token expired or invalid.".to_string()
                ))?;

            let v4_user = client.get_user_info(&token_info.user_id).await?;
            UserInfo {
                id: v4_user.id,
                email: v4_user.email,
                nickname: v4_user.nickname,
                group: v4_user.group.map(|g| g.name),
                status: v4_user.status,
            }
        }
    };

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
