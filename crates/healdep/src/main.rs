mod cli;
mod analyze;
mod heal;
mod ui;
mod config;
mod ai;
mod registry;
mod init;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::print_banner();
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { project_dir } => init::run(&project_dir)?,
        Commands::Analyze { manifest_path } => analyze::run(&manifest_path)?,
        Commands::Heal { manifest_path, crate_name, use_ai } => {
            heal::run(&manifest_path, crate_name.as_deref(), use_ai).await?
        },
    }
    Ok(())
}


