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
    info!("Creating share link for URI: {}", uri);

    // Build the request with required fields
    let request = CreateShareLinkRequest {
        uri,
        permissions: PermissionSetting {
            user_explicit: serde_json::json!({}),
            group_explicit: serde_json::json!({}),
            same_group: "read".to_string(),
            other: "none".to_string(),
            anonymous: "none".to_string(),
            everyone: "read".to_string(),
        },
        is_private: Some(false),
        share_view: Some(true),
        expire,
        price: Some(0),
        password,
        show_readme: Some(true),
    };

    match client.create_share_link(&request).await {
        Ok(share_link) => {
            info!("Share link created successfully!");
            info!("ID: {}", share_link.id);
            info!("Name: {}", share_link.name);
            info!("Created at: {}", share_link.created_at);
            if let Some(expired_at) = share_link.expired_at {
                info!("Expires at: {}", expired_at);
            }
        }
        Err(e) => {
            error!("Error creating share link: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
