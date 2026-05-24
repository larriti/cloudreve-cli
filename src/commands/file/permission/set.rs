use cloudreve_api::api::v4::models::SetFilePermissionRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::Value;

/// Permission options for setting file permissions
#[derive(Debug, Default)]
pub struct PermissionOptions {
    pub user_explicit: Option<String>,
    pub group_explicit: Option<String>,
    pub same_group: Option<String>,
    pub other: Option<String>,
    pub anonymous: Option<String>,
    pub everyone: Option<String>,
}

pub async fn handle_set(
    client: &CloudreveClient,
    uri: String,
    options: PermissionOptions,
) -> Result<()> {
    info!("Setting permissions for: {}", uri);

    // Parse JSON strings to Value, use empty object as default
    let user_explicit_value = options
        .user_explicit
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    let group_explicit_value = options
        .group_explicit
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .unwrap_or_else(|| serde_json::json!({}));

    // Use empty string as default for other fields
    let same_group_str = options.same_group.unwrap_or_default();
    let other_str = options.other.unwrap_or_default();
    let anonymous_str = options.anonymous.unwrap_or_default();
    let everyone_str = options.everyone.unwrap_or_default();

    let request = SetFilePermissionRequest {
        uri: &uri,
        user_explicit: Some(user_explicit_value),
        group_explicit: Some(group_explicit_value),
        same_group: Some(same_group_str.as_str()),
        other: Some(other_str.as_str()),
        anonymous: Some(anonymous_str.as_str()),
        everyone: Some(everyone_str.as_str()),
    };

    client.set_file_permission(&request).await?;

    info!("Permissions set successfully");
    Ok(())
}
