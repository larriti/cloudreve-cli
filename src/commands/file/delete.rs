use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};
use std::io::{self, Write};

pub async fn handle_delete(
    client: &CloudreveClient,
    uris: Vec<String>,
    force: bool,
) -> Result<()> {
    if uris.is_empty() {
        error!("No files specified for deletion");
        return Ok(());
    }

    // Confirmation prompt
    if !force {
        print!("Delete {} file(s)? [y/N]: ", uris.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            info!("Operation cancelled");
            return Ok(());
        }
    }

    let mut succeeded = 0;
    let mut failed = 0;

    for uri in &uris {
        match client.delete_file(uri).await {
            Ok(_) => {
                info!("Deleted: {}", uri);
                succeeded += 1;
            }
            Err(e) => {
                error!("Failed to delete {}: {}", uri, e);
                failed += 1;
            }
        }
    }

    info!("Delete complete: {} succeeded, {} failed", succeeded, failed);

    if failed > 0 {
        error!("Failed to delete {} out of {} files", failed, uris.len());
    }

    Ok(())
}
