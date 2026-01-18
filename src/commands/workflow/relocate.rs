use cloudreve_api::Result;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::api::v4::models::RelocateRequest;
use cloudreve_api::api::v4::uri::path_to_uri;
use log::info;

pub async fn handle_relocate(
    client: &ApiV4Client,
    files: String,
    policy: String,
    _path: Option<String>,
) -> Result<()> {
    info!("Relocating files to policy '{}': {}", policy, files);

    // 将文件列表转换为 URI
    let files_vec: Vec<String> = files.split(',').map(|s| path_to_uri(s.trim())).collect();

    let request = RelocateRequest {
        src: files_vec.iter().map(|s| s.as_str()).collect(),
        dst_policy_id: &policy,
    };

    let task = client.relocate(&request).await?;

    info!("");
    info!("✅ Relocate task created successfully");
    info!("  Task ID: {}", task.id);
    info!("  Status: {:?}", task.status);
    info!("  Type: {:?}", task.r#type);

    Ok(())
}
