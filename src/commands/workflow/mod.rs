// Workflow 命令模块

pub mod archive;
pub mod cancel;
pub mod download;
pub mod extract;
pub mod import;
pub mod list;
pub mod progress;
pub mod relocate;

use cloudreve_api::{CloudreveAPI, Result, UnifiedClient};
use log::debug;

#[derive(clap::Subcommand)]
pub enum WorkflowCommands {
    /// List workflow tasks
    List {
        /// Category of tasks (general, downloading, downloaded)
        #[clap(long, default_value = "general")]
        category: String,

        /// Items per page (10-100)
        #[clap(long, default_value = "25")]
        per_page: String,
    },

    /// Get task progress
    Progress {
        /// Task ID
        #[clap(short, long, required = true)]
        task: String,
    },

    /// Cancel a task
    Cancel {
        /// Task ID
        #[clap(short, long, required = true)]
        task: String,
    },

    /// Create an archive from files
    Archive {
        /// File path(s) to archive (comma-separated)
        #[clap(short, long, required = true)]
        files: String,

        /// Archive name
        #[clap(short('N'), long, required = true)]
        name: String,

        /// Destination path
        #[clap(short, long)]
        path: Option<String>,
    },

    /// Extract an archive
    Extract {
        /// Archive file path
        #[clap(short, long, required = true)]
        archive: String,

        /// Destination path
        #[clap(short, long)]
        path: Option<String>,
    },

    /// Relocate files to different storage policy
    Relocate {
        /// File path(s) to relocate (comma-separated)
        #[clap(short, long, required = true)]
        files: String,

        /// Target storage policy ID
        #[clap(long, required = true)]
        policy: String,
    },

    /// Import files from external storage (admin only)
    Import {
        /// Source path in external storage
        #[clap(short, long, required = true)]
        src: String,

        /// Target path
        #[clap(short, long, required = true)]
        dst: String,

        /// User ID to import files for (default: current user)
        #[clap(long)]
        user_id: Option<String>,

        /// Storage policy ID (default: policy from target directory)
        #[clap(long, default_value = "1")]
        policy_id: Option<i32>,

        /// Extract media metadata after import
        #[clap(long)]
        extract_media_meta: bool,

        /// Recursively import child folders
        #[clap(long)]
        recursive: bool,
    },

    /// Download task management
    Download {
        #[clap(subcommand)]
        command: download::DownloadCommands,
    },
}

pub async fn handle_workflow_command(
    api: &CloudreveAPI,
    command: WorkflowCommands,
) -> Result<()> {
    match api.inner() {
        UnifiedClient::V4(client) => match command {
            WorkflowCommands::List {
                category,
                per_page,
            } => list::handle_list(client, category, per_page).await,

            WorkflowCommands::Progress { task } => progress::handle_progress(client, task).await,

            WorkflowCommands::Cancel { task } => cancel::handle_cancel(client, task).await,

            WorkflowCommands::Archive { files, name, path } => {
                archive::handle_archive(client, files, name, path).await
            }

            WorkflowCommands::Extract { archive, path } => {
                extract::handle_extract(client, archive, path).await
            }

            WorkflowCommands::Relocate { files, policy } => {
                relocate::handle_relocate(client, files, policy, None).await
            }

            WorkflowCommands::Import {
                src,
                dst,
                user_id,
                policy_id,
                extract_media_meta,
                recursive,
            } => {
                // Get default user_id from token if not specified
                let final_user_id = if user_id.is_none() {
                    match crate::context::token_manager::TokenManager::new()
                        .and_then(|tm| tm.get_token_by_url(api.base_url()))
                    {
                        Ok(Some(token_info)) => {
                            debug!("Using user_id from token: {}", token_info.user_id);
                            Some(token_info.user_id)
                        }
                        _ => None,
                    }
                } else {
                    user_id
                };

                import::handle_import(client, src, dst, final_user_id, policy_id, extract_media_meta, recursive)
                    .await
            }

            WorkflowCommands::Download { command } => {
                download::handle_download_command(client, command).await
            }
        },
        UnifiedClient::V3(_) => Err(cloudreve_api::Error::InvalidResponse(
            "Workflow 命令仅支持 V4 API".to_string(),
        )),
    }
}
