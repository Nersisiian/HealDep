const vscode = require('vscode');
const { exec } = require('child_process');
const path = require('path');

function activate(context) {
    let disposable = vscode.commands.registerCommand('healdep.healCurrentFile', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return vscode.window.showErrorMessage('Нет открытого файла.');
        
        const docPath = editor.document.fileName;
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(docPath));
        if (!workspaceFolder) return vscode.window.showErrorMessage('Файл не принадлежит рабочей папке.');

        // Определяем тип проекта и вызываем соответствующую лечилку
        const root = workspaceFolder.uri.fsPath;
        let cmd = '';
        if (docPath.endsWith('Cargo.toml')) {
            cmd = `healdep heal "${docPath}" --ai`;
        } else if (docPath.endsWith('requirements.txt')) {
            cmd = `python "${path.join(root, 'python', 'healdep_python.py')}" heal "${docPath}" --ai`;
        } else if (docPath.endsWith('package.json')) {
            cmd = `python "${path.join(root, 'python', 'healdep_npm.py')}" heal "${docPath}"`;
        } else {
            return vscode.window.showErrorMessage('Тип файла не поддерживается. Используйте Cargo.toml, requirements.txt или package.json.');
        }

        vscode.window.showInformationMessage('HealDep запускает лечение…');
        exec(cmd, { cwd: root }, (error, stdout, stderr) => {
            if (error) {
                vscode.window.showErrorMessage(`Лечение не удалось: ${stderr}`);
            } else {
                vscode.window.showInformationMessage('HealDep: проект вылечен!');
                // Обновить окно проводника
                vscode.commands.executeCommand('workbench.files.action.refreshFilesExplorer');
            }
        });
    });

    context.subscriptions.push(disposable);
}
exports.activate = activate;
