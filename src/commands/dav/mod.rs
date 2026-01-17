// WebDAV account management commands

pub mod create;
pub mod delete;
pub mod list;
pub mod update;

use cloudreve_api::{CloudreveAPI, Result};

#[derive(clap::Subcommand)]
pub enum DavCommands {
    /// List all WebDAV accounts
    List {
        /// Items per page
        #[clap(long, default_value = "50")]
        page_size: u32,
    },

    /// Create a new WebDAV account
    Create {
        /// Root folder path (e.g., /folder)
        #[clap(long)]
        uri: String,

        /// Account name/annotation
        #[clap(long)]
        name: String,

        /// Readonly account
        #[clap(long)]
        readonly: bool,

        /// Enable proxy
        #[clap(long)]
        proxy: bool,
    },

    /// Update a WebDAV account
    Update {
        /// Account ID
        id: String,

        /// Root folder path
        #[clap(long)]
        uri: Option<String>,

        /// Account name
        #[clap(long)]
        name: Option<String>,

        /// Readonly account
        #[clap(long)]
        readonly: Option<bool>,

        /// Enable proxy
        #[clap(long)]
        proxy: Option<bool>,
    },

    /// Delete a WebDAV account
    Delete {
        /// Account ID
        id: String,
    },
}

pub async fn handle_dav_command(
    api: &CloudreveAPI,
    command: DavCommands,
) -> Result<()> {
    match command {
        DavCommands::List { page_size } => {
            list::handle_list(api, page_size).await
        }
        DavCommands::Create {
            uri,
            name,
            readonly,
            proxy,
        } => {
            create::handle_create(api, uri, name, readonly, proxy).await
        }
        DavCommands::Update {
            id,
            uri,
            name,
            readonly,
            proxy,
        } => {
            update::handle_update(api, id, uri, name, readonly, proxy).await
        }
        DavCommands::Delete { id } => {
            delete::handle_delete(api, id).await
        }
    }
}
