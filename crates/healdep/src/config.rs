use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ai: Option<AiConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub ollama_url: Option<String>,
    pub ollama_model: Option<String>,
    pub openai_api_key: Option<String>,
    pub openai_model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub fn load_config() -> anyhow::Result<Config> {
    let content = std::fs::read_to_string("healdep.toml")
        .or_else(|_| std::fs::read_to_string(Path::new("..").join("healdep.toml")))?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
