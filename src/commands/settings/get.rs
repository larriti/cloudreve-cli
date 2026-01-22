use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_get(client: &CloudreveClient, key: Option<String>) -> Result<()> {
    info!("Getting user settings...");

    let settings = client.get_settings().await?;

    if let Some(k) = key {
        match k.as_str() {
            "credit" => {
                info!("Credit: {}", settings.credit);
            }
            "passwordless" => {
                info!("Passwordless: {}", settings.passwordless);
            }
            "two_fa_enabled" => {
                info!("2FA Enabled: {}", settings.two_fa_enabled);
            }
            "version_retention" => {
                info!("Version Retention: {}", settings.version_retention_enabled);
                if let Some(max) = settings.version_retention_max {
                    info!("  Max versions: {}", max);
                }
                if let Some(exts) = &settings.version_retention_ext {
                    info!("  Extensions: {:?}", exts);
                }
            }
            "storage_packs" => {
                info!("Storage Packs:");
                for pack in &settings.storage_packs {
                    info!("  - {}", pack.name);
                    info!("    Size: {} GB", pack.size / 1024 / 1024 / 1024);
                    info!("    Expires: {}", pack.expire_at);
                }
            }
            "passkeys" => {
                if let Some(keys) = &settings.passkeys {
                    info!("Passkeys ({}):", keys.len());
                    for key in keys {
                        info!("  - {}", key.name);
                    }
                } else {
                    info!("No passkeys registered");
                }
            }
            "login_activity" => {
                if let Some(activities) = &settings.login_activity {
                    info!("Recent Login Activity ({}):", activities.len());
                    for activity in activities.iter().take(10) {
                        info!(
                            "  - {} {} from {} ({})",
                            activity.created_at,
                            activity.browser,
                            activity.ip,
                            if activity.success { "success" } else { "failed" }
                        );
                    }
                } else {
                    info!("No login activity available");
                }
            }
            _ => {
                info!("Setting '{}' not found. Available keys:", k);
                info!("  credit, passwordless, two_fa_enabled, version_retention,");
                info!("  storage_packs, passkeys, login_activity");
            }
        }
    } else {
        info!("User Settings:");
        info!("  Credit: {}", settings.credit);
        info!("  Passwordless: {}", settings.passwordless);
        info!("  2FA Enabled: {}", settings.two_fa_enabled);
        info!("  Version Retention: {}", settings.version_retention_enabled);
        info!("  Disable View Sync: {}", settings.disable_view_sync);

        if let Some(expires) = &settings.group_expires {
            info!("  Group Expires: {}", expires);
        }

        if !settings.storage_packs.is_empty() {
            info!("  Storage Packs:");
            for pack in &settings.storage_packs {
                info!("    - {}", pack.name);
            }
        }

        if let Some(keys) = &settings.passkeys {
            info!("  Passkeys: {}", keys.len());
        }
    }

    Ok(())
}
