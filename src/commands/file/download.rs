use cloudreve_api::{CloudreveAPI, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use reqwest::Client;
use std::fs::File;
use std::io::Write;

pub async fn handle_download(
    api: &CloudreveAPI,
    uri: String,
    output: String,
    _expires_in: Option<u32>,
) -> Result<()> {
    info!("Downloading file with URI: {} to {}", uri, output);

    // 1. Create download URL using CloudreveAPI (handles V3/V4 differences internally)
    let download_url = api.download_file(&uri).await?;
    info!("Download URL created: {}", download_url);

    // 2. Determine output filename
    let file_name = if uri.starts_with("cloudreve://") {
        uri.strip_prefix("cloudreve://")
            .and_then(|p| p.split('/').next_back())
            .unwrap_or("downloaded_file")
    } else {
        uri.split('/').next_back().unwrap_or("downloaded_file")
    };

    let output_path = if output.ends_with('/') || output == "." || output == "./" {
        format!("{}/{}", output.trim_end_matches('/'), file_name)
    } else {
        output
    };

    info!("Saving to: {}", output_path);

    // 3. Download file
    let http_client = Client::new();
    let response = http_client.get(&download_url).send().await?;

    if !response.status().is_success() {
        error!("Download failed with status: {}", response.status());
        return Err(cloudreve_api::Error::Api {
            code: response.status().as_u16() as i32,
            message: "Download failed".to_string(),
        });
    }

    let total_size = response.content_length().unwrap_or(0);

    // Create progress bar for download
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message("Downloading");

    // 4. Download and save to local file
    let bytes = response.bytes().await?;
    let mut file = File::create(&output_path)?;
    file.write_all(&bytes)?;

    pb.finish_with_message("Download completed!");

    info!("Saved to: {}", output_path);
    info!("Size: {}", format_bytes(bytes.len() as i64));

    Ok(())
}

fn format_bytes(bytes: i64) -> String {
    const TB: i64 = 1024 * 1024 * 1024 * 1024;
    const GB: i64 = 1024 * 1024 * 1024;
    const MB: i64 = 1024 * 1024;
    const KB: i64 = 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
