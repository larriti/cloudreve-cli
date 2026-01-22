// Site command module

pub mod config;
pub mod get;

use cloudreve_api::{CloudreveAPI, Result};

const SECTIONS: &[&str] = &[
    "basic", "login", "explorer", "emojis", "vas", "app", "thumb",
];

#[derive(clap::Subcommand)]
pub enum SiteCommands {
    /// Get site configuration
    Get {
        /// Configuration section
        #[clap(short, long, default_value = "basic", value_parser = clap::builder::PossibleValuesParser::new(SECTIONS))]
        section: String,
    },
}

pub async fn handle_site_command(api: &CloudreveAPI, command: SiteCommands) -> Result<()> {
    match command {
        SiteCommands::Get { section } => get::handle_get(api, section).await,
    }
}
