use cloudreve_api::{CloudreveAPI, VERSION};
use log::info;

pub async fn handle_version(api: &CloudreveAPI) {
    let cli_version = env!("CARGO_PKG_VERSION");
    let api_library_version = VERSION;

    info!("CLI Version: {}", cli_version);
    info!("API Library Version: {}", api_library_version);
    info!("Detected API Version: {}", api.api_version());
    info!("Base URL: {}", api.base_url());
}
