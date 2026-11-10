const vscode = require('vscode');
const { exec } = require('child_process');

let diagnosticCollection;

function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('kora');
    context.subscriptions.push(diagnosticCollection);

    // Run diagnostics on open, change, and save
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(doc => validateKoraDocument(doc)),
        vscode.workspace.onDidChangeTextDocument(e => validateKoraDocument(e.document)),
        vscode.workspace.onDidSaveTextDocument(doc => validateKoraDocument(doc))
    );

    // Validate active editor
    if (vscode.window.activeTextEditor) {
        validateKoraDocument(vscode.window.activeTextEditor.document);
    }
}

function validateKoraDocument(document) {
    if (document.languageId !== 'kora' && !document.fileName.endsWith('.kora')) {
        return;
    }

    const filePath = document.fileName;
    // Execute Kora compiler check
    exec(`kora check "${filePath}"`, (err, stdout, stderr) => {
        const diagnostics = [];
        const output = stderr || stdout || '';

        // Match Kora error diagnostic lines: error[E4001]: ... \n --> file:line:col
        const errorRegex = /(error|warning)\[(E\d+)\]:\s*(.*?)\n\s*-->\s*(.*?):(\d+):(\d+)/g;
        let match;

        while ((match = errorRegex.exec(output)) !== null) {
            const isError = match[1] === 'error';
            const code = match[2];
            const message = match[3];
            const line = parseInt(match[5], 10) - 1;
            const col = parseInt(match[6], 10) - 1;

            const range = new vscode.Range(
                new vscode.Position(Math.max(0, line), Math.max(0, col)),
                new vscode.Position(Math.max(0, line), Math.max(0, col + 10))
            );

            const diag = new vscode.Diagnostic(
                range,
                `[${code}] ${message}`,
                isError ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning
            );
            diag.source = 'kora';
            diagnostics.push(diag);
        }

        diagnosticCollection.set(document.uri, diagnostics);
    });
}

function deactivate() {
    if (diagnosticCollection) {
        diagnosticCollection.clear();
    }
}

module.exports = { activate, deactivate };
