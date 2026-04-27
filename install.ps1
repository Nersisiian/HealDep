# install.ps1 – HealDep Universe Installer (теперь с Docker)
# Запуск: PowerShell (администратор) -> .\install.ps1

$ErrorActionPreference = "Stop"
Write-Host "🩺 HealDep: установка полного окружения (Rust + Python + Docker)" -ForegroundColor Green

if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "⚠️  Рекомендуется запускать от администратора." -ForegroundColor Yellow
}

# --- 1. Rust ---
Write-Host "`n🔍 Проверяю Rust..." -ForegroundColor Cyan
try { $null = Get-Command cargo -ErrorAction Stop; Write-Host "✅ Rust уже установлен" -ForegroundColor Green }
catch {
    Write-Host "📦 Устанавливаю Rust через winget..." -ForegroundColor Cyan
    winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    Write-Host "✅ Rust установлен (перезапусти PowerShell при необходимости)" -ForegroundColor Green
}

# --- 2. Python ---
Write-Host "`n🔍 Проверяю Python..." -ForegroundColor Cyan
try { $null = Get-Command python -ErrorAction Stop; Write-Host "✅ Python уже установлен" -ForegroundColor Green }
catch {
    Write-Host "📦 Устанавливаю Python 3.11 через winget..." -ForegroundColor Cyan
    winget install Python.Python.3.11 --accept-source-agreements --accept-package-agreements
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
    Write-Host "✅ Python установлен" -ForegroundColor Green
}

# --- 3. Docker (опционально) ---
Write-Host "`n🐳 Проверяю Docker..." -ForegroundColor Cyan
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Docker не найден. Установи Docker Desktop с https://docs.docker.com/desktop/" -ForegroundColor Yellow
} else {
    Write-Host "✅ Docker доступен" -ForegroundColor Green
}

# --- 4. Python виртуальное окружение ---
Write-Host "`n🐍 Настройка Python-окружения..." -ForegroundColor Cyan
if (-not (Test-Path ".venv")) { python -m venv .venv }
.\.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
pip install -r python\requirements.txt
Write-Host "✅ Python-зависимости установлены" -ForegroundColor Green

# --- 5. Сборка Rust ---
Write-Host "`n🦀 Сборка HealDep (Rust)..." -ForegroundColor Cyan
cargo build --release
Write-Host "✅ Rust-сборка завершена" -ForegroundColor Green

# --- 6. Инструкция ---
Write-Host "`n🚀 Установка завершена! Используй:" -ForegroundColor Magenta
Write-Host "   .\target\release\healdep analyze" -ForegroundColor White
Write-Host "   .\target\release\healdep heal examples\demo_app\Cargo.toml --ai" -ForegroundColor White
Write-Host "   python python\server.py" -ForegroundColor White
Write-Host "   docker-compose up --build       (веб-дашборд в Docker)" -ForegroundColor White
