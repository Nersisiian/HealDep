use anyhow::Result;
use console::style;
use std::{fs, path::Path};

const BADGE_URL: &str = "[![HealDep Status](https://img.shields.io/endpoint?url=https://healdep.example.com/api/badge/YOUR_ACCOUNT/YOUR_REPO)](https://github.com/healdep/healdep)";

pub fn run(project_dir: &str) -> Result<()> {
    let dir = Path::new(project_dir);
    let config_path = dir.join("healdep.toml");
    if !config_path.exists() {
        fs::write(&config_path, "[ai]\nprovider = \"ollama\"\n")?;
        println!(
            "{}",
            style("Created healdep.toml with basic AI config.").green()
        );
    } else {
        println!("{}", style("healdep.toml already exists.").cyan());
    }

    let workflows_dir = dir.join(".github/workflows");
    fs::create_dir_all(&workflows_dir)?;
    let action_path = workflows_dir.join("healdep-action.yml");
    if !action_path.exists() {
        fs::write(&action_path, "name: Auto-Heal\non: [push]\njobs:\n  heal:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo HealDep\n")?;
        println!(
            "{}",
            style("Added GitHub Action in .github/workflows/healdep-action.yml").green()
        );
    }

    let readme_path = dir.join("README.md");
    if readme_path.exists() {
        println!("{}", style("Add this badge to your README.md:").yellow());
        println!("{}", BADGE_URL);
    } else {
        println!(
            "{}",
            style("Create a README.md and add this badge:").yellow()
        );
        println!("{}", BADGE_URL);
    }
    Ok(())
}
