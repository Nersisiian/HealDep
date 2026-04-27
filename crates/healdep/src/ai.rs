use crate::config::AiConfig;
use healdep_synthesizer::AiGenerator;
use std::future::Future;
use std::pin::Pin;

pub async fn generate_adapter(prompt: String, config: &AiConfig) -> anyhow::Result<String> {
    match config.provider.as_str() {
        "ollama" => generate_ollama(&prompt, config).await,
        "openai" => generate_openai(&prompt, config).await,
        _ => anyhow::bail!("Неизвестный AI-провайдер: {}", config.provider),
    }
}

async fn generate_ollama(prompt: &str, config: &AiConfig) -> anyhow::Result<String> {
    let url = config
        .ollama_url
        .as_deref()
        .unwrap_or("http://localhost:11434/api/generate");
    let model = config.ollama_model.as_deref().unwrap_or("llama2");
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": config.temperature.unwrap_or(0.2),
            "num_predict": config.max_tokens.unwrap_or(2048),
        }
    });
    let client = reqwest::Client::new();
    let res = client.post(url).json(&body).send().await?;
    let json: serde_json::Value = res.json().await?;
    json["response"]
        .as_str()
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow::anyhow!("Ollama не вернул response"))
}

async fn generate_openai(prompt: &str, config: &AiConfig) -> anyhow::Result<String> {
    let api_key = config
        .openai_api_key
        .as_ref()
        .ok_or(anyhow::anyhow!("Не указан openai_api_key"))?;
    let model = config.openai_model.as_deref().unwrap_or("gpt-3.5-turbo");
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": config.temperature.unwrap_or(0.2),
        "max_tokens": config.max_tokens.unwrap_or(2048),
    });
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;
    let json: serde_json::Value = res.json().await?;
    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow::anyhow!("OpenAI не вернул контент"))
}

// Реализация трейта для AiConfig
impl AiGenerator for AiConfig {
    fn generate(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send + '_>> {
        Box::pin(generate_adapter(prompt, self))
    }
}
