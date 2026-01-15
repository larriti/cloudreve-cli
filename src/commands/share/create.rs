use cloudreve_api::api::v4::models::{CreateShareLinkRequest, PermissionSetting};
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use serde_json::json;

pub async fn handle_create(
    client: &CloudreveClient,
    uri: String,
    _name: Option<String>,
    expire: Option<u32>,
    password: Option<String>,
) -> Result<()> {
    // Format URI with cloudreve://my/ prefix for v4 API
    let formatted_uri = if uri.starts_with("cloudreve://") {
        uri
    } else {
        let path = uri.strip_prefix('/').unwrap_or(&uri);
        format!("cloudreve://my/{}", path)
    };

    // Validate password format if provided
    if let Some(ref pwd) = password {
        if !pwd.chars().all(|c| c.is_alphanumeric()) {
            error!("Password can only contain letters and numbers (a-zA-Z0-9)");
            return Err(cloudreve_api::Error::InvalidResponse(
                "Password can only contain letters and numbers (a-zA-Z0-9)".to_string(),
            ));
        }
        if pwd.len() > 32 {
            error!("Password must be at most 32 characters");
            return Err(cloudreve_api::Error::InvalidResponse(
                "Password must be at most 32 characters".to_string(),
            ));
        }
    }

    // is_private must be true when password is set
    let is_private = Some(password.is_some());

    info!("Creating share link for: {}", formatted_uri);
    if is_private.unwrap() {
        info!("Password protected share");
    }

    // Create permission setting with proper defaults
    let permission = PermissionSetting {
        user_explicit: json!({}),
        group_explicit: json!({}),
        same_group: "read".to_string(),
        other: "none".to_string(),
        anonymous: "none".to_string(),
        everyone: "read".to_string(),
    };

    let request = CreateShareLinkRequest {
        permissions: permission,
        uri: formatted_uri,
        is_private,
        share_view: Some(true),
        expire,
        price: Some(0),
        password,
        show_readme: Some(true),
    };

    let share_url = client.create_share_link(&request).await?;

    info!("Share link created successfully:");
    info!("  URL: {}", share_url);

    Ok(())
}
