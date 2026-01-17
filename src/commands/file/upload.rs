use cloudreve_api::{CloudreveAPI, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn handle_upload(
    api: &CloudreveAPI,
    file: String,
    path: String,
    _overwrite: bool,
    policy_id: Option<String>,
) -> Result<()> {
    info!("Uploading file: {} to path: {}", file, path);

    // 1. Verify local file
    let file_path = Path::new(&file);
    if !file_path.exists() {
        error!("Error: File '{}' does not exist", file);
        return Err(cloudreve_api::Error::Api {
            code: 404,
            message: format!("File '{}' does not exist", file),
        });
    }

    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let file_size = file_path.metadata()?.len();
    info!("File size: {} bytes", file_size);

    // 2. Build full upload path
    let upload_path = if path.ends_with('/') || path.is_empty() {
        format!("{}{}", path, file_name)
    } else {
        format!("{}/{}", path, file_name)
    };

    info!("Upload path: {}", upload_path);

    // 3. Read file content
    let mut file_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut file_content)?;

    // 4. Create progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("=>-"));
    pb.set_message("Uploading");

    // 5. Upload using CloudreveAPI (handles V3/V4 differences internally)
    api.upload_file(&upload_path, file_content, policy_id.as_deref()).await?;

    pb.finish_with_message("Upload completed!");
    info!("File uploaded successfully!");

    Ok(())
}
