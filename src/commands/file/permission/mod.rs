pub mod delete;
pub mod set;

use cloudreve_api::{CloudreveClient, Result};

#[derive(clap::Subcommand)]
pub enum PermissionCommands {
    /// Set file permissions
    Set {
        /// File URI
        #[clap(long)]
        uri: String,

        /// User explicit permissions (JSON)
        #[clap(long)]
        user_explicit: Option<String>,

        /// Group explicit permissions (JSON)
        #[clap(long)]
        group_explicit: Option<String>,

        /// Same group permission
        #[clap(long)]
        same_group: Option<String>,

        /// Other users permission
        #[clap(long)]
        other: Option<String>,

        /// Anonymous permission
        #[clap(long)]
        anonymous: Option<String>,

        /// Everyone permission
        #[clap(long)]
        everyone: Option<String>,
    },

    /// Delete file permissions
    Delete {
        /// File URI
        #[clap(long)]
        uri: String,
    },
}

pub async fn handle_permission(
    client: &CloudreveClient,
    command: PermissionCommands,
) -> Result<()> {
    match command {
        PermissionCommands::Set {
            uri,
            user_explicit,
            group_explicit,
            same_group,
            other,
            anonymous,
            everyone,
        } => {
            set::handle_set(
                client,
                uri,
                user_explicit,
                group_explicit,
                same_group,
                other,
                anonymous,
                everyone,
            )
            .await
        }

        PermissionCommands::Delete { uri } => delete::handle_delete(client, uri).await,
    }
}
