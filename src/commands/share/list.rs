use cloudreve_api::{CloudreveClient, Result};
use log::info;

pub async fn handle_list(
    client: &CloudreveClient,
    page_size: Option<u32>,
    _order_by: Option<String>,
) -> Result<()> {
    info!("Listing share links...");

    let page_size = page_size.unwrap_or(50);
    let (shares, _next_token) = client
        .list_my_share_links_with_params(page_size, None, None, None)
        .await?;

    info!("Share links ({} total):", shares.len());
    for share in &shares {
        info!("  - ID: {}", share.id);
        info!("    Name: {}", share.name);
        info!("    URL: {}", share.url);
    }

    Ok(())
}
