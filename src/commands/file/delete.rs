use cloudreve_api::{CloudreveAPI, Result};
use log::{error, info};
use std::io::{self, Write};

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

    // Parse paths and expand wildcard patterns
    let mut paths_to_delete: Vec<String> = Vec::new();

    for uri in &uris {
        if uri.ends_with("/*") {
            // Wildcard pattern: clear folder contents
            let base_path = uri.trim_end_matches("/*");
            let normalized = if base_path.is_empty() { "/" } else { base_path };

            // List all items in the folder
            match api.list_files(normalized, None, None).await {
                Ok(file_list) => {
                    for item in file_list.items() {
                        let full_path =
                            format!("{}/{}", normalized.trim_end_matches('/'), item.name);
                        paths_to_delete.push(full_path);
                    }
                }
                Err(e) => {
                    error!("Failed to list directory {}: {}", normalized, e);
                }
            }
        } else {
            // Regular path
            paths_to_delete.push(uri.clone());
        }
    }

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
