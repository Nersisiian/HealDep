use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "healdep", about = "Self-healing package manager with AI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        project_dir: String,
    },
    Analyze {
        #[arg(default_value = "./Cargo.toml")]
        manifest_path: String,
    },
    Heal {
        #[arg(default_value = "./Cargo.toml")]
        manifest_path: String,
        #[arg(short, long)]
        crate_name: Option<String>,
        /// ???????????????? AI-?????????????????? ???????????????? (?????????????? healdep.toml)
        #[arg(short = 'a', long = "ai")]
        use_ai: bool,
    },
}
