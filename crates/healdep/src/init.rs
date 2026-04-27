use anyhow::Result;
use console::style;
use std::{fs, path::Path};

const BADGE_URL: &str = "[![HealDep Status](https://img.shields.io/endpoint?url=https://healdep.example.com/api/badge/ТВОЙ_АККАУНТ/ТВОЙ_РЕПО)](https://github.com/healdep/healdep)";

pub fn run(project_dir: &str) -> Result<()> {
    let dir = Path::new(project_dir);
    // Создаём healdep.toml с базовыми настройками, если его нет
    let config_path = dir.join("healdep.toml");
    if !config_path.exists() {
        fs::write(&config_path, "[ai]\nprovider = \"ollama\"\n")?;
        println!(
            "{}",
            style("✅ Создан healdep.toml с базовой конфигурацией AI.").green()
        );
    } else {
        println!("{}", style("ℹ️  healdep.toml уже существует.").cyan());
    }

    // Добавляем .github/workflows/healdep-action.yml для CI
    let workflows_dir = dir.join(".github/workflows");
    fs::create_dir_all(&workflows_dir)?;
    let action_path = workflows_dir.join("healdep-action.yml");
    if !action_path.exists() {
        fs::write(
            &action_path,
            include_str!("../../../.github/workflows/healdep-action.yml"),
        )?;
        println!(
            "{}",
            style("✅ Добавлен GitHub Action в .github/workflows/healdep-action.yml").green()
        );
    }

    // Предлагаем вставить бейдж в README
    let readme_path = dir.join("README.md");
    if readme_path.exists() {
        println!(
            "{}",
            style("💡 Добавьте этот бейдж в ваш README.md:").yellow()
        );
        println!("{}", BADGE_URL);
    } else {
        println!(
            "{}",
            style("💡 Создайте README.md с этим бейджем:").yellow()
        );
        println!("{}", BADGE_URL);
    }
    Ok(())
}
