//! Token management module for Cloudreve CLI
//!
//! Handles authentication token storage, retrieval, and validation.
//! Stores tokens in a single JSON file in ~/.cache/cloudreve-cli/tokens.json.

use cloudreve_api::api::v4::models::LoginData;
use cloudreve_api::error::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Token information structure for storage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    pub user_id: String,
    pub email: String,
    pub nickname: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires: String,
    pub refresh_expires: String,
    /// Cloudreve instance URL
    pub url: String,
}

impl TokenInfo {
    /// Creates a new TokenInfo from login data
    pub fn from_login_data(login_data: &LoginData, url: String) -> Self {
        Self {
            user_id: login_data.user.id.clone(),
            email: login_data.user.email.clone(),
            nickname: login_data.user.nickname.clone(),
            access_token: login_data.token.access_token.clone(),
            refresh_token: login_data.token.refresh_token.clone(),
            access_expires: login_data.token.access_expires.clone(),
            refresh_expires: login_data.token.refresh_expires.clone(),
            url,
        }
    }

    /// Checks if the access token is expired based on access_expires timestamp
    pub fn is_access_token_expired(&self) -> bool {
        // Parse the access_expires timestamp and compare with current time
        // The timestamp is usually in RFC3339 format like "2023-01-01T12:00:00Z"
        if self.access_expires.is_empty() {
            return true; // No expiration time means expired
        }

        // Parse the access_expires timestamp
        let expires_datetime = DateTime::parse_from_rfc3339(&self.access_expires)
            .map_err(|e| Error::InvalidTimestamp(e.to_string()))
            .unwrap_or_else(|_| Utc::now().into());

        // Compare with current UTC time
        let current_time = Utc::now();

        // If access_expires is before current time, token is expired
        expires_datetime < current_time
    }
}

/// Token manager for handling token persistence and validation
pub struct TokenManager {
    cache_dir: PathBuf,
    tokens_file: PathBuf,
}

impl TokenManager {
    /// Creates a new token manager with the default cache directory
    pub fn new() -> Result<Self, Error> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| {
                Error::Io(std::io::Error::other(
                    "Could not determine cache directory",
                ))
            })?
            .join("cloudreve-cli");

        let tokens_file = cache_dir.join("tokens.json");

        Ok(TokenManager {
            cache_dir,
            tokens_file,
        })
    }

    /// Loads all token information from the single tokens file
    pub fn load_all_tokens(&self) -> Result<Vec<TokenInfo>, Error> {
        if !self.tokens_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.tokens_file)?;
        let tokens: Vec<TokenInfo> = serde_json::from_str(&content).map_err(|e| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse tokens file: {}", e),
            ))
        })?;
        Ok(tokens)
    }

    /// Saves all token information to a single file
    pub fn save_all_tokens(&self, tokens: &[TokenInfo]) -> Result<(), Error> {
        // Ensure cache directory exists
        fs::create_dir_all(&self.cache_dir)?;

        let json_content = serde_json::to_string_pretty(tokens)?;
        fs::write(&self.tokens_file, json_content)?;

        Ok(())
    }

    /// Gets token for a specific email
    pub fn get_token_by_email(&self, email: &str) -> Result<Option<TokenInfo>, Error> {
        let tokens = self.load_all_tokens()?;
        let token = tokens.into_iter().find(|t| t.email == email);
        Ok(token)
    }

    /// Adds or updates a token in the collection
    pub fn save_token(&self, token_info: &TokenInfo) -> Result<(), Error> {
        let mut tokens = self.load_all_tokens()?;

        // Check if token for this email already exists
        let existing_index = tokens.iter().position(|t| t.email == token_info.email);

        if let Some(index) = existing_index {
            // Update existing token
            tokens[index] = token_info.clone();
        } else {
            // Add new token
            tokens.push(token_info.clone());
        }

        self.save_all_tokens(&tokens)
    }

    /// Removes a token for a specific email
    pub fn _remove_token(&self, email: &str) -> Result<(), Error> {
        let mut tokens = self.load_all_tokens()?;
        tokens.retain(|t| t.email != email);
        self.save_all_tokens(&tokens)
    }

    /// Gets the default token (first one if multiple, or the only one if single)
    pub fn get_default_token(&self) -> Result<Option<TokenInfo>, Error> {
        let tokens = self.load_all_tokens()?;
        Ok(tokens.first().cloned())
    }

    /// Checks if the refresh token is expired based on refresh_expires timestamp
    /// This parses the timestamp and compares it with current time
    pub fn is_refresh_token_expired(&self, token_info: &TokenInfo) -> bool {
        // Parse the refresh_expires timestamp and compare with current time
        // The timestamp is usually in RFC3339 format like "2023-01-01T12:00:00Z"
        if token_info.refresh_expires.is_empty() {
            return true; // No expiration time means expired
        }

        // Parse the refresh_expires timestamp
        let expires_datetime = DateTime::parse_from_rfc3339(&token_info.refresh_expires)
            .map_err(|e| Error::InvalidTimestamp(e.to_string()))
            .unwrap_or_else(|_| Utc::now().into());

        // Compare with current UTC time
        let current_time = Utc::now();

        // If refresh_expires is before current time, token is expired
        expires_datetime < current_time
    }
}
