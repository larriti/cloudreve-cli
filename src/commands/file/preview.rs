use cloudreve_api::{CloudreveAPI, Result};
use log::{error, info};

pub async fn handle_preview(api: &CloudreveAPI, uri: String, preview_type: String) -> Result<()> {
    info!("Previewing: {} (type: {})", uri, preview_type);

    // For preview, we need to get the download URL first
    match api.download_file(&uri).await {
        Ok(download_url) => match preview_type.as_str() {
            "text" => preview_text(&download_url).await?,
            "json" => preview_json(&download_url).await?,
            "image" => preview_image(&download_url).await?,
            _ => {
                error!("Unsupported preview type: {}", preview_type);
                error!("Supported types: text, json, image");
                return Err(cloudreve_api::Error::Api {
                    code: 400,
                    message: format!("Unsupported preview type: {}", preview_type),
                });
            }
        },
        Err(e) => {
            info!(
                "Preview requires download. Error getting download URL: {}",
                e
            );
            info!("For preview, download the file first:");
            info!(
                "  cloudreve-cli file download --uri {} --output ./preview_file",
                uri
            );
        }
    }

    Ok(())
}

async fn preview_text(url: &str) -> Result<()> {
    let http_client = reqwest::Client::new();
    let response = http_client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(cloudreve_api::Error::Api {
            code: response.status().as_u16() as i32,
            message: "Failed to fetch file content".to_string(),
        });
    }

    let content = response.text().await?;

    // Display content (limit to 100 lines for preview)
    let lines: Vec<&str> = content.lines().take(100).collect();
    for line in lines {
        info!("{}", line);
    }

    if content.lines().count() > 100 {
        info!("... ({} more lines)", content.lines().count() - 100);
    }

    Ok(())
}

async fn preview_json(url: &str) -> Result<()> {
    let http_client = reqwest::Client::new();
    let response = http_client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(cloudreve_api::Error::Api {
            code: response.status().as_u16() as i32,
            message: "Failed to fetch file content".to_string(),
        });
    }

    let content = response.text().await?;

    // Try to format as JSON
    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(value) => {
            info!(
                "{}",
                serde_json::to_string_pretty(&value).unwrap_or_default()
            );
        }
        Err(_) => {
            info!("Content is not valid JSON, displaying as text:");
            info!("{}", content);
        }
    }

    Ok(())
}

async fn preview_image(url: &str) -> Result<()> {
    error!("Image preview requires terminal support (e.g., iTerm2, kitty)");
    error!("Consider downloading the file instead:");
    error!(
        "  cloudreve-cli file download --uri {} --output ./image",
        url
    );
    Err(cloudreve_api::Error::Api {
        code: 400,
        message: "Image preview not supported in this terminal".to_string(),
    })
}
