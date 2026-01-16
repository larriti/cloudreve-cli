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
    // For now, use V4 client through inner()
    match api.inner() {
        UnifiedClient::V4(client) => match command {
            UserCommands::Info => info::handle_info(client, token_manager).await,
            UserCommands::Quota => quota::handle_quota(client).await,
            UserCommands::Policies => policies::handle_policies(client).await,
            UserCommands::UpdateProfile { nickname, avatar } => {
                update_profile::handle_update_profile(client, nickname, avatar).await
            }
            UserCommands::ChangePassword {
                old_password,
                new_password,
            } => change_password::handle_change_password(client, old_password, new_password).await,
        },
        UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("User commands not yet supported for V3 API".to_string())),
    }
}
