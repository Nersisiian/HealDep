# рџ©є HealDep вЂ“ Self-Healing Package Manager

РџРµСЂРІС‹Р№ РІ РјРёСЂРµ РїР°РєРµС‚РЅС‹Р№ РјРµРЅРµРґР¶РµСЂ, Р°РІС‚РѕРјР°С‚РёС‡РµСЃРєРё СЂР°Р·СЂРµС€Р°СЋС‰РёР№ РєРѕРЅС„Р»РёРєС‚С‹ Р·Р°РІРёСЃРёРјРѕСЃС‚РµР№
СЃ РїРѕРјРѕС‰СЊСЋ СЃРёРЅС‚РµР·Р° РєРѕРґР° (AIвЂ‘РіРµРЅРµСЂР°С†РёСЏ Р°РґР°РїС‚РµСЂРѕРІ).

## Р’РѕР·РјРѕР¶РЅРѕСЃС‚Рё
- РђРЅР°Р»РёР· РіСЂР°С„Р° Р·Р°РІРёСЃРёРјРѕСЃС‚РµР№ (Р»СЋР±РѕР№ CargoвЂ‘РїСЂРѕРµРєС‚)
- РћР±РЅР°СЂСѓР¶РµРЅРёРµ РЅРµСЃРѕРІРјРµСЃС‚РёРјС‹С… РІРµСЂСЃРёР№ (diamond dependency problem)
- РђРІС‚РѕРјР°С‚РёС‡РµСЃРєР°СЏ РіРµРЅРµСЂР°С†РёСЏ РїСЂРѕСЃР»РѕР№РєРёвЂ‘Р°РґР°РїС‚РµСЂР°
- РњРѕРґРёС„РёРєР°С†РёСЏ Cargo.toml В«РЅР° Р»РµС‚СѓВ»
- РџСЂРѕРІРµСЂРєР° СЃР±РѕСЂРєРё РІ РїРµСЃРѕС‡РЅРёС†Рµ

## Р‘С‹СЃС‚СЂС‹Р№ СЃС‚Р°СЂС‚
```bash
git clone git@github.com:your/healdep.git
cd healdep
cargo build --release
./target/release/healdep analyze ./examples/demo_app/Cargo.toml
./target/release/healdep heal ./examples/demo_app/Cargo.toml

## 🖼️ Скриншоты

### Rust CLI
| Анализ | Лечение | AI‑лечение |
|--------|---------|------------|
| ![Rust analyze](docs/images/rust-analyze.png) | ![Rust heal](docs/images/rust-heal.png) | ![Rust AI](docs/images/rust-heal-ai.png) |

### Python CLI
| Анализ | Лечение |
|--------|---------|
| ![Python analyze](docs/images/python-analyze.png) | ![Python heal](docs/images/python-heal.png) |

### npm CLI
| Анализ | Лечение |
|--------|---------|
| ![npm analyze](docs/images/npm-analyze.png) | ![npm heal](docs/images/npm-heal.png) |

### Веб‑дашборд
| Главная с историей | Анализ конфликта |
|--------------------|------------------|
| ![Dashboard main](docs/images/dashboard-main.png) | ![Dashboard analyze](docs/images/dashboard-analyze.png) |

### Docker
| Запуск в контейнере | Дашборд через Docker |
|---------------------|----------------------|
| ![Docker up](docs/images/docker-up.png) | ![Docker dashboard](docs/images/docker-dashboard.png) |

### VS Code расширение
| Команда в палитре |
|-------------------|
| ![VS Code](docs/images/vscode-extension.png) |
