use cloudreve_api::api::v4::models::*;
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};

pub async fn handle_share(
    client: &CloudreveClient,
    uri: String,
    _name: Option<String>, // Unused due to API structure
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

    info!("Creating share link for URI: {}", formatted_uri);
    if is_private.unwrap() {
        info!("Password protected share");
    }

    // Build the request with required fields
    let request = CreateShareLinkRequest {
        uri: formatted_uri,
        permissions: PermissionSetting {
            user_explicit: serde_json::json!({}),
            group_explicit: serde_json::json!({}),
            same_group: "read".to_string(),
            other: "none".to_string(),
            anonymous: "none".to_string(),
            everyone: "read".to_string(),
        },
        is_private,
        share_view: Some(true),
        expire,
        price: Some(0),
        password,
        show_readme: Some(true),
    };

    match client.create_share_link(&request).await {
        Ok(share_url) => {
            info!("Share link created successfully!");
            info!("URL: {}", share_url);
        }
        Err(e) => {
            error!("Error creating share link: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
