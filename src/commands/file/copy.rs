use cloudreve_api::{CloudreveAPI, Error, Result};
use log::error;
use log::info;

pub async fn handle_copy(api: &CloudreveAPI, src: Vec<String>, dest: String) -> Result<()> {
    info!("Copying {} file(s) to {}", src.len(), dest);

    // 扩展通配符模式
    let expanded_files = crate::utils::glob::expand_remote_patterns(api, &src, false).await?;
    info!("Expanded to {} file(s)", expanded_files.len());

    if expanded_files.is_empty() {
        error!("No files matched the specified patterns");
        return Err(Error::InvalidResponse("No files matched".to_string()));
    }

    // 使用并发控制执行批量复制
    let tasks: Vec<_> = expanded_files
        .iter()
        .map(|file_path| {
            let api = api.clone();
            let dest = dest.clone();
            let file_path = file_path.clone();
            (file_path.clone(), async move {
                api.copy_file(&file_path, &dest).await
            })
        })
        .collect();

    let results = crate::utils::concurrency::execute_with_concurrency(tasks, 5).await;

    // 统计结果
    let mut succeeded = 0;
    let mut failed = 0;
    for (file_path, result) in results {
        match result {
            Ok(_) => {
                info!("Copied: {}", file_path);
                succeeded += 1;
            }
            Err(e) => {
                error!("Failed to copy {}: {}", file_path, e);
                failed += 1;
            }
        }
    }

    info!("Copy complete: {} succeeded, {} failed", succeeded, failed);

    if failed > 0 {
        return Err(Error::InvalidResponse(format!(
            "Failed to copy {} out of {} files",
            failed,
            expanded_files.len()
        )));
    }

    Ok(())
}
