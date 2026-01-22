use cloudreve_api::{CloudreveAPI, Result};
use log::info;

use super::config;

pub async fn handle_get(api: &CloudreveAPI, section: String) -> Result<()> {
    info!("Fetching site configuration: {}", section);

    let config = api.get_site_config(Some(&section)).await?;
    config::display_config(&config, &section);

    Ok(())
}
