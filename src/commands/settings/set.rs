use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct UpdateSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_retention_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_retention_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_view_sync: Option<bool>,
}

pub async fn handle_set(client: &CloudreveClient, key: String, value: String) -> Result<()> {
    info!("Updating setting: {} = {}", key, value);

    let mut request = UpdateSettingsRequest {
        nick: None,
        language: None,
        preferred_theme: None,
        version_retention_enabled: None,
        version_retention_max: None,
        disable_view_sync: None,
    };

    match key.as_str() {
        "nick" | "nickname" => {
            request.nick = Some(value);
        }
        "language" => {
            request.language = Some(value);
        }
        "theme" | "preferred_theme" => {
            request.preferred_theme = Some(value);
        }
        "version_retention_enabled" => {
            let bool_val = value.parse::<bool>().map_err(|_| {
                cloudreve_api::Error::InvalidResponse("Invalid boolean value".to_string())
            })?;
            request.version_retention_enabled = Some(bool_val);
        }
        "version_retention_max" => {
            let int_val = value.parse::<i64>().map_err(|_| {
                cloudreve_api::Error::InvalidResponse("Invalid integer value".to_string())
            })?;
            request.version_retention_max = Some(int_val);
        }
        "disable_view_sync" => {
            let bool_val = value.parse::<bool>().map_err(|_| {
                cloudreve_api::Error::InvalidResponse("Invalid boolean value".to_string())
            })?;
            request.disable_view_sync = Some(bool_val);
        }
        _ => {
            return Err(cloudreve_api::Error::InvalidResponse(format!(
                "Unknown setting key: {}. Available keys: nick, language, theme, version_retention_enabled, version_retention_max, disable_view_sync",
                key
            )));
        }
    }

    // Use the patch method from ApiV4Client
    let _response: serde_json::Value = client.patch("/user/setting", &request).await?;

    info!("Setting updated successfully");
    Ok(())
}
