use crate::ui;
use anyhow::Context;
use cargo_metadata::MetadataCommand;
use console::style;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct Conflict {
    pub crate_name: String,
    pub versions: Vec<String>,
}

pub fn run(manifest_path: &str) -> anyhow::Result<()> {
    let spin = ui::spinner("🔄 Анализирую граф зависимостей...");
    let metadata = MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()
        .context("Не удалось выполнить cargo metadata")?;
    let conflicts = detect_conflicts(&metadata);
    ui::finish_spinner(&spin, true, "Анализ завершён");
    if conflicts.is_empty() {
        println!(
            "{}",
            style("✅ Конфликтов не обнаружено. Ваш проект здоров!").green()
        );
    } else {
        println!(
            "{}",
            style("⚠️  Обнаружены конфликтующие версии:")
                .yellow()
                .bold()
        );
        ui::separator();
        for (i, c) in conflicts.iter().enumerate() {
            let name = style(&c.crate_name).cyan().bold();
            let versions = c
                .versions
                .iter()
                .map(|v| style(v).red().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("  {}. {} : {}", i + 1, name, versions);
        }
        ui::separator();
        println!(
            "\n{}",
            style("💡 Запустите 'healdep heal', чтобы автоматически исправить их.").dim()
        );
    }
    Ok(())
}

pub fn detect_conflicts(metadata: &cargo_metadata::Metadata) -> Vec<Conflict> {
    let mut name_to_versions: HashMap<String, Vec<String>> = HashMap::new();
    for pkg in &metadata.packages {
        name_to_versions
            .entry(pkg.name.clone())
            .or_default()
            .push(pkg.version.to_string());
    }
    let mut conflicts = Vec::new();
    for (name, versions) in name_to_versions {
        let unique: std::collections::HashSet<_> = versions.iter().collect();
        if unique.len() > 1 {
            conflicts.push(Conflict {
                crate_name: name,
                versions: unique.into_iter().cloned().collect(),
            });
        }
    }
    conflicts
}
