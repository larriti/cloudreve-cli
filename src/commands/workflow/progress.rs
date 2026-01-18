use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::Result;
use log::info;

pub async fn handle_progress(client: &ApiV4Client, task_id: String) -> Result<()> {
    info!("Getting progress for task: {}", task_id);

    let progress = client.get_task_progress(&task_id).await?;

    info!("");
    info!("📊 Task Progress:");
    if let Some(identifier) = &progress.identifier {
        info!("  Identifier: {}", identifier);
    }

    match (progress.total, progress.current) {
        (Some(total), Some(current)) => {
            if total > 0 {
                let percentage = (current as f64 / total as f64 * 100.0) as u32;
                info!("  Progress: {} / {} ({}%)", current, total, percentage);
            } else {
                info!("  Progress: {}", current);
            }
        }
        (Some(total), None) => {
            info!("  Total: {} (no progress data)", total);
        }
        (None, Some(current)) => {
            info!("  Current: {} (no total data)", current);
        }
        (None, None) => {
            info!("  No progress data available");
        }
    }

    info!("");
    info!("  Details: {:#?}", progress);

    Ok(())
}
