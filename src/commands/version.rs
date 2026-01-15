use cloudreve_api::{CloudreveClient, VERSION};
use log::info;

pub async fn handle_version(client: &CloudreveClient) {
    let cli_version = env!("CARGO_PKG_VERSION");
    let api_library_version = VERSION;

    info!("CLI Version: {}", cli_version);
    info!("API Library Version: {}", api_library_version);
    info!("API Version: v4");
    info!("Base URL: {}", client.base_url);

    // Try to get server version if available
    match client.get_version().await {
        Ok(version_info) => {
            info!("Server Version: {}", version_info.server_version);
            info!("Detected API Version: {}", version_info.api_version);
        }
        Err(e) => {
            info!("Could not retrieve server version: {}", e);
        }
    }
}
