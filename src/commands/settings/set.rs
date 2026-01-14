use cloudreve_api::api::v4::models::UserSettings;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_set(
    client: &CloudreveClient,
    key: String,
    value: String,
) -> Result<()> {
    info!("Updating setting: {} = {}", key, value);

    // Get current settings first
    let current_settings = client.get_settings().await?;

    // Update the specific setting
    let new_settings = UserSettings {
        theme: if key == "theme" {
            Some(value.clone())
        } else {
            current_settings.theme
        },
        language: if key == "language" {
            Some(value.clone())
        } else {
            current_settings.language
        },
        timezone: if key == "timezone" {
            Some(value)
        } else {
            current_settings.timezone
        },
    };

    client.update_settings(&new_settings).await?;

    info!("Setting updated successfully");
    Ok(())
}
