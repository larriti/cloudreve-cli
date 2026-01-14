use cloudreve_api::api::v4::models::{CreateShareLinkRequest, PermissionSetting};
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::json;

pub async fn handle_create(
    client: &CloudreveClient,
    uri: String,
    _name: Option<String>,
    expire: Option<u32>,
    password: Option<String>,
) -> Result<()> {
    info!("Creating share link for: {}", uri);

    // Create a default permission setting
    let permission = PermissionSetting {
        user_explicit: json!({}),
        group_explicit: json!({}),
        same_group: String::new(),
        other: String::new(),
        anonymous: String::new(),
        everyone: String::new(),
    };

    let request = CreateShareLinkRequest {
        permissions: permission,
        uri,
        is_private: Some(password.is_some()),
        share_view: None,
        expire,
        price: None,
        password,
        show_readme: None,
    };

    let share = client.create_share_link(&request).await?;

    info!("Share link created successfully:");
    info!("  ID: {}", share.id);
    info!("  URL: {}", share.url);

    Ok(())
}
