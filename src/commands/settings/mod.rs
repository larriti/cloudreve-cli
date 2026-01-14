pub mod get;
pub mod set;

use cloudreve_api::{CloudreveClient, Result};

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

pub async fn handle_settings_command(
    client: &CloudreveClient,
    command: SettingsCommands,
) -> Result<()> {
    match command {
        SettingsCommands::Get { key } => get::handle_get(client, key).await,
        SettingsCommands::Set { key, value } => set::handle_set(client, key, value).await,
    }
}
