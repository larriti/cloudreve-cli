use cloudreve_api::{CloudreveClient, Result};
use log::info;

fn format_bytes(bytes: u64) -> String {
    const TB: u64 = 1024 * 1024 * 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub async fn handle_quota(client: &CloudreveClient) -> Result<()> {
    info!("Getting user storage quota...");
    let quota = client.get_user_capacity().await?;

    let percent = if quota.total > 0 {
        (quota.used as f64 / quota.total as f64) * 100.0
    } else {
        0.0
    };

    info!("Storage Quota:");
    info!("  Used: {} ({})", format_bytes(quota.used), quota.used);
    info!("  Total: {} ({})", format_bytes(quota.total), quota.total);
    info!("  Usage: {:.2}%", percent);

    if let Some(pack_total) = quota.storage_pack_total {
        info!("  Storage Pack Total: {} ({})", format_bytes(pack_total), pack_total);
    }

    Ok(())
}
