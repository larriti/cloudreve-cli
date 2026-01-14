use cloudreve_api::api::v4::models::MoveCopyFileRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_move(
    client: &CloudreveClient,
    src: Vec<String>,
    dest: String,
) -> Result<()> {
    info!("Moving {} file(s) to {}", src.len(), dest);

    let src_refs: Vec<&str> = src.iter().map(|s| s.as_str()).collect();
    let request = MoveCopyFileRequest {
        from: src_refs,
        to: &dest,
        copy: Some(false),
    };

    client.move_copy_files(&request).await?;

    info!("Move completed successfully");
    Ok(())
}
