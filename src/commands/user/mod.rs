pub mod change_password;
pub mod info;
pub mod policies;
pub mod quota;
pub mod update_profile;

use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};
use crate::context::TokenManager;

#[derive(clap::Subcommand)]
pub enum UserCommands {
    /// Get current user information
    Info,

    /// View user storage quota
    Quota,

    /// List available storage policies
    Policies,

    /// Update user profile
    UpdateProfile {
        /// New nickname
        #[clap(long)]
        nickname: Option<String>,

        /// New avatar
        #[clap(long)]
        avatar: Option<String>,
    },

    /// Change password
    ChangePassword {
        /// Current password
        #[clap(long)]
        old_password: String,

        /// New password
        #[clap(long)]
        new_password: String,
    },
}

pub async fn handle_user_command(
    api: &CloudreveAPI,
    token_manager: &TokenManager,
    command: UserCommands,
) -> Result<()> {
    match command {
        UserCommands::Info => info::handle_info(api, token_manager).await,
        UserCommands::Quota => quota::handle_quota(api).await,
        UserCommands::Policies => {
            // Policies is not supported in V3
            match api.inner() {
                UnifiedClient::V4(client) => policies::handle_policies(client).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Storage policies list not available in V3 API".to_string()
                )),
            }
        }
        UserCommands::UpdateProfile { nickname, avatar } => {
            // Update profile is not supported in V3
            match api.inner() {
                UnifiedClient::V4(client) => update_profile::handle_update_profile(client, nickname, avatar).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Update profile not available in V3 API".to_string()
                )),
            }
        }
        UserCommands::ChangePassword {
            old_password,
            new_password,
        } => {
            // Change password is not supported in V3
            match api.inner() {
                UnifiedClient::V4(client) => change_password::handle_change_password(client, old_password, new_password).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Change password not available in V3 API".to_string()
                )),
            }
        }
    }
}
