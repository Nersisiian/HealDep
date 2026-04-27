pub fn save_shim(name: &str, data: &[u8]) -> anyhow::Result<()> {
    std::fs::create_dir_all("./healdep_cache")?;
    std::fs::write(format!("./healdep_cache/{}", name), data)?;
    Ok(())
}
