// 文件命令模块的导出文件

pub mod batch;
pub mod copy;
pub mod delete;
pub mod diff;
pub mod download;
pub mod info;
pub mod list;
pub mod metadata;
pub mod mkdir;
pub mod move_cmd;
pub mod permission;
pub mod preview;
pub mod rename;
pub mod restore;
pub mod search;
pub mod share;
pub mod sync;
pub mod upload;

use cloudreve_api::api::v4::models::FileType;
use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};

#[derive(clap::Subcommand)]
pub enum FileCommands {
    /// List files in a directory
    List {
        /// Path to list files in
        #[clap(short, long, default_value = "/")]
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
        /// File path
        #[clap(short, long)]
        path: String,

        /// Include extended information
        #[clap(short, long)]
        extended: bool,
    },

    /// Upload a file
    Upload {
        /// Local file path
        #[clap(short, long)]
        file: String,

        /// Destination path
        #[clap(short, long, default_value = "/")]
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
        /// File path
        #[clap(short, long)]
        path: String,

        /// Local destination path
        #[clap(short, long)]
        output: String,

        /// Download URL expiration time in seconds
        #[clap(long)]
        expires_in: Option<u32>,
    },

    /// Delete files
    Delete {
        /// File path(s) to delete
        #[clap(short, long, required = true)]
        path: Vec<String>,

        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
    },

    /// Rename a file
    Rename {
        /// Source file path
        #[clap(short, long)]
        src: String,

        /// New name
        #[clap(short, long)]
        name: String,
    },

    /// Move files
    Move {
        /// Source file path(s)
        #[clap(short, long, required = true)]
        src: Vec<String>,

        /// Destination directory path
        #[clap(short, long, required = true)]
        dest: String,
    },

    /// Copy files
    Copy {
        /// Source file path(s)
        #[clap(short, long, required = true)]
        src: Vec<String>,

        /// Destination directory path
        #[clap(short, long, required = true)]
        dest: String,
    },

    /// Create a directory
    Mkdir {
        /// Directory path
        #[clap(short, long, required = true)]
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
        /// File path to share
        #[clap(short, long)]
        path: String,

        /// Share link name
        #[clap(short, long)]
        name: Option<String>,

        /// Expiration time in seconds
        #[clap(long)]
        expire: Option<u32>,

        /// Password for the share link
        #[clap(short('P'), long)]
        password: Option<String>,
    },

    /// Batch upload files
    BatchUpload {
        /// Local file(s) or directory path(s) (can be specified multiple times)
        #[clap(short, long, required = true, num_args = 1..)]
        paths: Vec<String>,

        /// Destination path
        #[clap(short, long, default_value = "/")]
        dest: String,

        /// Overwrite if exists
        #[clap(long)]
        overwrite: bool,

        /// Storage policy ID
        #[clap(short, long)]
        policy: Option<String>,

        /// Recursive upload for directories
        #[clap(short, long)]
        recursive: bool,
    },

    /// Batch download files
    BatchDownload {
        /// File URI(s) to download (can be specified multiple times)
        #[clap(short, short, long, required = true, num_args = 1..)]
        paths: Vec<String>,

        /// Local output directory
        #[clap(short, long, default_value = ".")]
        output: String,

        /// URL expiration time in seconds
        #[clap(long)]
        expires_in: Option<u32>,
    },

    /// Search files
    Search {
        /// Path to search in
        #[clap(short, long, default_value = "/")]
        path: String,

        /// Name pattern (case-insensitive substring)
        #[clap(short, long)]
        name: Option<String>,

        /// File type (file or folder)
        #[clap(short, long)]
        type_: Option<String>,

        /// Minimum size in bytes
        #[clap(long)]
        min_size: Option<i64>,

        /// Maximum size in bytes
        #[clap(long)]
        max_size: Option<i64>,

        /// File extension (e.g., "txt", "jpg")
        #[clap(short, long)]
        extension: Option<String>,

        /// Recursive search
        #[clap(short, long)]
        recursive: bool,
    },

    /// Sync files between local and remote
    Sync {
        /// Local path
        #[clap(long, required = true)]
        local: String,

        /// Remote path
        #[clap(long, required = true)]
        remote: String,

        /// Sync direction (up, down, both)
        #[clap(long, default_value = "up")]
        direction: String,

        /// Dry run (show what would be done without making changes)
        #[clap(long)]
        dry_run: bool,
    },

    /// Preview file content
    Preview {
        /// File URI to preview
        #[clap(long, required = true)]
        uri: String,

        /// Preview type (text, json, image)
        #[clap(long, default_value = "text")]
        type_: String,
    },

    /// Compare local and remote files
    Diff {
        /// Local file path
        #[clap(long, required = true)]
        local: String,

        /// Remote file URI
        #[clap(long, required = true)]
        remote: String,
    },
}

pub async fn handle_file_command(client: &CloudreveAPI, command: FileCommands) -> Result<()> {
    match command {
        FileCommands::List {
            path,
            page,
            page_size,
        } => list::handle_list(client, path, page, page_size).await,

        FileCommands::Info { path, extended } => info::handle_info(client, path, extended).await,

        FileCommands::Upload {
            file,
            path,
            overwrite,
            policy,
        } => upload::handle_upload(client, file, path, overwrite, policy).await,

        FileCommands::Download {
            path,
            output,
            expires_in,
        } => download::handle_download(client, path, output, expires_in).await,

        FileCommands::Delete { path, force } => delete::handle_delete(client, path, force).await,

        FileCommands::Rename { src, name } => rename::handle_rename(client, src, name).await,

        FileCommands::Move { src, dest } => move_cmd::handle_move(client, src, dest).await,

        FileCommands::Copy { src, dest } => copy::handle_copy(client, src, dest).await,

        FileCommands::Mkdir { path } => mkdir::handle_mkdir(client, path).await,

        FileCommands::Restore { uri } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => restore::handle_restore(v4_client, uri).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Restore not yet supported for V3 API".to_string(),
                )),
            }
        }

        FileCommands::Permission { command } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => {
                    permission::handle_permission(v4_client, command).await
                }
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Permission not yet supported for V3 API".to_string(),
                )),
            }
        }

        FileCommands::Metadata { command } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => metadata::handle_metadata(v4_client, command).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Metadata not yet supported for V3 API".to_string(),
                )),
            }
        }

        FileCommands::Share {
            path,
            name,
            expire,
            password,
        } => share::handle_share(client, path, name, expire, password).await,

        FileCommands::BatchUpload {
            paths,
            dest,
            overwrite,
            policy,
            recursive,
        } => batch::handle_batch_upload(client, paths, dest, overwrite, policy, recursive).await,

        FileCommands::BatchDownload {
            paths,
            output,
            expires_in,
        } => batch::handle_batch_download(client, paths, output, expires_in).await,

        FileCommands::Search {
            path,
            name,
            type_,
            min_size,
            max_size,
            extension,
            recursive,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => {
                    let filter = search::SearchFilter {
                        name_pattern: name,
                        file_type: type_.and_then(|t| match t.as_str() {
                            "file" => Some(FileType::File),
                            "folder" => Some(FileType::Folder),
                            _ => None,
                        }),
                        min_size,
                        max_size,
                        extension,
                    };
                    search::handle_search(v4_client, path, filter, recursive).await
                }
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Search not yet supported for V3 API".to_string(),
                )),
            }
        }

        FileCommands::Sync {
            local,
            remote,
            direction,
            dry_run,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => {
                    sync::handle_sync(v4_client, local, remote, direction, dry_run).await
                }
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Sync not yet supported for V3 API".to_string(),
                )),
            }
        }

        FileCommands::Preview { uri, type_ } => preview::handle_preview(client, uri, type_).await,

        FileCommands::Diff { local, remote } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => diff::handle_diff(v4_client, local, remote).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
                    "Diff not yet supported for V3 API".to_string(),
                )),
            }
        }
    }
}
