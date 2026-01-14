use cloudreve_api::api::v4::models::CreateDownloadUrlRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use reqwest::Client;
use std::fs::File;
use std::io::Write;

pub async fn handle_download(
    client: &CloudreveClient,
    uri: String,
    output: String,
    _expires_in: Option<u32>,
) -> Result<()> {
    info!("Downloading file with URI: {} to {}", uri, output);

    // Convert to cloudreve://my/ format if needed
    let full_uri = if uri.starts_with("cloudreve://") {
        &uri
    } else if uri.starts_with('/') {
        &format!("cloudreve://my{}", uri)
    } else {
        &format!("cloudreve://my/{}", uri)
    };

    info!("Using full URI: {}", full_uri);

    // 1. Create download URL
    let request = CreateDownloadUrlRequest {
        uris: vec![full_uri],
        download: Some(true),
        redirect: None,
        entity: None,
        use_primary_site_url: None,
        skip_error: None,
        archive: None,
        no_cache: None,
    };

    let download_response = client.create_download_url(&request).await?;

    // Get the first URL from the response
    if download_response.urls.is_empty() {
        error!("No download URLs returned from API");
        return Err(cloudreve_api::Error::Api {
            code: 400,
            message: "No download URLs available".to_string(),
        });
    }

    let url_item = &download_response.urls[0];
    let download_url = &url_item.url;
    info!("Download URL created: {}", download_url);

    // Determine output filename
    let output_path = if output.ends_with('/') || output == "." || output == "./" {
        // Extract filename from URI or use display name from response
        let filename = if let Some(ref display_name) = url_item.stream_saver_display_name {
            display_name
        } else {
            // Extract from URI: cloudreve://my/packages/rust/cloudreve-cli
            uri.split('/')
                .next_back()
                .unwrap_or("downloaded_file")
        };
        format!("{}/{}", output.trim_end_matches('/'), filename)
    } else {
        output
    };

    info!("Saving to: {}", output_path);

    // 2. Download file
    let http_client = Client::new();
    let response = http_client.get(download_url).send().await?;

    if !response.status().is_success() {
        error!("Download failed with status: {}", response.status());
        return Err(cloudreve_api::Error::Api {
            code: response.status().as_u16() as i32,
            message: "Download failed".to_string(),
        });
    }

    let total_size = response.content_length().unwrap_or(0);
    info!("Downloading {} bytes...", total_size);

    // 3. Save to local file
    let bytes = response.bytes().await?;
    let mut file = File::create(&output_path)?;
    file.write_all(&bytes)?;

    info!("Download completed successfully!");
    info!("Saved to: {}", output_path);
    info!("Size: {} bytes", bytes.len());

    Ok(())
}
