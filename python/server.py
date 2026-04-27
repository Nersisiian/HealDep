from flask import Flask, render_template, request, jsonify
from healdep_python import detect_conflicts, heal
from datetime import datetime
import json, os

app = Flask(__name__, template_folder='templates', static_folder='../web/static')

HISTORY_LOG = "logs/heal_history.json"

def load_history():
    if not os.path.exists(HISTORY_LOG):
        return []
    with open(HISTORY_LOG, "r", encoding="utf-8") as f:
        return json.load(f)

def save_history(entry):
    history = load_history()
    history.append(entry)
    with open(HISTORY_LOG, "w", encoding="utf-8") as f:
        json.dump(history, f, indent=2, ensure_ascii=False)

@app.route('/')
def dashboard():
    return render_template('dashboard.html')

@app.route('/api/analyze', methods=['POST'])
def analyze():
    data = request.get_json()
    req_file = data.get('file', 'requirements.txt')
    conflicts = detect_conflicts(req_file)
    return jsonify({'conflicts': conflicts, 'healthy': len(conflicts)==0})

@app.route('/api/heal', methods=['POST'])
def api_heal():
    data = request.get_json()
    req_file = data.get('file', 'requirements.txt')
    use_ai = data.get('ai', False)
    try:
        heal(req_file, use_ai=use_ai)
        entry = {
            "timestamp": datetime.now().isoformat(),
            "file": req_file,
            "ai": use_ai,
            "status": "success"
        }
        save_history(entry)
        return jsonify({'status': 'ok', 'message': 'Лечение завершено'})
    except Exception as e:
        entry = {
            "timestamp": datetime.now().isoformat(),
            "file": req_file,
            "ai": use_ai,
            "status": "error",
            "error": str(e)
        }
        save_history(entry)
        return jsonify({'status': 'error', 'message': str(e)})

@app.route('/api/history', methods=['GET'])
def history():
    return jsonify(load_history())


@app.route('/api/badge/<user>/<repo>')
def badge(user, repo):
    history = load_history()
    last = history[-1] if history else None
    healthy = (last and last.get("status") == "success") if last else False
    return jsonify({
        "schemaVersion": 1,
        "label": "HealDep",
        "message": "healthy" if healthy else "conflicts",
        "color": "green" if healthy else "red"
    })

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000, debug=True)


