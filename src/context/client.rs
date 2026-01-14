use cloudreve_api::{CloudreveClient, RefreshTokenRequest, Result};
use log::{error, info, warn};
use super::token_manager::{TokenManager, TokenInfo};

/// Client initialization configuration
pub struct ClientConfig {
    pub url: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

/// Client initialization result
pub struct ClientContext {
    pub client: Option<CloudreveClient>,
    pub token_manager: TokenManager
}

/// Initializes client and handles token management
pub async fn initialize_client(config: ClientConfig) -> Result<ClientContext> {
    let token_manager = TokenManager::new()?;
    let token_provided = config.token.is_some();

    let client = if !token_provided {
        // Try to load from cache
        info!("Checking for cached token...");

        let token_result = load_cached_token(&token_manager, config.email.as_ref())?;

        match token_result {
            Some(token_info) => {
                info!("Found cached token for user: {}", token_info.email);
                let url_from_cache = token_info.url.clone();
                let mut temp_client = CloudreveClient::new(&url_from_cache);

                // Check if access token is expired
                if token_info.is_access_token_expired() {
                    warn!("Access token is expired. Attempting to refresh...");
                    let refresh_result = refresh_access_token(&mut temp_client, &token_manager, &token_info).await;

                    match refresh_result {
                        Ok(true) => {
                            info!("Access token refreshed successfully!");
                        }
                        Ok(false) => {
                            // Check if refresh token is also expired
                            if token_manager.is_refresh_token_expired(&token_info) {
                                error!("Both access and refresh tokens expired. Please authenticate again.");
                                std::process::exit(1);
                            }
                            // Refresh token still valid, use old access token
                            warn!("Using old access token despite refresh failure");
                            temp_client.set_token(token_info.access_token.clone());
                        }
                        Err(e) => {
                            error!("Error during token refresh: {}", e);
                            if token_manager.is_refresh_token_expired(&token_info) {
                                std::process::exit(1);
                            }
                            temp_client.set_token(token_info.access_token.clone());
                        }
                    }
                } else {
                    temp_client.set_token(token_info.access_token.clone());
                    info!("Using cached access token for authentication.");
                }

                Some(temp_client)
            }
            None => {
                // No cached token found - caller will handle
                None
            }
        }
    } else {
        // Token provided via command line
        let url = config
            .url
            .expect("URL is required when token is provided");
        let mut temp_client = CloudreveClient::new(&url);
        temp_client.set_token(config.token.unwrap());
        Some(temp_client)
    };

    Ok(ClientContext {
        client,
        token_manager
    })
}

/// Attempts to load a cached token
fn load_cached_token(
    token_manager: &TokenManager,
    email: Option<&String>,
) -> Result<Option<TokenInfo>> {
    

    match email {
        Some(email) => {
            if token_manager
                .get_token_by_email(email)
                .is_ok_and(|t| t.is_some())
            {
                token_manager.get_token_by_email(email)
            } else {
                Ok(None)
            }
        }
        None => token_manager.get_default_token(),
    }
}

/// Attempts to refresh the access token
async fn refresh_access_token(
    client: &mut CloudreveClient,
    token_manager: &TokenManager,
    token_info: &TokenInfo,
) -> Result<bool> {
    let refresh_request = RefreshTokenRequest {
        refresh_token: &token_info.refresh_token,
    };

    match client.refresh_token(&refresh_request).await {
        Ok(refreshed_token) => {
            client.set_token(refreshed_token.access_token.clone());
            let updated_token_info = TokenInfo {
                user_id: token_info.user_id.clone(),
                email: token_info.email.clone(),
                nickname: token_info.nickname.clone(),
                access_token: refreshed_token.access_token,
                refresh_token: refreshed_token.refresh_token,
                access_expires: refreshed_token.access_expires,
                refresh_expires: refreshed_token.refresh_expires,
                url: token_info.url.clone(),
            };
            token_manager.save_token(&updated_token_info)?;
            info!("Updated token saved to cache.");
            Ok(true)
        }
        Err(e) => {
            warn!("Failed to refresh access token: {}. Checking refresh token...", e);
            Ok(false)
        }
    }
}
