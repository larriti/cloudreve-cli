use cloudreve_api::api::v4::models::CreateArchiveRequest;
use cloudreve_api::api::v4::uri::path_to_uri;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::Result;
use log::info;

pub async fn handle_archive(
    client: &ApiV4Client,
    files: String,
    name: String,
    path: Option<String>,
) -> Result<()> {
    info!("Creating archive '{}' from files: {}", name, files);

    // 将文件列表转换为 URI
    let files_vec: Vec<String> = files
        .split(',')
        .map(|s| path_to_uri(s.trim()))
        .collect();

    // 构建目标 URI (name 应该是完整路径，包含文件名)
    let dir = path.as_deref().unwrap_or("/");
    let dst = path_to_uri(&format!("{}/{}", dir.trim_end_matches('/'), name));

    let request = CreateArchiveRequest {
        src: files_vec.iter().map(|s| s.as_str()).collect(),
        dst: &dst,
    };

    let task = client.create_archive(&request).await?;

    info!("");
    info!("✅ Archive task created successfully");
    info!("  Task ID: {}", task.id);
    info!("  Status: {:?}", task.status);
    info!("  Type: {:?}", task.r#type);

    Ok(())
}
