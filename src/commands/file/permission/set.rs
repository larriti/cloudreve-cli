use cloudreve_api::api::v4::models::SetFilePermissionRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::Value;

pub async fn handle_set(
    client: &CloudreveClient,
    uri: String,
    user_explicit: Option<String>,
    group_explicit: Option<String>,
    same_group: Option<String>,
    other: Option<String>,
    anonymous: Option<String>,
    everyone: Option<String>,
) -> Result<()> {
    info!("Setting permissions for: {}", uri);

    // Parse JSON strings to Value, use empty object as default
    let user_explicit_value = user_explicit
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    let group_explicit_value = group_explicit
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .unwrap_or_else(|| serde_json::json!({}));

    // Use empty string as default for other fields
    let same_group_str = same_group.unwrap_or_default();
    let other_str = other.unwrap_or_default();
    let anonymous_str = anonymous.unwrap_or_default();
    let everyone_str = everyone.unwrap_or_default();

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
