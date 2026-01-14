use cloudreve_api::{CloudreveClient, Result};
use log::info;
use crate::utils::format_bytes;

pub async fn handle_quota(client: &CloudreveClient) -> Result<()> {
    info!("Getting user storage quota...");
    let quota = client.get_user_capacity().await?;

    let percent = if quota.total > 0 {
        (quota.used as f64 / quota.total as f64) * 100.0
    } else {
        0.0
    };

    info!("Storage Quota:");
    info!("  Used: {} ({})", format_bytes(quota.used as i64), quota.used);
    info!("  Total: {} ({})", format_bytes(quota.total as i64), quota.total);
    info!("  Usage: {:.2}%", percent);

    if let Some(pack_total) = quota.storage_pack_total {
        info!("  Storage Pack Total: {} ({})", format_bytes(pack_total as i64), pack_total);
    }

    Ok(())
}
