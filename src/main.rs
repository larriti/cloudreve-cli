use clap::{Parser, Subcommand};
use cloudreve_api::{CloudreveClient, Result};
use log::error;

mod commands;
mod context;

// Local logging initialization (CLI-specific)
fn init_logging_with_level(level: &str) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level)).init();
}

#[derive(Parser)]
#[clap(name = "cloudreve-cli", version = "0.1.0", author = "Cloudreve CLI")]
/// A command-line interface for Cloudreve API
struct Cli {
    /// Cloudreve instance URL
    #[clap(short, long)]
    url: Option<String>,

    /// Cloudreve login email (optional, will use default if not provided)
    #[clap(short, long)]
    email: Option<String>,

    /// Authentication token
    #[clap(short, long)]
    token: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[clap(long, default_value = "info")]
    log_level: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with Cloudreve
    Auth {
        /// Password
        #[clap(short, long)]
        password: Option<String>,
    },

    /// File management
    File {
        #[clap(subcommand)]
        command: commands::file::FileCommands,
    },

    /// User management
    User {
        #[clap(subcommand)]
        command: commands::user::UserCommands,
    },

    /// Share management
    Share {
        #[clap(subcommand)]
        command: commands::share::ShareCommands,
    },

    /// Settings management
    Settings {
        #[clap(subcommand)]
        command: commands::settings::SettingsCommands,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger with the specified log level
    init_logging_with_level(&cli.log_level);

    // Unified client initialization via context module
    let ctx = context::initialize_client(context::ClientConfig {
        url: cli.url.clone(),
        email: cli.email.clone(),
        token: cli.token.clone(),
    }).await?;

    // Determine if this is an auth command
    let is_auth_command = matches!(cli.command, Commands::Auth { .. });

    // Get or create client
    let mut client = match ctx.client {
        Some(client) => client,
        None => {
            if is_auth_command {
                let url = cli.url.as_ref().expect("URL is required for authentication");
                CloudreveClient::new(url)
            } else {
                error!("No cached token found. Please authenticate first using the auth command.");
                std::process::exit(1);
            }
        }
    };

    // Command dispatch
    match cli.command {
        Commands::Auth { password } => {
            commands::auth::handle_auth(
                &mut client,
                &ctx.token_manager,
                cli.email,
                cli.url,
                password
            ).await?;
        }
        Commands::File { command } => {
            commands::file::handle_file_command(&client, command).await?;
        }
        Commands::User { command } => {
            commands::user::handle_user_command(&client, &ctx.token_manager, command).await?;
        }
        Commands::Share { command } => {
            commands::share::handle_share_command(&client, command).await?;
        }
        Commands::Settings { command } => {
            commands::settings::handle_settings_command(&client, command).await?;
        }
    }

    Ok(())
}
