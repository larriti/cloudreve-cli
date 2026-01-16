use cloudreve_api::{CloudreveAPI, Result};
use log::info;
use log::error;

pub async fn handle_copy(
    api: &CloudreveAPI,
    src: Vec<String>,
    dest: String,
) -> Result<()> {
    info!("Copying {} file(s) to {}", src.len(), dest);

    let mut succeeded = 0;
    let mut failed = 0;

    for src_path in &src {
        match api.copy_file(src_path, &dest).await {
            Ok(_) => {
                info!("Copied: {}", src_path);
                succeeded += 1;
            }
            Err(e) => {
                error!("Failed to copy {}: {}", src_path, e);
                failed += 1;
            }
        }
    }

    info!("Copy complete: {} succeeded, {} failed", succeeded, failed);

    if failed > 0 {
        error!("Failed to copy {} out of {} files", failed, src.len());
    }

    Ok(())
}
