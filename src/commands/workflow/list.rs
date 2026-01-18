use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::Result;
use log::info;

pub async fn handle_list(
    client: &ApiV4Client,
    category: String,
    per_page: String,
) -> Result<()> {
    info!("Listing workflow tasks (category: {})...", category);

    let page_size = per_page.parse().unwrap_or(25);
    let response = client.list_workflow_tasks(page_size, &category).await?;

    if response.tasks.is_empty() {
        info!("No tasks found");
        return Ok(());
    }

    info!("");
    info!("📋 Tasks ({}):", response.tasks.len());
    for task in &response.tasks {
        let status_icon = match task.status {
            cloudreve_api::api::v4::models::TaskStatus::Queued => "⏳",
            cloudreve_api::api::v4::models::TaskStatus::Processing => "🔄",
            cloudreve_api::api::v4::models::TaskStatus::Suspending => "⏸️",
            cloudreve_api::api::v4::models::TaskStatus::Error => "❌",
            cloudreve_api::api::v4::models::TaskStatus::Canceled => "🚫",
            cloudreve_api::api::v4::models::TaskStatus::Completed => "✅",
        };

        // 获取任务类型作为字符串
        let type_str = format!("{:?}", task.r#type);
        let type_str = type_str.to_lowercase();

        info!(
            "  {} {} | {} | {} | {}",
            status_icon, task.id, type_str, task.created_at, format!("{:?}", task.status)
        );
        if let Some(duration) = task.duration {
            info!("     Duration: {}s", duration / 1000);
        }
        if let Some(error) = &task.error {
            info!("     Error: {}", error);
        }
    }
    info!("");
    info!("Total: {} tasks", response.tasks.len());

    Ok(())
}
