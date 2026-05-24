use cloudreve_api::Result;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::api::v4::models::{CreateDownloadRequest, SelectDownloadFilesRequest};
use cloudreve_api::api::v4::uri::path_to_uri;
use log::info;

#[derive(clap::Subcommand)]
pub enum DownloadCommands {
    /// Create a download task
    Create {
        /// URL to download
        #[clap(short, long, required = true)]
        url: String,

        /// Destination path
        #[clap(short, long)]
        path: Option<String>,

        /// Node ID
        #[clap(short, long)]
        node: Option<u64>,
    },

    /// Select files for download
    Select {
        /// Task ID
        #[clap(short, long, required = true)]
        task: String,

        /// Files to select (comma-separated)
        #[clap(short, long, required = true)]
        files: String,
    },

    /// Cancel a download task
    Cancel {
        /// Task ID
        #[clap(short, long, required = true)]
        task: String,
    },
}

pub async fn handle_download_command(
    client: &ApiV4Client,
    command: DownloadCommands,
) -> Result<()> {
    match command {
        DownloadCommands::Create { url, path, node } => {
            handle_create(client, url, path, node).await
        }
        DownloadCommands::Select { task, files } => handle_select(client, task, files).await,
        DownloadCommands::Cancel { task } => handle_cancel(client, task).await,
    }
}

async fn handle_create(
    client: &ApiV4Client,
    url: String,
    path: Option<String>,
    node: Option<u64>,
) -> Result<()> {
    info!("Creating download task: {}", url);

    // 使用 path_to_uri 转换路径
    let dst = path.as_deref().unwrap_or("/");

    let request = CreateDownloadRequest {
        dst: &path_to_uri(dst),
        src: vec![&url],
        preferred_node_id: node.map(|n| n.to_string()),
    };

    let tasks = client.create_download(&request).await?;

    info!("");
    if let Some(task) = tasks.first() {
        info!("✅ Download task created successfully");
        info!("  Task ID: {}", task.id);
        if let Some(name) = &task.name {
            info!("  Name: {}", name);
        }
        info!("  Status: {}", task.status);
        info!("  URL: {}", url);
        info!("  Destination: {}", dst);
        info!("  Created: {}", task.created_at);
        info!("");
        info!("💡 To cancel this task, run:");
        info!("   cloudreve-cli workflow download cancel -t {}", task.id);
        info!("   cloudreve-cli workflow cancel -t {}", task.id);
    } else {
        info!("⚠️  No task returned");
    }

    Ok(())
}

async fn handle_select(client: &ApiV4Client, task_id: String, files: String) -> Result<()> {
    info!("Selecting files for task {}: {}", task_id, files);

    let files_vec: Vec<&str> = files.split(',').map(|s| s.trim()).collect();

    let request = SelectDownloadFilesRequest {
        selected_files: files_vec.iter().map(|s| s.as_ref()).collect(),
    };

    let tasks = client.select_download_files(&task_id, &request).await?;

    info!("");
    if let Some(task) = tasks.first() {
        info!("✅ Files selected successfully");
        info!("  Task ID: {}", task.id);
        if let Some(name) = &task.name {
            info!("  Name: {}", name);
        }
        info!("  Status: {}", task.status);
        info!("  Updated: {}", task.updated_at);
        info!("");
        info!("💡 To cancel this task, run:");
        info!("   cloudreve-cli workflow download cancel -t {}", task.id);
        info!("   cloudreve-cli workflow cancel -t {}", task.id);
    } else {
        info!("⚠️  No task returned");
    }

    Ok(())
}

async fn handle_cancel(client: &ApiV4Client, task_id: String) -> Result<()> {
    info!("Canceling download task: {}", task_id);

    client.cancel_download_task(&task_id).await?;

    info!("");
    info!("✅ Download task {} canceled successfully", task_id);

    Ok(())
}
