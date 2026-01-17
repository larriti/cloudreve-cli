use cloudreve_api::{CloudreveAPI, Result};
use log::info;
use crate::utils::format_bytes;

pub async fn handle_quota(api: &CloudreveAPI) -> Result<()> {
    info!("Getting user storage quota...");
    let quota = api.get_storage_quota().await?;

    let percent = if quota.total > 0 {
        (quota.used as f64 / quota.total as f64) * 100.0
    } else {
        0.0
    };

    info!("Storage Quota:");
    info!("  Used: {} ({})", format_bytes(quota.used as i64), quota.used);
    info!("  Total: {} ({})", format_bytes(quota.total as i64), quota.total);
    info!("  Free: {} ({})", format_bytes(quota.free as i64), quota.free);
    info!("  Usage: {:.2}%", percent);

    Ok(())
}
