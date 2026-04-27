#!/usr/bin/env python3
# healdep_npm.py – Анализатор и лекарь для npm/package.json
import sys, json, os, subprocess
from pathlib import Path
from typing import Dict, List

def parse_package_json(path: str) -> dict:
    with open(path, 'r', encoding='utf-8-sig') as f:
        return json.load(f)

def detect_npm_conflicts(pkg: dict) -> List[dict]:
    """Ищет множественные объявления одного пакета в разных секциях."""
    conflicts = []
    deps = pkg.get('dependencies', {})
    dev = pkg.get('devDependencies', {})
    for name in set(deps.keys()) & set(dev.keys()):
        if deps[name] != dev[name]:
            conflicts.append({
                'package': name,
                'versions': [deps[name], dev[name]]
            })
    return conflicts

def generate_npm_shim(conflict: dict, output_dir: str):
    pkg_name = conflict['package']
    versions = sorted(conflict['versions'])
    shim_name = f"heal-npm-{pkg_name}"
    shim_dir = Path(output_dir) / shim_name
    shim_dir.mkdir(parents=True, exist_ok=True)
    package_json = {
        "name": shim_name,
        "version": "1.0.0",
        "description": f"Auto-generated HealDep shim for {pkg_name}",
        "main": "index.js",
        "scripts": {"test": "echo 'no tests'"},
        "dependencies": {
            f"{pkg_name}-a": "npm:" + pkg_name + "@" + conflict['versions'][0],
            f"{pkg_name}-b": "npm:" + pkg_name + "@" + conflict['versions'][1]
        }
    }
    with open(shim_dir / "package.json", 'w') as f:
        json.dump(package_json, f, indent=2)
    index_js = f"""
// HealDep shim for {pkg_name}
const a = require('{pkg_name}-a');
const b = require('{pkg_name}-b');
module.exports = a; // Default to older version, extend if needed
"""
    (shim_dir / "index.js").write_text(index_js)
    return shim_name

def analyze_npm(file_path: str):
    pkg = parse_package_json(file_path)
    conflicts = detect_npm_conflicts(pkg)
    if conflicts:
        for c in conflicts:
            print(f"⚠️  {c['package']}: {c['versions'][0]} vs {c['versions'][1]}")
    else:
        print("✅ Конфликтов npm не найдено.")
    return conflicts

def heal_npm(file_path: str):
    pkg = parse_package_json(file_path)
    conflicts = detect_npm_conflicts(pkg)
    if not conflicts:
        print("Нет конфликтов, лечение не требуется.")
        return
    healed_dir = Path("healed_shims_npm")
    healed_dir.mkdir(exist_ok=True)
    for c in conflicts:
        print(f"🔧 Генерирую шот для {c['package']}...")
        shim_name = generate_npm_shim(c, str(healed_dir))
        print(f"   Создан {shim_name}")
    print("Готово.")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Использование: python healdep_npm.py analyze|heal <package.json>")
    else:
        cmd = sys.argv[1]
        f = sys.argv[2]
        if cmd == "analyze":
            analyze_npm(f)
        elif cmd == "heal":
            heal_npm(f)

