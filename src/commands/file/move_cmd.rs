use cloudreve_api::{CloudreveAPI, Result};
use log::error;
use log::info;

pub async fn handle_move(api: &CloudreveAPI, src: Vec<String>, dest: String) -> Result<()> {
    info!("Moving {} file(s) to {}", src.len(), dest);

    let mut succeeded = 0;
    let mut failed = 0;

    for src_path in &src {
        match api.move_file(src_path, &dest).await {
            Ok(_) => {
                info!("Moved: {}", src_path);
                succeeded += 1;
            }
            Err(e) => {
                error!("Failed to move {}: {}", src_path, e);
                failed += 1;
            }
        }
    }

    info!("Move complete: {} succeeded, {} failed", succeeded, failed);

    if failed > 0 {
        error!("Failed to move {} out of {} files", failed, src.len());
    }

    Ok(())
}
