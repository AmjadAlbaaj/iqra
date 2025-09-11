use anyhow::Result;
use clap::{Parser, Subcommand};
use iqra::cli::{run_file, run_repl};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "iqra")]
#[command(about = "Iqra - Arabic-first scripting language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive REPL
    Repl,
    /// Run a script file
    Run {
        /// Path to the script file
        file: String,
    },
    /// Run code directly from command line
    Code {
        /// Code to execute
        #[arg(short, long)]
        code: String,
    },
}

fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();

    match cli.command {
        Commands::Repl => run_repl()?,
        Commands::Run { file } => run_file(&file)?,
        Commands::Code { code } => {
            use iqra::lang::runtime::Runtime;
            let mut runtime = Runtime::new();
            let result = runtime.execute(&code)?;
            if !result.is_nil() {
                println!("{}", result);
            }
        }
    }

    Ok(())
}
