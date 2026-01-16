pub mod create;
pub mod delete;
pub mod list;
pub mod update;

use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};

#[derive(clap::Subcommand)]
pub enum ShareCommands {
    /// List my share links
    List {
        /// Page size
        #[clap(long, default_value = "50")]
        page_size: Option<u32>,

        /// Order by field
        #[clap(long)]
        order_by: Option<String>,
    },

    /// Create a new share link
    Create {
        /// File URI to share
        #[clap(long)]
        uri: String,

        /// Share link name
        #[clap(long)]
        name: Option<String>,

        /// Expiration time in seconds
        #[clap(long)]
        expire: Option<u32>,

        /// Password for the share link
        #[clap(long)]
        password: Option<String>,
    },

    /// Update a share link
    Update {
        /// Share ID
        #[clap(long)]
        id: String,

        /// New name
        #[clap(long)]
        name: Option<String>,

        /// New expiration time in seconds
        #[clap(long)]
        expire: Option<u32>,

        /// New password
        #[clap(long)]
        password: Option<String>,
    },

    /// Delete a share link
    Delete {
        /// Share ID
        #[clap(long)]
        id: String,
    },
}

pub async fn handle_share_command(
    api: &CloudreveAPI,
    command: ShareCommands,
) -> Result<()> {
    // For now, use V4 client through inner()
    match api.inner() {
        UnifiedClient::V4(client) => match command {
            ShareCommands::List { page_size, order_by } => {
                list::handle_list(client, page_size, order_by).await
            }
            ShareCommands::Create {
                uri,
                name,
                expire,
                password,
            } => create::handle_create(client, uri, name, expire, password).await,
            ShareCommands::Update {
                id,
                name,
                expire,
                password,
            } => update::handle_update(client, id, name, expire, password).await,
            ShareCommands::Delete { id } => delete::handle_delete(client, id).await,
        },
        UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Share commands not yet supported for V3 API".to_string())),
    }
}
