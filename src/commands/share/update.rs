use cloudreve_api::api::v4::models::{EditShareLinkRequest, PermissionSetting};
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::json;

pub async fn handle_update(
    client: &CloudreveClient,
    id: String,
    _name: Option<String>,
    expire: Option<u32>,
    _password: Option<String>,
) -> Result<()> {
    info!("Updating share link: {}", id);

    // Create a default permission setting
    let permission = PermissionSetting {
        user_explicit: json!({}),
        group_explicit: json!({}),
        same_group: String::new(),
        other: String::new(),
        anonymous: String::new(),
        everyone: String::new(),
    };

    let request = EditShareLinkRequest {
        permissions: permission,
        uri: String::new(), // Empty URI means not changing
        share_view: None,
        show_readme: None,
        expire,
        price: None,
    };

    client.edit_share_link(&id, &request).await?;

    info!("Share link updated successfully");
    Ok(())
}
