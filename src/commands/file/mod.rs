// 文件命令模块的导出文件

pub mod copy;
pub mod delete;
pub mod download;
pub mod info;
pub mod list;
pub mod metadata;
pub mod mkdir;
pub mod move_cmd;
pub mod permission;
pub mod rename;
pub mod restore;
pub mod share;
pub mod upload;

use cloudreve_api::{CloudreveClient, Result};

#[derive(clap::Subcommand)]
pub enum FileCommands {
    /// List files in a directory
    List {
        /// Path to list files in
        #[clap(long, default_value = "/")]
        path: String,

        /// Page number
        #[clap(long, default_value = "0")]
        page: Option<u32>,

        /// Items per page
        #[clap(long, default_value = "50")]
        page_size: Option<u32>,
    },

    /// Get file information
    Info {
        /// File URI
        #[clap(long)]
        uri: String,

        /// Include extended information
        #[clap(long)]
        extended: bool,
    },

    /// Upload a file
    Upload {
        /// Local file path
        #[clap(long)]
        file: String,

        /// Destination path
        #[clap(long, default_value = "/")]
        path: String,

        /// Overwrite if file exists
        #[clap(long)]
        overwrite: bool,

        /// Storage policy ID (uses first available if not specified)
        #[clap(long)]
        policy: Option<String>,
    },

    /// Download a file
    Download {
        /// File URI
        #[clap(long)]
        uri: String,

        /// Local destination path
        #[clap(long)]
        output: String,

        /// Download URL expiration time in seconds
        #[clap(long)]
        expires_in: Option<u32>,
    },

    /// Delete files
    Delete {
        /// File URI(s) to delete
        #[clap(long, required = true)]
        uri: Vec<String>,

        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
    },

    /// Rename a file
    Rename {
        /// Source file URI
        #[clap(long)]
        src: String,

        /// New name
        #[clap(long)]
        name: String,
    },

    /// Move files
    Move {
        /// Source file URI(s)
        #[clap(long, required = true)]
        src: Vec<String>,

        /// Destination directory URI
        #[clap(long, required = true)]
        dest: String,
    },

    /// Copy files
    Copy {
        /// Source file URI(s)
        #[clap(long, required = true)]
        src: Vec<String>,

        /// Destination directory URI
        #[clap(long, required = true)]
        dest: String,
    },

    /// Create a directory
    Mkdir {
        /// Directory path
        #[clap(long, required = true)]
        path: String,
    },

    /// Restore files from trash
    Restore {
        /// File URI(s) to restore
        #[clap(long, required = true)]
        uri: Vec<String>,
    },

    /// Permission management
    Permission {
        #[clap(subcommand)]
        command: permission::PermissionCommands,
    },

    /// Metadata management
    Metadata {
        #[clap(subcommand)]
        command: metadata::MetadataCommands,
    },

    /// Create a share link
    Share {
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
}

pub async fn handle_file_command(client: &CloudreveClient, command: FileCommands) -> Result<()> {
    match command {
        FileCommands::List {
            path,
            page,
            page_size,
        } => list::handle_list(client, path, page, page_size).await,

        FileCommands::Info { uri, extended } => info::handle_info(client, uri, extended).await,

        FileCommands::Upload {
            file,
            path,
            overwrite,
            policy,
        } => upload::handle_upload(client, file, path, overwrite, policy).await,

        FileCommands::Download {
            uri,
            output,
            expires_in,
        } => download::handle_download(client, uri, output, expires_in).await,

        FileCommands::Delete { uri, force } => delete::handle_delete(client, uri, force).await,

        FileCommands::Rename { src, name } => rename::handle_rename(client, src, name).await,

        FileCommands::Move { src, dest } => move_cmd::handle_move(client, src, dest).await,

        FileCommands::Copy { src, dest } => copy::handle_copy(client, src, dest).await,

        FileCommands::Mkdir { path } => mkdir::handle_mkdir(client, path).await,

        FileCommands::Restore { uri } => restore::handle_restore(client, uri).await,

        FileCommands::Permission { command } => {
            permission::handle_permission(client, command).await
        }

        FileCommands::Metadata { command } => metadata::handle_metadata(client, command).await,

        FileCommands::Share {
            uri,
            name,
            expire,
            password,
        } => share::handle_share(client, uri, name, expire, password).await,
    }
}
