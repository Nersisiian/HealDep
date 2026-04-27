use crate::analyze::{detect_conflicts, Conflict};
use crate::config::load_config;
use crate::registry::search_registry;
use crate::ui;
use anyhow::{Context, Result};
use console::style;
use healdep_sandbox::{run_tests, test_build};
use healdep_synthesizer::{generate_shim, generate_shim_ai, ConflictInfo};
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;

const MAX_AI_RETRIES: u32 = 3;

pub async fn run(manifest_path: &str, crate_name: Option<&str>, use_ai: bool) -> Result<()> {
    let path = Path::new(manifest_path);
    let project_dir = path.parent().unwrap_or(Path::new("."));
    let original_manifest = fs::read_to_string(path)?;
    let mut doc: toml::Value = toml::from_str(&original_manifest)?;

    let spin = ui::spinner("🔍 Сканирую конфликты...");
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(path)
        .exec()?;
    let conflicts = detect_conflicts(&metadata);
    ui::finish_spinner(&spin, true, "Сканирование завершено");

    let targets: Vec<&Conflict> = if let Some(name) = crate_name {
        conflicts.iter().filter(|c| c.crate_name == name).collect()
    } else {
        conflicts.iter().collect()
    };

    if targets.is_empty() {
        println!("{}", style("ℹ️  Нет конфликтов для лечения.").cyan());
        return Ok(());
    }

    let ai_config = if use_ai {
        Some(
            load_config()
                .context(
                    "Не удалось загрузить healdep.toml. Убедитесь, что файл существует и заполнен.",
                )?
                .ai
                .ok_or_else(|| anyhow::anyhow!("Секция [ai] отсутствует в healdep.toml"))?,
        )
    } else {
        None
    };

    println!("{}", style("🚑 Начинаю лечение...").bold().green());
    let healed_dir = project_dir.join("healed_shims");
    fs::create_dir_all(&healed_dir)?;

    let pb_total = ProgressBar::new(targets.len() as u64);
    pb_total.set_style(
        indicatif::ProgressStyle::with_template("[{bar:30.green/white}] {pos}/{len} крейтов")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    for conflict in &targets {
        pb_total.set_message(format!("🧬 Лечу {} …", conflict.crate_name));
        let mut versions = conflict.versions.clone();
        versions.sort();
        let info = ConflictInfo {
            crate_name: conflict.crate_name.clone(),
            version_a: versions[0].clone(),
            version_b: versions.last().unwrap().clone(),
        };

        // Проверяем реестр сначала
        if let Some(existing) = search_registry(&info.crate_name).await {
            println!(
                "{}",
                style(format!(
                    "📦 Найдено готовое решение в реестре: {}",
                    existing
                ))
                .green()
            );
            // здесь мы бы скачали shim, но пока просто помечаем
        }
        let shim = if let Some(ref ai_cfg) = ai_config {
            let mut attempts = 0;
            let mut shim = None;
            while attempts < MAX_AI_RETRIES {
                let ai_spin = ui::spinner(&format!(
                    "🧠 AI генерирует адаптер (попытка {})...",
                    attempts + 1
                ));
                match generate_shim_ai(&info, ai_cfg).await {
                    Ok(s) => {
                        // Создаём временный проект с shim и проверяем сборку + тесты
                        let tmp_dir = healed_dir.join(format!("tmp_{}", s.name));
                        let _ = std::fs::remove_dir_all(&tmp_dir);
                        fs::create_dir_all(tmp_dir.join("src"))?;
                        fs::write(tmp_dir.join("Cargo.toml"), &s.manifest)?;
                        fs::write(tmp_dir.join("src/lib.rs"), &s.lib_rs)?;
                        // Проверка сборки
                        if test_build(&tmp_dir).is_ok() {
                            // Если есть тесты, запускаем
                            if let Err(e) = run_tests(&tmp_dir) {
                                ui::finish_spinner(
                                    &ai_spin,
                                    false,
                                    &format!("AI-адаптер не прошёл тесты: {}. Повтор...", e),
                                );
                                attempts += 1;
                                continue;
                            }
                            ui::finish_spinner(&ai_spin, true, "AI-адаптер создан и прошёл тесты");
                            shim = Some(s);
                            break;
                        } else {
                            ui::finish_spinner(
                                &ai_spin,
                                false,
                                "AI-адаптер не компилируется. Повтор...",
                            );
                            attempts += 1;
                        }
                    }
                    Err(e) => {
                        ui::finish_spinner(
                            &ai_spin,
                            false,
                            &format!("AI-ошибка: {}. Повтор...", e),
                        );
                        attempts += 1;
                    }
                }
            }
            match shim {
                Some(s) => s,
                None => {
                    println!(
                        "{}",
                        style("⚠️  AI не смог создать работающий адаптер. Использую базовый.")
                            .yellow()
                    );
                    generate_shim(&info)?
                }
            }
        } else {
            generate_shim(&info)?
        };

        let shim_dir = healed_dir.join(&shim.name);
        fs::create_dir_all(shim_dir.join("src"))?;
        fs::write(shim_dir.join("Cargo.toml"), &shim.manifest)?;
        fs::write(shim_dir.join("src/lib.rs"), &shim.lib_rs)?;

        if let Some(deps) = doc.get_mut("dependencies").and_then(|d| d.as_table_mut()) {
            if deps.contains_key(&conflict.crate_name) {
                let shim_dep = format!(
                    "{{ package = \"{}\", version = \"0.1.0\", path = \"{}/{}\" }}",
                    shim.name,
                    healed_dir.display(),
                    shim.name
                );
                let parsed: toml::Value = toml::from_str(&shim_dep)?;
                deps.insert(conflict.crate_name.clone(), parsed);
            }
        }
        pb_total.inc(1);
    }
    pb_total.finish_with_message("Генерация адаптеров завершена");
    println!();

    let new_manifest = toml::to_string_pretty(&doc)?;
    fs::write(path, &new_manifest)?;

    let build_spin = ui::spinner("🔨 Проверяю сборку изменённого проекта...");
    match test_build(project_dir) {
        Ok(_) => {
            ui::finish_spinner(&build_spin, true, "Проект успешно собирается после лечения");
            println!(
                "{}",
                style("✅ HealDep вылечил все конфликты!").green().bold()
            );
        }
        Err(e) => {
            ui::finish_spinner(&build_spin, false, "Сборка провалилась");
            println!("{}", style(format!("❌ Ошибка: {}", e)).red());
            return Err(e);
        }
    }
    Ok(())
}
