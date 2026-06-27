use clap::{Parser, Subcommand};
use llm_proxy::cli::Cli as LlmProxyCli;
use webdav_backup::cli::Cli as WebdavBackupCli;
#[derive(Parser)]
#[command(version, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(disable_help_flag = true)]
    WebdavBackup {
        #[arg(allow_hyphen_values = true)]
        args: Option<Vec<String>>,
    },
    #[command(disable_help_flag = true)]
    LLMProxy {
        #[arg(allow_hyphen_values = true)]
        args: Option<Vec<String>>,
    },
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Command::WebdavBackup { args } => {
            let cli = get_subcommand::<WebdavBackupCli>("webdav-backup", &args.unwrap_or_default());
            if let Err(e) = cli.run().await {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Command::LLMProxy { args } => {
            let cli = get_subcommand::<LlmProxyCli>("llm-proxy", &args.unwrap_or_default());
            if let Err(e) = cli.run().await {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}

fn get_subcommand<T: Parser>(name: &str, args: &[String]) -> T {
    let full_args = std::iter::once(name).chain(args.iter().map(std::string::String::as_str));

    T::parse_from(full_args)
}
