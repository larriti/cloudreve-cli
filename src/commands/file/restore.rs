use cloudreve_api::api::v4::models::RestoreFileRequest;
use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_restore(client: &CloudreveClient, uris: Vec<String>) -> Result<()> {
    if uris.is_empty() {
        return Ok(());
    }

    info!("Restoring {} file(s) from trash", uris.len());

    let uri_refs: Vec<&str> = uris.iter().map(|u| u.as_str()).collect();
    let request = RestoreFileRequest { uris: uri_refs };

    client.restore_from_trash(&request).await?;

    info!("Restore completed successfully");
    Ok(())
}
