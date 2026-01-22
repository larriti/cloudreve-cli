use cloudreve_api::{CloudreveAPI, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Upload a single file
pub async fn upload_single_file(
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
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message("Uploading");

    // 5. Upload using CloudreveAPI (handles V3/V4 differences internally)
    api.upload_file(&upload_path, file_content, policy_id.as_deref())
        .await?;

    pb.finish_with_message("Upload completed!");
    info!("File uploaded successfully!");

    Ok(())
}

/// Handle upload with support for multiple files, glob patterns, and concurrency
pub async fn handle_upload(
    api: &CloudreveAPI,
    files: Vec<String>,
    path: String,
    overwrite: bool,
    policy: Option<String>,
    recursive: bool,
    concurrency: usize,
) -> Result<()> {
    // 1. Expand glob patterns
    let expanded_files = crate::utils::glob::expand_glob_patterns(&files);

    if expanded_files.is_empty() {
        return Err(cloudreve_api::Error::InvalidResponse(
            "No files to upload".to_string(),
        ));
    }

    // 2. If directory and recursive=true, collect files recursively
    let all_files = if recursive {
        collect_files_recursive(&expanded_files)?
    } else {
        expanded_files
    };

    if all_files.is_empty() {
        return Err(cloudreve_api::Error::InvalidResponse(
            "No files to upload".to_string(),
        ));
    }

    info!(
        "Starting upload of {} file(s) to {}",
        all_files.len(),
        path
    );

    // 3. Use concurrency control to upload
    let tasks: Vec<_> = all_files
        .into_iter()
        .map(|file_path| {
            let api = api.clone();
            let path = path.clone();
            let policy = policy.clone();
            let file_name =
                Path::new(&file_path).file_name().unwrap().to_string_lossy().to_string();

            (
                file_name.clone(),
                async move {
                    upload_single_file(&api, file_path, path, overwrite, policy).await
                },
            )
        })
        .collect();

    let results = crate::utils::concurrency::execute_with_concurrency(tasks, concurrency).await;

    // 4. Statistics
    let mut success = 0;
    let mut failed = 0;
    for (name, result) in results {
        match result {
            Ok(_) => {
                success += 1;
                info!("✓ {}", name);
            }
            Err(e) => {
                failed += 1;
                error!("✗ {}: {}", name, e);
            }
        }
    }

    info!("");
    info!("Upload complete: {} succeeded, {} failed", success, failed);

    Ok(())
}

/// Recursively collect files from directories
fn collect_files_recursive(paths: &[String]) -> Result<Vec<String>> {
    let mut result = Vec::new();

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            result.push(path.clone());
        } else if path_obj.is_dir() {
            collect_files_from_dir(path_obj, &mut result)?;
        }
    }

    Ok(result)
}

/// Collect all files from a directory recursively
fn collect_files_from_dir(dir: &Path, result: &mut Vec<String>) -> Result<()> {
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                result.push(path_str.to_string());
            }
        } else if path.is_dir() {
            collect_files_from_dir(&path, result)?;
        }
    }
    Ok(())
}
