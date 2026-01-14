use cloudreve_api::api::v4::models::{FileType, ListFilesRequest};
use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use crate::utils::format_bytes;

/// Search filter options
#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub name_pattern: Option<String>,
    pub file_type: Option<FileType>,
    pub min_size: Option<i64>,
    pub max_size: Option<i64>,
    pub extension: Option<String>,
}

/// Handle file search
pub async fn handle_search(
    client: &CloudreveClient,
    path: String,
    filter: SearchFilter,
    recursive: bool,
) -> Result<()> {
    info!("Searching in path: {}", path);
    if let Some(ref pattern) = filter.name_pattern {
        info!("Name pattern: {}", pattern);
    }
    if let Some(ref ext) = filter.extension {
        info!("Extension: {}", ext);
    }
    if let Some(min) = filter.min_size {
        info!("Min size: {}", format_bytes(min));
    }
    if let Some(max) = filter.max_size {
        info!("Max size: {}", format_bytes(max));
    }
    if let Some(ref ft) = filter.file_type {
        info!("Type: {:?}", ft);
    }
    info!("Recursive: {}", recursive);

    let mut results = Vec::new();
    search_recursive(client, &path, &filter, recursive, &mut results).await?;

    if results.is_empty() {
        info!("No matching files found");
        return Ok(());
    }

    info!("");
    info!("Found {} matching items:", results.len());
    for file in &results {
        match file.r#type {
            FileType::Folder => info!("  📁 {}/", file.name),
            FileType::File => info!("  📄 {} ({})", file.name, format_bytes(file.size)),
        }
    }

    Ok(())
}

async fn search_recursive(
    client: &CloudreveClient,
    path: &str,
    filter: &SearchFilter,
    recursive: bool,
    results: &mut Vec<cloudreve_api::api::v4::models::File>,
) -> Result<()> {
    let request = ListFilesRequest {
        path,
        page: Some(0),
        page_size: Some(100),
        ..Default::default()
    };

    match client.list_files(&request).await {
        Ok(response) => {
            for file in response.files {
                if matches_filter(&file, filter) {
                    results.push(file.clone());
                }

                if recursive && file.r#type == FileType::Folder {
                    let sub_path = format!("{}/{}/", path.trim_end_matches('/'), file.name);
                    // Use Box::pin for recursive async call
                    let search_future = Box::pin(search_recursive(
                        client,
                        &sub_path,
                        filter,
                        recursive,
                        results,
                    ));
                    let _ = search_future.await;
                }
            }
        }
        Err(e) => {
            error!("Error searching in {}: {}", path, e);
        }
    }

    Ok(())
}

fn matches_filter(file: &cloudreve_api::api::v4::models::File, filter: &SearchFilter) -> bool {
    // Name pattern filter (case-insensitive substring)
    if let Some(pattern) = &filter.name_pattern {
        if !file.name.to_lowercase().contains(&pattern.to_lowercase()) {
            return false;
        }
    }

    // Type filter
    if let Some(ft) = &filter.file_type {
        if &file.r#type != ft {
            return false;
        }
    }

    // Size filter
    if let Some(min) = filter.min_size {
        if file.size < min {
            return false;
        }
    }
    if let Some(max) = filter.max_size {
        if file.size > max {
            return false;
        }
    }

    // Extension filter (case-insensitive)
    if let Some(ext) = &filter.extension {
        if !file.name.to_lowercase().ends_with(&format!(".{}", ext.to_lowercase())) {
            return false;
        }
    }

    true
}
