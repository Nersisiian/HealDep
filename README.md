<h1 align="center">
  🩺 HealDep
</h1>

<p align="center">
  <strong>The first self-healing package manager powered by AI</strong><br>
  Automatically resolves dependency conflicts in Rust, Python, and npm
</p>

<p align="center">
  <a href="https://github.com/Nersisiian/HealDep/actions"><img src="https://github.com/Nersisiian/HealDep/actions/workflows/ci.yml/badge.svg" alt="CI/CD"></a>
  <a href="https://github.com/Nersisiian/HealDep/pkgs/container/healdep"><img src="https://img.shields.io/badge/docker-ghcr-blue" alt="Docker"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green" alt="License"></a>
</p>

---

## ❓ What is HealDep?

HealDep is the **first package manager** that doesn't just report dependency conflicts — it **heals them**.  
When two libraries demand incompatible versions of the same dependency, HealDep automatically synthesises an adapter shim, letting your project compile and run without manual intervention.

### ✨ Features
- 🔍 **Conflict detection** for Cargo (Rust), pip (Python), and npm (JavaScript)
- 🧠 **AI‑powered adapter generation** (supports local Ollama, OpenAI, or any compatible API)
- 🐳 **Docker image** ready for CI/CD
- 🖥 **Web dashboard** with healing history
- 💉 **Self‑healing loop** – retries with AI until tests pass
- 📛 **Badge & init** – add HealDep to any project in one command

---

## 🚀 Quick Start

### Local (Rust CLI)
```bash
git clone https://github.com/Nersisiian/HealDep.git
cd healdep
cargo build --release
./target/release/healdep analyze examples/demo_app/Cargo.toml
./target/release/healdep heal examples/demo_app/Cargo.toml
With AI (requires Ollama or OpenAI key)
bash
# Ollama
ollama pull codellama:7b
./target/release/healdep heal examples/demo_app/Cargo.toml --ai
Python & npm
bash
python python/healdep_python.py analyze requirements.txt
python python/healdep_npm.py heal package.json --ai
🌐 Web Dashboard
bash
python python/server.py
# Open http://localhost:5000
🐳 Docker
bash
docker pull ghcr.io/nersisiian/healdep:v0.1.0
docker run -p 5000:5000 ghcr.io/nersisiian/healdep:v0.1.0
🖼️ Screenshots
Rust CLI
AnalyzeHeal
https://docs/images/rust-analyze.pnghttps://docs/images/rust-heal.png
Python CLI
AnalyzeHeal
https://docs/images/python-analyze.pnghttps://docs/images/python-heal.png
npm CLI
AnalyzeHeal
https://docs/images/npm-analyze.pnghttps://docs/images/npm-heal.png
Web Dashboard
Main pageConflict analysis
https://docs/images/dashboard-main.pnghttps://docs/images/dashboard-analyze.png
Docker
Container launchDashboard in Docker
https://docs/images/docker-up.pnghttps://docs/images/docker-dashboard.png
🤖 AI Providers
HealDep supports multiple AI backends for intelligent adapter generation.
Configure healdep.toml:

toml
[ai]
provider = "ollama"           # or "openai"
ollama_model = "codellama:7b"
# openai_api_key = "sk-..."
📦 Package Ecosystem
LanguageFileCommand
RustCargo.tomlhealdep analyze / heal
Pythonrequirements.txtpython healdep_python.py analyze / heal
npmpackage.jsonpython healdep_npm.py analyze / heal
🧪 CI/CD Integration
HealDep includes ready‑to‑use GitHub Actions workflows:

ci.yml – lint, test, build, and publish Docker image

healdep-action.yml – auto‑heal dependencies on push

dependabot-heal.yml – automatically heal PRs opened by Dependabot

📜 License
MIT © 2025 Nersisiian