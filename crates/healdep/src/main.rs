mod ai;
mod analyze;
mod cli;
mod config;
mod heal;
mod init;
mod registry;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::print_banner();
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { project_dir } => init::run(&project_dir)?,
        Commands::Analyze { manifest_path } => analyze::run(&manifest_path)?,
        Commands::Heal {
            manifest_path,
            crate_name,
            use_ai,
        } => heal::run(&manifest_path, crate_name.as_deref(), use_ai).await?,
    }
    Ok(())
}
