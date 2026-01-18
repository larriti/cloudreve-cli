use super::token_manager::{TokenInfo as CliTokenInfo, TokenManager};
use cloudreve_api::{ApiVersion, CloudreveAPI, Result, UnifiedClient};
use log::{debug, info, warn};

/// Client initialization configuration
pub struct ClientConfig {
    pub url: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

/// Client initialization result
pub struct ClientContext {
    pub api: Option<CloudreveAPI>,
    pub token_manager: TokenManager,
}

/// Initializes client and handles token management
pub async fn initialize_client(config: ClientConfig) -> Result<ClientContext> {
    let token_manager = TokenManager::new()?;
    let token_provided = config.token.is_some();

    let api = if !token_provided {
        // Try to load from cache
        info!("Checking for cached token...");

        let token_result = load_and_refresh_token(
            &token_manager,
            config.url.as_deref(),
            config.email.as_deref(),
        )
        .await?;

        match token_result {
            Some((token_info, api)) => {
                info!("Using token for user: {}", token_info.email);
                Some(api)
            }
            None => {
                // No valid token found - caller will handle login
                None
            }
        }
    } else {
        // Token provided via command line
        let url = config.url.expect("URL is required when token is provided");
        let mut api = CloudreveAPI::new(&url).await?;
        api.set_token(&config.token.unwrap())?;
        Some(api)
    };

    Ok(ClientContext { api, token_manager })
}

/// Attempts to load and refresh a cached token if needed
async fn load_and_refresh_token(
    token_manager: &TokenManager,
    url: Option<&str>,
    email: Option<&str>,
) -> Result<Option<(CliTokenInfo, CloudreveAPI)>> {
    // Load token from cache based on URL and/or email
    let token_info = match (url, email) {
        (Some(url), Some(email)) => {
            // Both URL and email provided - find matching token
            let result = token_manager.get_token_by_url_and_email(url, email)?;
            if let Some(token) = result {
                token
            } else {
                info!(
                    "No cached token found for URL: {} and email: {}",
                    url, email
                );
                return Ok(None);
            }
        }
        (Some(url), None) => {
            // Only URL provided - find token by URL
            let result = token_manager.get_token_by_url(url)?;
            if let Some(token) = result {
                token
            } else {
                info!("No cached token found for URL: {}", url);
                return Ok(None);
            }
        }
        (None, Some(email)) => {
            // Only email provided - find token by email
            let result = token_manager.get_token_by_email(email)?;
            if let Some(token) = result {
                token
            } else {
                info!("No cached token found for email: {}", email);
                return Ok(None);
            }
        }
        (None, None) => {
            // Neither provided - use default token
            let result = token_manager.get_default_token()?;
            if let Some(token) = result {
                token
            } else {
                info!("No cached token found");
                return Ok(None);
            }
        }
    };

    info!(
        "Found cached token for user: {} (API version: {})",
        token_info.email, token_info.api_version
    );
    debug!("token: {}", token_info.access_token);
    let cached_url = token_info.url.clone();

    // When URL is provided, verify it matches the cached token's URL
    if let Some(provided_url) = url {
        let normalized_provided = provided_url.trim_end_matches('/');
        let normalized_cached = cached_url.trim_end_matches('/');
        if normalized_provided != normalized_cached {
            info!(
                "Provided URL ({}) differs from cached token URL ({}), ignoring cache",
                provided_url, cached_url
            );
            return Ok(None);
        }
    }

    // Parse the API version from cache
    let api_version = ApiVersion::from_str_inner(&token_info.api_version).unwrap_or(ApiVersion::V4); // Default to V4 for backward compatibility

    // Create API client with known version (no probing needed!)
    info!(
        "Using cached API version: {:?} (skipping version detection)",
        api_version
    );
    let mut api = CloudreveAPI::with_version(&cached_url, api_version)?;

    // V3 tokens don't have expiration info - use directly and let API errors handle auth
    if api_version == ApiVersion::V3 {
        info!("V3 token detected - skipping expiration check");
        // V3 uses session cookie, set it directly
        let session_cookie = token_info.access_token.clone();
        if let UnifiedClient::V3(client) = api.inner_mut() {
            client.set_session_cookie(session_cookie);
        }
        return Ok(Some((token_info, api)));
    }

    // For V4, check if access token is expired
    if token_info.is_access_token_expired() {
        info!("Access token expired, attempting to refresh...");

        // Check if refresh token is also expired
        if token_manager.is_refresh_token_expired(&token_info) {
            warn!("Refresh token also expired, please login again");
            return Ok(None);
        }

        // Attempt to refresh the token
        match refresh_token(&api, &token_info, token_manager).await {
            Ok(new_token_info) => {
                info!("Token refreshed successfully");
                // Set the new access token
                api.set_token(&new_token_info.access_token)?;
                Ok(Some((new_token_info, api)))
            }
            Err(e) => {
                warn!("Failed to refresh token: {}", e);
                warn!("Please login again");
                Ok(None)
            }
        }
    } else {
        // Token is still valid, use it directly
        // V4 uses JWT token
        api.set_token(&token_info.access_token)?;
        Ok(Some((token_info, api)))
    }
}

/// Refreshes an expired token using the refresh token
async fn refresh_token(
    api: &CloudreveAPI,
    token_info: &CliTokenInfo,
    token_manager: &TokenManager,
) -> Result<CliTokenInfo> {
    match api.inner() {
        UnifiedClient::V4(client) => {
            use cloudreve_api::api::v4::models::RefreshTokenRequest;

            let request = RefreshTokenRequest {
                refresh_token: &token_info.refresh_token,
            };

            let new_token = client.refresh_token(&request).await?;

            // Create updated token info
            let updated_token_info = CliTokenInfo {
                user_id: token_info.user_id.clone(),
                email: token_info.email.clone(),
                nickname: token_info.nickname.clone(),
                access_token: new_token.access_token.clone(),
                refresh_token: new_token.refresh_token.clone(),
                access_expires: new_token.access_expires.clone(),
                refresh_expires: new_token.refresh_expires.clone(),
                url: token_info.url.clone(),
                api_version: token_info.api_version.clone(), // Keep the same API version
            };

            // Save the updated token to cache
            token_manager.save_token(&updated_token_info)?;

            Ok(updated_token_info)
        }
        UnifiedClient::V3(_) => {
            // V3 doesn't support token refresh
            Err(cloudreve_api::Error::InvalidResponse(
                "Token refresh not supported for V3 API".to_string(),
            ))
        }
    }
}
