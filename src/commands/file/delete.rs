use cloudreve_api::{CloudreveAPI, Result};
use log::{error, info};
use std::io::{self, Write};

use crate::utils::glob;

pub async fn handle_delete(
    api: &CloudreveAPI,
    uris: Vec<String>,
    force: bool,
    _recursive: bool,
) -> Result<()> {
    if uris.is_empty() {
        error!("No files specified for deletion");
        return Ok(());
    }

    // Expand wildcard patterns (including /*, *.gz, etc.)
    // For delete, we include folders to support /* pattern
    let paths_to_delete = glob::expand_remote_patterns(api, &uris, true).await?;

    if paths_to_delete.is_empty() {
        info!("No files to delete");
        return Ok(());
    }

    // Confirmation
    if !force {
        println!("Delete operation:");
        println!("  Items: {}", paths_to_delete.len());
        for path in &paths_to_delete {
            println!("  - {}", path);
        }
        print!("Proceed? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            info!("Operation cancelled");
            return Ok(());
        }
    }

    // Convert to slice for batch_delete
    let paths_refs: Vec<&str> = paths_to_delete.iter().map(|s| s.as_str()).collect();

    match api.batch_delete(&paths_refs).await {
        Ok(result) => {
            info!(
                "Delete complete: {} deleted, {} failed",
                result.deleted, result.failed
            );

            for (path, error) in &result.errors {
                error!("Failed to delete {}: {}", path, error);
            }

            if result.failed > 0 {
                error!("Failed to delete {} items", result.failed);
            }
        }
        Err(e) => {
            error!("Delete operation failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
