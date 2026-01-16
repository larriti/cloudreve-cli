use cloudreve_api::{CloudreveAPI, LoginResponse, Result};
use crate::context::token_manager::TokenManager;
use crate::context::token_manager::TokenInfo as CliTokenInfo;
use log::{debug, info};
use rpassword::read_password;
use std::io::{self, Write};

pub async fn handle_auth(
    api: &mut CloudreveAPI,
    token_manager: &TokenManager,
    email: Option<String>,
    url: Option<String>,
    password: Option<String>,
) -> Result<()> {
    info!("Authenticating...");

    // If no URL provided, we need to ask for it
    let url = if let Some(url) = url {
        url
    } else {
        print!("Enter Cloudreve instance URL: ");
        io::stdout().flush()?;
        let mut url_input = String::new();
        io::stdin().read_line(&mut url_input)?;
        url_input.trim().to_string()
    };

    // If no email provided, we need to ask for it
    let email = if let Some(email) = email {
        email
    } else {
        print!("Enter your email: ");
        io::stdout().flush()?;
        let mut email_input = String::new();
        io::stdin().read_line(&mut email_input)?;
        email_input.trim().to_string()
    };

    // If no password provided, we need to ask for it securely
    let password = if let Some(pwd) = password {
        pwd
    } else {
        print!("Enter your password: ");
        io::stdout().flush()?;
        let password_input = read_password().unwrap_or_default();
        password_input.trim().to_string()
    };

    // Use the unified login method
    let login_response = api.login(&email, &password).await?;

    // Get API version
    let api_version = api.api_version().to_string();

    // Extract user info and token info from response
    let (user_id, nickname, cli_token_info) = match &login_response {
        LoginResponse::V3(r) => {
            // V3 uses session cookie - get it from the client
            let session_cookie = api.get_session_cookie()
                .unwrap_or_else(|| {
                    debug!("No session cookie found after V3 login");
                    String::new()
                });

            if session_cookie.is_empty() {
                debug!("Warning: V3 login returned empty session cookie");
            }

            let api_v = api_version.clone();
            (
                r.user.id.clone(),
                r.user.nickname.clone(),
                CliTokenInfo {
                    user_id: r.user.id.clone(),
                    email: email.clone(),
                    nickname: r.user.nickname.clone(),
                    access_token: session_cookie, // Save the session cookie
                    refresh_token: String::new(),
                    access_expires: String::new(),
                    refresh_expires: String::new(),
                    url: url.clone(),
                    api_version: api_v,
                },
            )
        }
        LoginResponse::V4(r) => {
            // V4 has proper JWT tokens
            let api_v = api_version.clone();
            (
                r.user.id.clone(),
                r.user.nickname.clone(),
                CliTokenInfo {
                    user_id: r.user.id.clone(),
                    email: email.clone(),
                    nickname: r.user.nickname.clone(),
                    access_token: r.token.access_token.clone(),
                    refresh_token: r.token.refresh_token.clone(),
                    access_expires: r.token.access_expires.clone(),
                    refresh_expires: r.token.refresh_expires.clone(),
                    url: url.clone(),
                    api_version: api_v,
                },
            )
        }
    };

    info!("Authentication successful!");
    info!("User ID: {}", user_id);
    info!("User Nickname: {}", nickname);
    info!("API Version: {}", api_version);

    debug!("Login response: {:?}", login_response);

    // Save token to cache
    token_manager.save_token(&cli_token_info)?;
    info!(
        "Token saved to cache for user: {}({})",
        nickname, email
    );

    Ok(())
}
