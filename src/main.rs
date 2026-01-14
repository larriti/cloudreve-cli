use clap::{Parser, Subcommand};
use cloudreve_api::{CloudreveClient, Result};
use log::error;
use std::sync::atomic::{AtomicBool, Ordering};

mod commands;
mod config;
mod context;
mod utils;

// Global flag to control log prefix display
static LOG_PREFIX: AtomicBool = AtomicBool::new(false);

// Local logging initialization (CLI-specific)
fn init_logging_with_level(level: &str, show_prefix: bool) {
    // Set global flag
    LOG_PREFIX.store(show_prefix, Ordering::SeqCst);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level))
        .format(|buf, record| {
            use std::io::Write;
            if LOG_PREFIX.load(Ordering::SeqCst) {
                // Full format: timestamp + level + module path
                writeln!(buf, "[{} {}{}] {}",
                    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
                    record.level(),
                    record.module_path().map(|m| format!(" {}", m)).unwrap_or_default(),
                    record.args())
            } else {
                // Clean format: message only
                writeln!(buf, "{}", record.args())
            }
        })
        .init();
}

#[derive(Parser)]
#[clap(name = "cloudreve-cli", version = env!("CARGO_PKG_VERSION"), author = "Cloudreve CLI")]
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

    /// Show full log prefix (timestamp, level, module path)
    #[clap(long)]
    log_prefix: bool,

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
    },

    /// Generate shell completion script
    Completions {
        /// Shell type (bash, zsh, fish, elvish, powershell)
        #[clap(long)]
        shell: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration file
    let cfg = config::Config::load().unwrap_or_default();

    // Parse command line arguments
    let cli = Cli::parse();

    // Use config values as defaults if not provided via command line
    let url = cli.url.or_else(|| cfg.default_url.clone());
    let email = cli.email.or_else(|| cfg.default_email.clone());
    let log_level = if cli.log_level == "info" {
        // Use config log_level if default wasn't overridden
        cfg.log_level.clone().unwrap_or_else(|| "info".to_string())
    } else {
        cli.log_level
    };

    // Initialize logger with the specified log level and prefix setting
    init_logging_with_level(&log_level, cli.log_prefix);

    // Unified client initialization via context module
    let ctx = context::initialize_client(context::ClientConfig {
        url: url.clone(),
        email: email.clone(),
        token: cli.token.clone(),
    }).await?;

    // Determine if this is an auth command
    let is_auth_command = matches!(cli.command, Commands::Auth { .. });

    // Get or create client
    let mut client = match ctx.client {
        Some(client) => client,
        None => {
            if is_auth_command {
                let url_val = match url.as_ref() {
                    Some(u) => u,
                    None => {
                        error!("URL is required for authentication. Please provide it with --url option or set default_url in config file.");
                        std::process::exit(1);
                    }
                };
                CloudreveClient::new(url_val)
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
                email,
                url,
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
        Commands::Completions { shell } => {
            generate_completions(&shell);
            return Ok(());
        }
    }

    Ok(())
}

fn generate_completions(shell: &str) {
    use clap::CommandFactory;
    use std::io;

    let mut app = Cli::command();

    match shell {
        "bash" => {
            clap_complete::generate(clap_complete::shells::Bash, &mut app, "cloudreve-cli", &mut io::stdout());
        }
        "zsh" => {
            clap_complete::generate(clap_complete::shells::Zsh, &mut app, "cloudreve-cli", &mut io::stdout());
        }
        "fish" => {
            clap_complete::generate(clap_complete::shells::Fish, &mut app, "cloudreve-cli", &mut io::stdout());
        }
        "elvish" => {
            clap_complete::generate(clap_complete::shells::Elvish, &mut app, "cloudreve-cli", &mut io::stdout());
        }
        "powershell" | "pwsh" => {
            clap_complete::generate(clap_complete::shells::PowerShell, &mut app, "cloudreve-cli", &mut io::stdout());
        }
        _ => {
            error!("Unsupported shell: {}", shell);
            error!("Supported shells: bash, zsh, fish, elvish, powershell");
            std::process::exit(1);
        }
    }
}
