use std::path::Path;
use std::process::Command;

pub fn test_build(project_dir: &Path) -> anyhow::Result<()> {
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(project_dir)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Сборка не удалась:\n{}", stderr))
    }
}

/// Запускает тесты проекта (если есть)
pub fn run_tests(project_dir: &Path) -> anyhow::Result<()> {
    let output = Command::new("cargo")
        .arg("test")
        .current_dir(project_dir)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Тесты провалились:\n{}", stderr))
    }
}
