use anyhow::Result;
use console::style;
use std::{fs, path::Path};

const BADGE_URL: &str = "[![HealDep Status](https://img.shields.io/endpoint?url=https://healdep.example.com/api/badge/РўР’РћР™_РђРљРљРђРЈРќРў/РўР’РћР™_Р Р•РџРћ)](https://github.com/healdep/healdep)";

pub fn run(project_dir: &str) -> Result<()> {
    let dir = Path::new(project_dir);
    // РЎРѕР·РґР°С‘Рј healdep.toml СЃ Р±Р°Р·РѕРІС‹РјРё РЅР°СЃС‚СЂРѕР№РєР°РјРё, РµСЃР»Рё РµРіРѕ РЅРµС‚
    let config_path = dir.join("healdep.toml");
    if !config_path.exists() {
        fs::write(&config_path, "[ai]\nprovider = \"ollama\"\n")?;
        println!(
            "{}",
            style("вњ… РЎРѕР·РґР°РЅ healdep.toml СЃ Р±Р°Р·РѕРІРѕР№ РєРѕРЅС„РёРіСѓСЂР°С†РёРµР№ AI.").green()
        );
    } else {
        println!("{}", style("в„№пёЏ  healdep.toml СѓР¶Рµ СЃСѓС‰РµСЃС‚РІСѓРµС‚.").cyan());
    }

    // Р”РѕР±Р°РІР»СЏРµРј .github/workflows/healdep-action.yml РґР»СЏ CI
    let workflows_dir = dir.join(".github/workflows");
    fs::create_dir_all(&workflows_dir)?;
    let action_path = workflows_dir.join("healdep-action.yml");
    if !action_path.exists() {
        fs::write(
            &action_path,
            "name: Auto-Heal with HealDep
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
jobs:
  heal:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.86.0
        override: true
    - name: Install HealDep
      run: cargo install healdep || true
    - name: Run HealDep for Cargo
      if: hashFiles('Cargo.toml') != ''
      run: healdep heal --ai || true
    - name: Run HealDep for Python
      if: hashFiles('requirements.txt') != ''
      run: python python/healdep_python.py heal requirements.txt --ai || true
    - name: Commit healed files
      uses: stefanzweifel/git-auto-commit-action@v4
      with:
        commit_message: "🤖 HealDep: автоматическое лечение зависимостей"
        file_pattern: 'healed_shims/** healed_shims_py/** healed_shims_npm/**'",
        )?;
        println!(
            "{}",
            style("вњ… Р”РѕР±Р°РІР»РµРЅ GitHub Action РІ .github/workflows/healdep-action.yml").green()
        );
    }

    // РџСЂРµРґР»Р°РіР°РµРј РІСЃС‚Р°РІРёС‚СЊ Р±РµР№РґР¶ РІ README
    let readme_path = dir.join("README.md");
    if readme_path.exists() {
        println!(
            "{}",
            style("рџ’Ў Р”РѕР±Р°РІСЊС‚Рµ СЌС‚РѕС‚ Р±РµР№РґР¶ РІ РІР°С€ README.md:").yellow()
        );
        println!("{}", BADGE_URL);
    } else {
        println!(
            "{}",
            style("рџ’Ў РЎРѕР·РґР°Р№С‚Рµ README.md СЃ СЌС‚РёРј Р±РµР№РґР¶РµРј:").yellow()
        );
        println!("{}", BADGE_URL);
    }
    Ok(())
}

