use cloudreve_api::Result;
use cloudreve_api::api::v4::ApiV4Client;
use log::info;

pub async fn handle_cancel(client: &ApiV4Client, task_id: String) -> Result<()> {
    info!("Canceling task: {}", task_id);
    info!("Note: This only works for download tasks");

    client.cancel_download_task(&task_id).await?;

    info!("");
    info!("✅ Task {} canceled successfully", task_id);

    Ok(())
}
