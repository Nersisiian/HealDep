use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictInfo {
    pub crate_name: String,
    pub version_a: String,
    pub version_b: String,
}

pub struct ShimCrate {
    pub name: String,
    pub manifest: String,
    pub lib_rs: String,
}

pub fn generate_shim(conflict: &ConflictInfo) -> Result<ShimCrate> {
    let name = format!(
        "heal_{}_{}_{}",
        conflict.crate_name,
        conflict.version_a.replace('.', "_"),
        conflict.version_b.replace('.', "_")
    );

    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
crate_a = {{ package = "{}", version = "={}" }}
crate_b = {{ package = "{}", version = "={}" }}
"#,
        name, conflict.crate_name, conflict.version_a, conflict.crate_name, conflict.version_b
    );

    let lib_rs = format!(
        r#"// Автосгенерированный адаптер HealDep
// Для версий {} ({}) и {} ({})
// В будущем AI добавит преобразования и обёртки.

pub mod unified {{
    pub use crate_a::*;
}}
"#,
        conflict.crate_name, conflict.version_a, conflict.crate_name, conflict.version_b
    );

    Ok(ShimCrate {
        name,
        manifest,
        lib_rs,
    })
}

pub trait AiGenerator {
    fn generate(&self, prompt: String)
        -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>>;
}

pub async fn generate_shim_ai(
    conflict: &ConflictInfo,
    ai_config: &dyn AiGenerator,
) -> Result<ShimCrate> {
    let prompt = format!(
        "You are an expert Rust developer. A project has two conflicting versions of '{}': {} and {}. \
        Generate a Rust library (shim crate) that re-exports the API of the older version and adapts the newer version's calls to it. \
        Provide the complete contents for lib.rs, including any necessary trait implementations and function wrappers. \
        Only output valid Rust code, no explanations.",
        conflict.crate_name, conflict.version_a, conflict.version_b
    );
    let ai_code = ai_config.generate(prompt).await?;
    let name = format!(
        "heal_{}_{}_{}",
        conflict.crate_name,
        conflict.version_a.replace('.', "_"),
        conflict.version_b.replace('.', "_")
    );
    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
crate_a = {{ package = "{}", version = "={}" }}
crate_b = {{ package = "{}", version = "={}" }}
"#,
        name, conflict.crate_name, conflict.version_a, conflict.crate_name, conflict.version_b
    );

    Ok(ShimCrate {
        name,
        manifest,
        lib_rs: ai_code,
    })
}
