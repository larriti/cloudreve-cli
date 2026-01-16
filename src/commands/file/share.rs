use cloudreve_api::{CloudreveAPI, Result};
use log::{error, info};

pub async fn handle_share(
    api: &CloudreveAPI,
    uri: String,
    _name: Option<String>, // Unused due to API structure
    expire: Option<u32>,
    password: Option<String>,
) -> Result<()> {
    // Validate password format if provided
    if let Some(ref pwd) = password {
        if !pwd.chars().all(|c| c.is_alphanumeric()) {
            error!("Password can only contain letters and numbers (a-zA-Z0-9)");
            return Err(cloudreve_api::Error::InvalidResponse(
                "Password can only contain letters and numbers (a-zA-Z0-9)".to_string(),
            ));
        }
        if pwd.len() > 32 {
            error!("Password must be at most 32 characters");
            return Err(cloudreve_api::Error::InvalidResponse(
                "Password must be at most 32 characters".to_string(),
            ));
        }
    }

    info!("Creating share link for URI: {}", uri);
    if password.is_some() {
        info!("Password protected share");
    }

    match api.create_share(&uri, _name.as_deref(), expire, password.as_deref()).await {
        Ok(share_url) => {
            info!("Share link created successfully!");
            info!("URL: {}", share_url);
        }
        Err(e) => {
            error!("Error creating share link: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
