document.addEventListener('DOMContentLoaded', () => {
    const fileInput = document.getElementById('fileInput');
    const analyzeBtn = document.getElementById('analyzeBtn');
    const healBtn = document.getElementById('healBtn');
    const useAiCheckbox = document.getElementById('useAiCheckbox');
    const statusDiv = document.getElementById('status');
    const historyDiv = document.getElementById('historyTable');

    function loadHistory() {
        fetch('/api/history')
            .then(res => res.json())
            .then(data => {
                if (data.length === 0) {
                    historyDiv.innerHTML = '<p>Пока нет записей.</p>';
                    return;
                }
                let table = '<table style="width:100%; border-collapse:collapse;"><tr><th>Время</th><th>Файл</th><th>AI</th><th>Статус</th></tr>';
                data.reverse().forEach(entry => {
                    const statusColor = entry.status === 'success' ? '#3fb950' : '#f85149';
                    table += `<tr style="border-bottom:1px solid #30363d;">
                        <td>${new Date(entry.timestamp).toLocaleString()}</td>
                        <td>${entry.file}</td>
                        <td>${entry.ai ? '✅' : '❌'}</td>
                        <td style="color:${statusColor}">${entry.status}</td>
                    </tr>`;
                });
                table += '</table>';
                historyDiv.innerHTML = table;
            });
    }
    loadHistory();

    analyzeBtn.addEventListener('click', async () => {
        statusDiv.innerHTML = '⏳ Анализ...';
        const response = await fetch('/api/analyze', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({file: 'requirements.txt'})
        });
        const data = await response.json();
        if (data.healthy) {
            statusDiv.innerHTML = '✅ Проект здоров!';
        } else {
            let html = `<strong>Найдено конфликтов: ${data.conflicts.length}</strong><div class="conflict-list">`;
            data.conflicts.forEach(c => {
                html += `<div class="conflict-item">📦 ${c.package}: запрошено ${c.requested} → реально ${c.resolved}</div>`;
            });
            html += '</div>';
            statusDiv.innerHTML = html;
        }
    });

    healBtn.addEventListener('click', async () => {
        const useAi = useAiCheckbox.checked;
        statusDiv.innerHTML = useAi ? '🤖 AI-лечение...' : '🚑 Лечение...';
        const response = await fetch('/api/heal', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({file: 'requirements.txt', ai: useAi})
        });
        const data = await response.json();
        if (data.status === 'ok') {
            statusDiv.innerHTML = '✅ Лечение завершено! Shim-пакеты созданы.';
            loadHistory();
        } else {
            statusDiv.innerHTML = `❌ Ошибка: ${data.message}`;
        }
    });
});
