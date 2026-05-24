use crate::context::token_manager::TokenInfo as CliTokenInfo;
use crate::context::token_manager::TokenManager;
use crate::utils::captcha;
use cloudreve_api::error::Error;
use cloudreve_api::{CloudreveAPI, LoginResponse, Result};
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

    // Clear any existing authentication to ensure we get fresh site config
    // This is important for V3 which uses session cookies
    api.clear_auth();
    debug!("Cleared existing authentication for fresh login check");

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

    // Try login with CAPTCHA retry support
    // First attempt without CAPTCHA, if server returns error 40026, prompt for CAPTCHA
    let mut captcha_code: Option<String> = None;
    let mut captcha_ticket: Option<String> = None;

    let login_response = loop {
        debug!(
            "Attempting login with CAPTCHA: code={:?}, ticket={:?}",
            captcha_code, captcha_ticket
        );

        let result = api
            .login_with_captcha(
                &email,
                &password,
                captcha_code.as_deref(),
                captcha_ticket.as_deref(),
            )
            .await;

        match result {
            Ok(response) => break response,
            Err(Error::Api { code: 40026, .. }) => {
                // CAPTCHA required - error 40026 means "CAPTCHA not match"
                info!("CAPTCHA required by server");

                let captcha_info = api.get_captcha().await?;

                // Display CAPTCHA to user
                let display = captcha::display_captcha(&captcha_info.image)
                    .map_err(|e| Error::Auth(format!("Failed to display CAPTCHA: {}", e)))?;
                captcha::print_captcha_prompt(&display);

                // Flush stdout to ensure prompt is visible
                let _ = io::stdout().flush();

                let mut captcha_input = String::new();
                io::stdin().read_line(&mut captcha_input)?;
                let code = captcha_input.trim().to_string();

                captcha_code = Some(code);
                // For V3, ticket is empty, convert to None
                captcha_ticket = if captcha_info.ticket.is_empty() {
                    None
                } else {
                    Some(captcha_info.ticket)
                };

                debug!(
                    "Retrying with CAPTCHA: code={:?}, ticket={:?}",
                    captcha_code, captcha_ticket
                );

                // Retry login with CAPTCHA
                continue;
            }
            Err(Error::TwoFactorRequired(session_id)) => {
                if session_id.is_empty() {
                    info!("Two-factor authentication required");
                } else {
                    info!(
                        "Two-factor authentication required (session ID: {})",
                        session_id
                    );
                }
                print!("Enter your 6-digit OTP code: ");
                io::stdout().flush()?;
                let mut otp_input = String::new();
                io::stdin().read_line(&mut otp_input)?;
                let otp_code = otp_input.trim().to_string();

                // Validate OTP code is 6 digits
                if otp_code.len() != 6 || !otp_code.chars().all(|c| c.is_ascii_digit()) {
                    return Err(Error::Auth(
                        "Invalid OTP code. Must be exactly 6 digits.".to_string(),
                    ));
                }

                break api.login_2fa(&otp_code).await?;
            }
            Err(e) => return Err(e),
        }
    };

    // Get API version
    let api_version = api.api_version().to_string();

    // Extract user info and token info from response
    let (user_id, nickname, cli_token_info) = match &login_response {
        LoginResponse::V3(r) => {
            // V3 uses session cookie - get it from the client
            let session_cookie = api.get_session_cookie().unwrap_or_else(|| {
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
    info!("Token saved to cache for user: {}({})", nickname, email);

    Ok(())
}
