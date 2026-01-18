use cloudreve_api::{CloudreveClient, Result};
use log::{error, info};

pub async fn handle_policies(client: &CloudreveClient) -> Result<()> {
    info!("Fetching available storage policies...");

    match client.get_storage_policies().await {
        Ok(policies) => {
            if policies.is_empty() {
                info!("No storage policies available.");
                return Ok(());
            }

            info!("Available storage policies:");
            info!("");
            for (index, policy) in policies.iter().enumerate() {
                info!("  [{}] {} (ID: {})", index, policy.name, policy.id);
                info!("      Type: {}", policy.type_);
                info!("      Max Size: {} MB", policy.max_size);
                if let Some(relay) = policy.relay {
                    info!("      Relay: {}", relay);
                }
                info!("");
            }

            info!("Use the policy ID with --policy parameter when uploading files:");
            info!(
                "  Example: cloudreve-cli file upload --file <file> --path <path> --policy {}",
                policies[0].id
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch storage policies: {}", e);
            error!("This feature may not be available on your Cloudreve instance.");
            error!("Please manually specify a storage policy ID using --policy parameter.");
            Err(e)
        }
    }
}
