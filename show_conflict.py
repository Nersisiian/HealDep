# show_conflict.py – временный анализатор для скриншота
import re, sys

def detect_conflicts(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()
    pkgs = {}
    for line in lines:
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        m = re.match(r'^([A-Za-z0-9_-]+)\s*([><=!~].*)$', line)
        if m:
            pkg = m.group(1).lower().replace('-','_')
            ver = m.group(2).strip()
            pkgs.setdefault(pkg, []).append(ver)
    for pkg, vers in pkgs.items():
        if len(vers) > 1:
            print(f"⚠️  {pkg}: запрошено {vers} -> реально невозможно (конфликт)")
            return
    print("✅ Конфликтов нет.")

if __name__ == '__main__':
    detect_conflicts(sys.argv[1] if len(sys.argv)>1 else 'requirements.txt')
