#!/usr/bin/env python3
# healdep_python.py – Python Dependency Healer (исправленный, с argparse и AI)
import sys, subprocess, json, os, re, urllib.request
from pathlib import Path
from typing import List, Dict, Optional

# ---------- парсинг и обнаружение конфликтов ----------
def parse_requirements(req_file: str) -> Dict[str, List[str]]:
    with open(req_file, 'r', encoding='utf-8-sig') as f:
        lines = f.readlines()
    deps = {}
    for line in lines:
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        m = re.match(r'^([A-Za-z0-9_\-\.]+)\s*(([><=!~]+\s*[\w\.\*]+)(,\s*[><=!~]+\s*[\w\.\*]+)*)?', line)
        if m:
            pkg = m.group(1).lower().replace('-', '_')
            spec = m.group(2).strip() if m.group(2) else 'any'
            deps.setdefault(pkg, []).append(spec)
    return deps

def detect_conflicts_local(req_file: str) -> List[dict]:
    requested = parse_requirements(req_file)
    conflicts = []
    for pkg, specs in requested.items():
        if len(specs) > 1:
            conflicts.append({'package': pkg, 'requested': specs, 'resolved': 'невозможно (конфликт)'})
    return conflicts

def get_resolved_versions(req_file: str) -> Dict[str, str]:
    try:
        res = subprocess.run([sys.executable, '-m', 'pip', 'install', '--dry-run', '--report', '-', '-r', req_file],
                             capture_output=True, text=True, check=True)
        report = json.loads(res.stdout)
        return {item['metadata']['name'].lower().replace('-','_'): item['metadata']['version']
                for item in report.get('install', [])}
    except Exception:
        return {}

def detect_conflicts(req_file: str) -> List[dict]:
    resolved = get_resolved_versions(req_file)
    if resolved:
        requested = parse_requirements(req_file)
        conflicts = []
        for pkg, specs in requested.items():
            if pkg in resolved:
                for spec in specs:
                    if spec != 'any' and resolved[pkg] not in spec:
                        conflicts.append({'package': pkg, 'requested': specs, 'resolved': resolved[pkg]})
                        break
        return conflicts
    return detect_conflicts_local(req_file)

# ---------- генерация shim ----------
def generate_shim(conflict: dict, output_dir: str):
    pkg = conflict['package']
    vers = conflict.get('requested', ['latest'])
    safe_name = f"heal_{pkg}_{vers[0].replace('.','_').replace(' ','_')}"
    pkg_path = Path(output_dir) / safe_name
    pkg_path.mkdir(parents=True, exist_ok=True)
    (pkg_path / 'setup.py').write_text(
        f"from setuptools import setup; setup(name='{safe_name}', version='0.1.0', packages=['{safe_name}'])")
    (pkg_path / safe_name).mkdir(exist_ok=True)
    (pkg_path / safe_name / '__init__.py').write_text(
        f"# Auto-generated shim for {pkg}\ntry:\n    import {pkg}\nexcept ImportError:\n    pass\n", encoding='utf-8')
    return safe_name

# ---------- AI-часть ----------
def load_ai_config():
    try:
        with open('healdep.toml', 'rb') as f:
            import tomllib
            return tomllib.load(f).get('ai')
    except Exception:
        return None

def call_ai(prompt: str, config: dict) -> str:
    provider = config.get('provider', 'ollama')
    if provider == 'ollama':
        url = config.get('ollama_url', 'http://localhost:11434/api/generate')
        model = config.get('ollama_model', 'codellama:7b')
        body = json.dumps({'model': model, 'prompt': prompt, 'stream': False,
                           'options': {'temperature': config.get('temperature', 0.2),
                                       'num_predict': config.get('max_tokens', 2048)}}).encode()
        req = urllib.request.Request(url, data=body, headers={'Content-Type': 'application/json'})
        with urllib.request.urlopen(req) as resp:
            return json.loads(resp.read())['response']
    elif provider == 'openai':
        key = config['openai_api_key']
        model = config.get('openai_model', 'gpt-4o')
        body = json.dumps({'model': model, 'messages': [{'role': 'user', 'content': prompt}],
                           'temperature': config.get('temperature', 0.2),
                           'max_tokens': config.get('max_tokens', 2048)}).encode()
        req = urllib.request.Request('https://api.openai.com/v1/chat/completions', data=body,
                                     headers={'Authorization': f'Bearer {key}', 'Content-Type': 'application/json'})
        with urllib.request.urlopen(req) as resp:
            return json.loads(resp.read())['choices'][0]['message']['content']
    else:
        raise ValueError(f'Unknown provider: {provider}')

# ---------- главные команды ----------
def heal(req_file: str, use_ai: bool = False):
    conflicts = detect_conflicts(req_file)
    if not conflicts:
        print('✅ Конфликтов не найдено.')
        return
    ai_cfg = load_ai_config() if use_ai else None
    healed_dir = Path('healed_shims_py')
    healed_dir.mkdir(exist_ok=True)
    for c in conflicts:
        print(f"🔧 Генерирую адаптер для {c['package']}...")
        if ai_cfg:
            prompt = f"Write a Python shim module that wraps conflicting versions of '{c['package']}': {c['requested']}. Output only valid Python code."
            try:
                code = call_ai(prompt, ai_cfg)
                shim_name = f"heal_{c['package']}_ai"
                p = healed_dir / shim_name
                p.mkdir(exist_ok=True)
                (p / 'setup.py').write_text(f"from setuptools import setup; setup(name='{shim_name}', version='0.1.0', packages=['{shim_name}'])")
                (p / shim_name).mkdir(exist_ok=True)
                (p / shim_name / '__init__.py').write_text(code, encoding='utf-8')
                print(f'   Создан AI-адаптер {shim_name}')
            except Exception as e:
                print(f'❌ AI-ошибка: {e}. Использую базовый адаптер.')
                generate_shim(c, str(healed_dir))
        else:
            name = generate_shim(c, str(healed_dir))
            print(f'   Создан {name}')
    print('💡 Shim-пакеты готовы в папке healed_shims_py.')

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('command', choices=['analyze', 'heal'])
    parser.add_argument('file', nargs='?', default='requirements.txt')
    parser.add_argument('--ai', action='store_true')
    args = parser.parse_args()
    if args.command == 'analyze':
        conflicts = detect_conflicts(args.file)
        if not conflicts:
            print('✅ Конфликтов нет.')
        for c in conflicts:
            print(f"⚠️  {c['package']}: запрошено {c['requested']} -> реально {c['resolved']}")
    else:
        heal(args.file, args.ai)
