use cloudreve_api::api::v4::models::CreateDownloadUrlRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use reqwest::Client;

pub async fn handle_preview(
    client: &CloudreveClient,
    uri: String,
    preview_type: String,
) -> Result<()> {
    info!("Previewing: {} (type: {})", uri, preview_type);

    // Create download URL (API layer handles URI conversion)
    let request = CreateDownloadUrlRequest {
        uris: vec![&uri],
        download: Some(true),
        redirect: None,
        entity: None,
        use_primary_site_url: None,
        skip_error: None,
        archive: None,
        no_cache: None,
    };

    let download_response = client.create_download_url(&request).await?;

    if download_response.urls.is_empty() {
        error!("No download URLs returned from API");
        return Err(cloudreve_api::Error::Api {
            code: 400,
            message: "No download URLs available".to_string(),
        });
    }

    let download_url = &download_response.urls[0].url;

    match preview_type.as_str() {
        "text" => preview_text(download_url).await?,
        "json" => preview_json(download_url).await?,
        "image" => preview_image(download_url).await?,
        _ => {
            error!("Unsupported preview type: {}", preview_type);
            error!("Supported types: text, json, image");
            return Err(cloudreve_api::Error::Api {
                code: 400,
                message: format!("Unsupported preview type: {}", preview_type),
            });
        }
    }

    Ok(())
}

async fn preview_text(url: &str) -> Result<()> {
    let http_client = Client::new();
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
    let http_client = Client::new();
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
            info!("{}", serde_json::to_string_pretty(&value).unwrap_or_default());
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
    error!("  cloudreve-cli file download --uri {} --output ./image", url);
    Err(cloudreve_api::Error::Api {
        code: 400,
        message: "Image preview not supported in this terminal".to_string(),
    })
}
