pub mod update;

use cloudreve_api::{CloudreveClient, Result};

#[derive(clap::Subcommand)]
pub enum MetadataCommands {
    /// Update file metadata
    Update {
        /// File URI
        #[clap(long)]
        uri: String,

        /// Metadata JSON string
        #[clap(long)]
        metadata: String,

        /// Clear existing metadata
        #[clap(long)]
        clear: bool,
    },
}

pub async fn handle_metadata(
    client: &CloudreveClient,
    command: MetadataCommands,
) -> Result<()> {
    match command {
        MetadataCommands::Update {
            uri,
            metadata,
            clear,
        } => update::handle_update(client, uri, metadata, clear).await,
    }
}
