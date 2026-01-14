use cloudreve_api::api::v4::models::*;
use cloudreve_api::{CloudreveClient, Result};
use crate::context::token_manager::TokenInfo;
use crate::context::TokenManager;
use log::{debug, error, info};
use rpassword::read_password;
use std::io::{self, Write};

pub async fn handle_auth(
    client: &mut CloudreveClient,
    token_manager: &TokenManager,
    email: Option<String>,
    url: Option<String>,
    password: Option<String>,
) -> Result<()> {
    info!("Authenticating with email: {:?}", email);

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

    let login_request = LoginRequest {
        email: &email,
        password: &password,
    };

    match client.login(&login_request).await {
        Ok(login_data) => {
            info!("Authentication successful!");
            info!("User ID: {}", login_data.user.id);
            info!("User Email: {}", login_data.user.email);
            info!("User Nickname: {}", login_data.user.nickname);
            debug!("Access Token: {}", login_data.token.access_token);
            debug!("Refresh Token: {}", login_data.token.refresh_token);
            debug!("Access Expires: {}", login_data.token.access_expires);
            debug!("Refresh Expires: {}", login_data.token.refresh_expires);

            // Store the token for subsequent requests
            let access_token = login_data.token.access_token.clone();
            client.set_token(access_token);

            // Save token to cache
            let token_info = TokenInfo::from_login_data(&login_data, url);
            token_manager.save_token(&token_info)?;
            info!(
                "Token saved to cache for user: {}({})",
                login_data.user.nickname, login_data.user.email
            );
        }
        Err(e) => {
            error!("Authentication failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
