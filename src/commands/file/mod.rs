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

    /// Batch upload files
    BatchUpload {
        /// Local file(s) or directory path(s) (can be specified multiple times)
        #[clap(long, required = true, num_args = 1..)]
        paths: Vec<String>,

        /// Destination path
        #[clap(long, default_value = "/")]
        dest: String,

        /// Overwrite if exists
        #[clap(long)]
        overwrite: bool,

        /// Storage policy ID
        #[clap(long)]
        policy: Option<String>,

        /// Recursive upload for directories
        #[clap(long)]
        recursive: bool,
    },

    /// Batch download files
    BatchDownload {
        /// File URI(s) to download (can be specified multiple times)
        #[clap(long, required = true, num_args = 1..)]
        uris: Vec<String>,

        /// Local output directory
        #[clap(long, default_value = ".")]
        output: String,

        /// URL expiration time in seconds
        #[clap(long)]
        expires_in: Option<u32>,
    },

    /// Search files
    Search {
        /// Path to search in
        #[clap(long, default_value = "/")]
        path: String,

        /// Name pattern (case-insensitive substring)
        #[clap(long)]
        name: Option<String>,

        /// File type (file or folder)
        #[clap(long)]
        type_: Option<String>,

        /// Minimum size in bytes
        #[clap(long)]
        min_size: Option<i64>,

        /// Maximum size in bytes
        #[clap(long)]
        max_size: Option<i64>,

        /// File extension (e.g., "txt", "jpg")
        #[clap(long)]
        extension: Option<String>,

        /// Recursive search
        #[clap(long)]
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

        FileCommands::Info { uri, extended } => info::handle_info(client, uri, extended).await,

        FileCommands::Upload {
            file,
            path,
            overwrite,
            policy,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => upload::handle_upload(v4_client, file, path, overwrite, policy).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Upload not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Download {
            uri,
            output,
            expires_in,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => download::handle_download(v4_client, uri, output, expires_in).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Download not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Delete { uri, force } => delete::handle_delete(client, uri, force).await,

        FileCommands::Rename { src, name } => rename::handle_rename(client, src, name).await,

        FileCommands::Move { src, dest } => move_cmd::handle_move(client, src, dest).await,

        FileCommands::Copy { src, dest } => copy::handle_copy(client, src, dest).await,

        FileCommands::Mkdir { path } => mkdir::handle_mkdir(client, path).await,

        FileCommands::Restore { uri } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => restore::handle_restore(v4_client, uri).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Restore not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Permission { command } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => permission::handle_permission(v4_client, command).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Permission not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Metadata { command } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => metadata::handle_metadata(v4_client, command).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Metadata not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Share {
            uri,
            name,
            expire,
            password,
        } => share::handle_share(client, uri, name, expire, password).await,

        FileCommands::BatchUpload {
            paths,
            dest,
            overwrite,
            policy,
            recursive,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => batch::handle_batch_upload(v4_client, paths, dest, overwrite, policy, recursive).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Batch upload not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::BatchDownload {
            uris,
            output,
            expires_in,
        } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => batch::handle_batch_download(v4_client, uris, output, expires_in).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Batch download not yet supported for V3 API".to_string())),
            }
        }

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
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Search not yet supported for V3 API".to_string())),
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
                UnifiedClient::V4(v4_client) => sync::handle_sync(v4_client, local, remote, direction, dry_run).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Sync not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Preview { uri, type_ } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => preview::handle_preview(v4_client, uri, type_).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Preview not yet supported for V3 API".to_string())),
            }
        }

        FileCommands::Diff { local, remote } => {
            // Use V4 client for now (not yet migrated to CloudreveAPI)
            match client.inner() {
                UnifiedClient::V4(v4_client) => diff::handle_diff(v4_client, local, remote).await,
                UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse("Diff not yet supported for V3 API".to_string())),
            }
        }
    }
}
