pub mod get;
pub mod set;

use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};

#[derive(clap::Subcommand)]
pub enum SettingsCommands {
    /// Get settings
    Get {
        /// Setting key (optional, get all if not specified)
        #[clap(long)]
        key: Option<String>,
    },

    /// Set a setting value
    Set {
        /// Setting key
        #[clap(long)]
        key: String,

        /// Setting value (JSON)
        #[clap(long)]
        value: String,
    },
}

pub async fn handle_settings_command(api: &CloudreveAPI, command: SettingsCommands) -> Result<()> {
    // For now, use V4 client through inner()
    match api.inner() {
        UnifiedClient::V4(client) => match command {
            SettingsCommands::Get { key } => get::handle_get(client, key).await,
            SettingsCommands::Set { key, value } => set::handle_set(client, key, value).await,
        },
        UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
            "Settings commands not yet supported for V3 API".to_string(),
        )),
    }
}
