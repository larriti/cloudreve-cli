use cloudreve_api::Result;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::api::v4::models::ExtractArchiveRequest;
use cloudreve_api::api::v4::uri::path_to_uri;
use log::info;

pub async fn handle_extract(
    client: &ApiV4Client,
    archive: String,
    path: Option<String>,
) -> Result<()> {
    info!("Extracting archive: {}", archive);

    // 使用 path_to_uri 转换路径
    let dst = path.as_deref().unwrap_or("/");
    let src_uri = path_to_uri(&archive);
    let dst_uri = path_to_uri(dst);

    let request = ExtractArchiveRequest {
        src: vec![&src_uri],
        dst: &dst_uri,
    };

    let task = client.extract_archive(&request).await?;

    info!("");
    info!("✅ Extract task created successfully");
    info!("  Task ID: {}", task.id);
    info!("  Status: {:?}", task.status);
    info!("  Type: {:?}", task.r#type);

    Ok(())
}
