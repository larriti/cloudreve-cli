use cloudreve_api::api::v4::models::ImportRequest;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::Result;
use log::info;

pub async fn handle_import(
    client: &ApiV4Client,
    src: String,
    dst: String,
    user_id: Option<String>,
    policy_id: Option<i32>,
    extract_media_meta: bool,
    recursive: bool,
) -> Result<()> {
    // Normalize src path to absolute if not already
    let src = if src.starts_with('/') {
        src
    } else {
        // Get current working directory and append relative path
        let current_dir = std::env::current_dir()
            .map_err(|e| cloudreve_api::Error::InvalidResponse(format!("Failed to get current directory: {}", e)))?;
        let current_dir_str = current_dir.to_string_lossy().to_string();
        format!("{}/{}", current_dir_str, src)
    };

    info!("Importing from external storage: {} -> {}", src, dst);

    // User ID is required (get from token if not specified)
    let user_id = user_id.ok_or_else(|| {
        cloudreve_api::Error::InvalidResponse(
            "Could not determine user_id. Please specify --user-id or ensure you're logged in.".to_string()
        )
    })?;
    info!("Using user ID: {}", user_id);

    // Get default storage policy from target directory if not specified
    let policy_id = if let Some(pid) = policy_id {
        pid
    } else {
        return Err(cloudreve_api::Error::InvalidResponse(
            "Target directory has no storage policy. Please specify --policy-id.".to_string()));
    };

    let request = ImportRequest {
        src: &src,
        dst: &dst,
        user_id: &user_id,
        policy_id,
        extract_media_meta: if extract_media_meta { Some(true) } else { None },
        recursive: if recursive { Some(true) } else { None },
    };

    let task = client.import(&request).await?;

    info!("");
    info!("✅ Import task created successfully");
    info!("  Task ID: {}", task.id);
    info!("  Status: {:?}", task.status);
    info!("  Type: {:?}", task.r#type);
    info!("  User ID: {}", user_id);
    info!("  Policy ID: {}", policy_id);

    Ok(())
}
