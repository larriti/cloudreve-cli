use cloudreve_api::{CloudreveClient, Result};
use log::info;
use serde_json::to_string_pretty;

pub async fn handle_get(
    client: &CloudreveClient,
    key: Option<String>,
) -> Result<()> {
    info!("Getting user settings...");

    let settings = client.get_settings().await?;

    if let Some(k) = key {
        match k.as_str() {
            "theme" => {
                if let Some(theme) = settings.theme {
                    info!("Setting '{}': {}", k, theme);
                }
            }
            "language" => {
                if let Some(language) = settings.language {
                    info!("Setting '{}': {}", k, language);
                }
            }
            "timezone" => {
                if let Some(timezone) = settings.timezone {
                    info!("Setting '{}': {}", k, timezone);
                }
            }
            _ => {
                info!("Setting '{}' not found", k);
            }
        }
    } else {
        info!("All settings:");
        info!("{}", to_string_pretty(&settings).unwrap_or_default());
    }

    Ok(())
}
