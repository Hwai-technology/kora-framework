const vscode = require('vscode');
const path = require('path');
const fs = require('fs');
const { EXAMPLES_CATALOG, getNonce, getWebviewHtml } = require('./kora-webview-html');

function isCloudEndpoint(urlStr) {
    if (!urlStr || typeof urlStr !== 'string') return false;
    try {
        const raw = urlStr.trim();
        const fullUrl = (raw.startsWith('http://') || raw.startsWith('https://')) ? raw : ('http://' + raw);
        const parsed = new URL(fullUrl);
        const host = parsed.hostname.toLowerCase();
        const cloudDomains = ['azure.com', 'openai.com', 'anthropic.com', 'googleapis.com', 'github.com'];
        return cloudDomains.some(d => host === d || host.endsWith('.' + d));
    } catch (e) {
        return false;
    }
}

/**
 * Kora Assistant Webview View Provider
 * Manages the AI Copilot side panel, chat sessions, diff reviews, and compiler diagnostics.
 */
function getDiscoveredCatalog(extensionUri) {
    // 1. Dynamic live scan from open workspace
    try {
        if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
            for (const folder of vscode.workspace.workspaceFolders) {
                const examplesDir = path.join(folder.uri.fsPath, 'examples');
                if (fs.existsSync(examplesDir) && fs.statSync(examplesDir).isDirectory()) {
                    const entries = fs.readdirSync(examplesDir, { withFileTypes: true });
                    const dynamicList = [];
                    for (const entry of entries) {
                        if (entry.isDirectory() && !entry.name.startsWith('.')) {
                            const entryPath = path.join(examplesDir, entry.name);
                            const readmePath = path.join(entryPath, 'README.md');
                            let name = entry.name;
                            let desc = '';
                            let category = 'Production Applications';
                            let difficulty = 'Intermediate';
                            if (fs.existsSync(readmePath)) {
                                const content = fs.readFileSync(readmePath, 'utf8');
                                for (const line of content.split('\n')) {
                                    const trimmed = line.trim();
                                    if (trimmed.startsWith('# ') && name === entry.name) {
                                        const raw = trimmed.substring(2).trim();
                                        const pos = raw.indexOf(':');
                                        name = pos !== -1 ? raw.substring(pos + 1).trim() : raw;
                                    }
                                    if (trimmed.startsWith('Category:') || trimmed.startsWith('**Category**:')) {
                                        category = trimmed.split(':')[1].replace(/\*/g, '').trim();
                                    }
                                    if (trimmed.startsWith('Difficulty:') || trimmed.startsWith('**Difficulty**:')) {
                                        difficulty = trimmed.split(':')[1].replace(/\*/g, '').trim();
                                    }
                                    if (!desc && trimmed && !trimmed.startsWith('#') && !trimmed.startsWith('---') && !trimmed.startsWith('```') && !trimmed.startsWith('>')) {
                                        desc = trimmed;
                                    }
                                }
                            }
                            let fileCount = 0;
                            const countFiles = (dir) => {
                                try {
                                    const subEntries = fs.readdirSync(dir, { withFileTypes: true });
                                    for (const se of subEntries) {
                                        if (se.isDirectory() && !se.name.startsWith('.')) countFiles(path.join(dir, se.name));
                                        else if (se.name.endsWith('.kora') || se.name.endsWith('.kr')) fileCount++;
                                    }
                                } catch (e) {}
                            };
                            countFiles(entryPath);
                            if (fileCount > 0) {
                                dynamicList.push({
                                    slug: entry.name,
                                    name,
                                    category,
                                    difficulty,
                                    desc: desc || `Production-grade Kora implementation in ${category}`,
                                    file_count: fileCount
                                });
                            }
                        }
                    }
                    if (dynamicList.length > 0) {
                        // Natural sorting by numeric prefix
                        dynamicList.sort((a, b) => {
                            const numA = parseInt(a.slug.match(/^\d+/)?.[0] || '999999', 10);
                            const numB = parseInt(b.slug.match(/^\d+/)?.[0] || '999999', 10);
                            if (numA !== numB) return numA - numB;
                            return a.slug.localeCompare(b.slug);
                        });
                        return dynamicList;
                    }
                }
            }
        }
    } catch (e) {}

    // 2. Try reading catalog.json from extension knowledge
    try {
        const catalogPath = path.join(extensionUri.fsPath, 'knowledge', 'examples', 'catalog.json');
        if (fs.existsSync(catalogPath)) {
            const raw = fs.readFileSync(catalogPath, 'utf8');
            const parsed = JSON.parse(raw);
            if (parsed && Array.isArray(parsed.examples) && parsed.examples.length > 0) {
                return parsed.examples.map(ex => ({
                    slug: ex.slug,
                    name: ex.name,
                    category: ex.category,
                    difficulty: ex.difficulty,
                    desc: ex.description || ex.desc || '',
                    file_count: ex.file_count || 0
                }));
            }
        }
    } catch (e) {}

    return EXAMPLES_CATALOG;
}

function analyzeKoraDocument(document) {
    if (!document) return null;
    const text = document.getText();
    const fileName = path.basename(document.fileName);

    const hasMain = /fn\s+main\s*\(/.test(text);
    const hasListen = /app\.listen\s*\(/.test(text) || /\.listen\s*\(/.test(text);
    const isTestFile = fileName.includes('test') || /class\s+[A-Za-z0-9_]*Test/.test(text);
    const hasRoutes = /class\s+[A-Za-z0-9_]*Routes/.test(text) || /\b(get|post|put|delete)\s*\(/.test(text);
    const hasController = /class\s+[A-Za-z0-9_]*Controller/.test(text);
    const hasService = /class\s+[A-Za-z0-9_]*Service/.test(text);
    const hasRepository = /class\s+[A-Za-z0-9_]*Repository/.test(text) || /interface\s+[A-Za-z0-9_]*Repository/.test(text);
    const hasEntity = /@table\s*\(/.test(text) || /@entity\b/.test(text);
    const hasModel = /\bmodel\s+[A-Za-z0-9_]+/.test(text);
    const hasActor = /class\s+[A-Za-z0-9_]*Actor/.test(text);

    let classification = "Library Module";
    let icon = "";
    let isRunnable = false;
    let runCommand = null;

    if (isTestFile) {
        classification = "Automated Test Suite";
        icon = "";
        isRunnable = true;
        runCommand = "kora.runTests";
    } else if (hasMain || hasListen) {
        classification = "Runnable Entrypoint";
        icon = "";
        isRunnable = true;
        runCommand = "kora.runFile";
    } else if (hasController) {
        classification = "Clean Architecture Controller";
        icon = "";
    } else if (hasService) {
        classification = "Clean Architecture Service";
        icon = "";
    } else if (hasRepository) {
        classification = "Data Access Repository";
        icon = "";
    } else if (hasEntity) {
        classification = "Database Entity (@table)";
        icon = "";
    } else if (hasModel) {
        classification = "Pure DTO Model";
        icon = "";
    } else if (hasRoutes) {
        classification = "HTTP Routing Module";
        icon = "";
    } else if (hasActor) {
        classification = "Concurrency Actor";
        icon = "";
    }

    // Extract symbols
    const symbols = [];
    const symRegex = /\b(pub\s+)?(model|class|interface|enum|fn)\s+([A-Za-z0-9_]+)/g;
    let m;
    while ((m = symRegex.exec(text)) !== null) {
        symbols.push({ kind: m[2], name: m[3] });
    }

    // Extract imports
    const imports = [];
    const impRegex = /import\s*\{\s*([^}]+)\s*\}\s*from\s*["']([^"']+)["']/g;
    let im;
    while ((im = impRegex.exec(text)) !== null) {
        imports.push({ symbols: im[1].split(',').map(s => s.trim()), path: im[2] });
    }

    return {
        fileName,
        filePath: document.fileName,
        classification,
        icon,
        isRunnable,
        runCommand,
        symbols,
        imports,
        lineCount: document.lineCount,
        hasMain,
        isTestFile
    };
}

class KoraAssistantViewProvider {
    constructor(extensionUri, engine) {
        this.extensionUri = extensionUri;
        this.engine = engine;
        this.view = null;
        this.currentAnalysis = null;
        this.currentDiagnostics = [];
        this.activeProposal = null;
        this.activeAttachments = new Map();
    }

    resolveWebviewView(webviewView, context, token) {
        try {
            this.view = webviewView;
            webviewView.webview.options = {
                enableScripts: true,
                enableCommandUris: true,
                localResourceRoots: [this.extensionUri]
            };

            webviewView.webview.html = this.getHtmlForWebview(webviewView.webview);

        webviewView.webview.onDidReceiveMessage(async (message) => {
            switch (message.type) {
                case 'smartRun':
                    vscode.commands.executeCommand('kora.smartRun', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'checkFile':
                    vscode.commands.executeCommand('kora.checkFile', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'runTests':
                    vscode.commands.executeCommand('kora.runTests', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'generateTests':
                    vscode.commands.executeCommand('kora.generateTests', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'explainCode':
                    vscode.commands.executeCommand('kora.explainCode', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'auditFile':
                    vscode.commands.executeCommand('kora.auditFile', this.currentAnalysis ? this.currentAnalysis.filePath : null);
                    break;
                case 'analyzeWorkspace':
                    vscode.commands.executeCommand('kora.analyzeWorkspace');
                    break;
                case 'cloneExample':
                    if (message.slug) {
                        let terminal = vscode.window.terminals.find(t => t.name === 'Kora Example') || vscode.window.createTerminal('Kora Example');
                        terminal.show();
                        terminal.sendText(`kora example init ${message.slug}`);
                    }
                    break;
                case 'insertCode':
                    if (message.code && vscode.window.activeTextEditor) {
                        const editor = vscode.window.activeTextEditor;
                        editor.edit(editBuilder => {
                            editBuilder.insert(editor.selection.active, message.code);
                        });
                    }
                    break;
                case 'askQuestion':
                    await this.answerKoraPrompt(message.prompt, message.scope || 'workspace');
                    break;
                case 'requestAiFix':
                case 'requestWorkspaceRefactor':
                    {
                        const activeFile = (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document)
                            ? vscode.window.activeTextEditor.document.fileName
                            : (this.currentAnalysis ? this.currentAnalysis.filePath : null);
                        const goal = message.goal || 'Auto-Fix and refactor Kora invariants';
                        const lower = goal.toLowerCase();
                        const isAnalysisIntent = lower.includes('analyze') || lower.includes('analysis') || lower.includes('audit') || lower.includes('explain') || lower.includes('inspect') || lower.includes('check') || lower.includes('show') || lower.includes('what') || lower.includes('why') || lower.includes('how') || /حلل|فحص|اشرح|استكشف|طبقات|معمارية|ما هي|ما هو/.test(lower);

                        if (isAnalysisIntent) {
                            await this.answerKoraPrompt(goal, message.scope || 'workspace');
                        } else {
                            await this.planWorkspaceRefactor(message.targetPath || activeFile, goal, message.scope || 'workspace');
                        }
                    }
                    break;
                case 'executeRefactorPlan':
                    {
                        const activeFile = (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document)
                            ? vscode.window.activeTextEditor.document.fileName
                            : null;
                        await this.triggerWorkspaceRefactor(message.targetPath || activeFile, message.goal || 'Auto-Fix and refactor Kora invariants');
                    }
                    break;
                case 'newChat':
                    {
                        this.activeProposal = null;
                        this.currentAnalysis = null;
                        this.activeAttachments.clear();
                        if (this.view) {
                            this.view.webview.postMessage({ type: 'chatCleared' });
                            this.view.webview.postMessage({ type: 'attachmentsResolved', attachments: [] });
                        }
                    }
                    break;
                case 'acceptProposal':
                    await this.acceptActiveProposal(message.proposalId);
                    break;
                case 'rejectProposal':
                    await this.rejectActiveProposal(message.proposalId);
                    break;
                case 'setFileStatus':
                    this.setFileApprovalStatus(message.proposalId, message.filePath, message.status);
                    break;
                case 'previewDiff':
                    try {
                        if (message.code) {
                            let targetFilePath = message.filePath ? String(message.filePath).replace(/^file:\/\//, '') : null;
                            if (targetFilePath) {
                                try { targetFilePath = decodeURIComponent(targetFilePath); } catch (e) {}
                            }

                            // 1. If not specified, look at active editor
                            if (!targetFilePath && vscode.window.activeTextEditor) {
                                const activeDoc = vscode.window.activeTextEditor.document;
                                if (activeDoc && activeDoc.fileName && !activeDoc.fileName.includes('.diff_cache') && !activeDoc.fileName.includes('kora_visual_diffs') && !activeDoc.isUntitled) {
                                    targetFilePath = activeDoc.fileName;
                                }
                            }

                            // 2. Look at current analysis
                            if (!targetFilePath && this.currentAnalysis && this.currentAnalysis.filePath) {
                                targetFilePath = this.currentAnalysis.filePath;
                            }

                            // 3. Inspect visible text editors
                            if (!targetFilePath && vscode.window.visibleTextEditors && vscode.window.visibleTextEditors.length > 0) {
                                for (const ed of vscode.window.visibleTextEditors) {
                                    if (ed.document && ed.document.fileName && !ed.document.isUntitled && !ed.document.fileName.includes('kora_visual_diffs')) {
                                        targetFilePath = ed.document.fileName;
                                        break;
                                    }
                                }
                            }

                            // 4. Try to infer from code content / class name
                            if (!targetFilePath) {
                                const codeStr = String(message.code);
                                let candidateName = null;
                                if (codeStr.includes('class OrderProcessor') || codeStr.includes('OrderProcessor')) candidateName = 'order_processor.service.kora';
                                else if (codeStr.includes('model Coordinates') || codeStr.includes('Coordinates')) candidateName = 'coordinates.model.kora';
                                else if (codeStr.includes('test') && codeStr.includes('hello')) candidateName = 'hello.test.kora';

                                if (candidateName) {
                                    try {
                                        const found = await vscode.workspace.findFiles('**/' + candidateName, '**/node_modules/**', 1);
                                        if (found && found.length > 0) targetFilePath = found[0].fsPath;
                                    } catch (e) {}
                                }
                            }

                            const os = require('os');
                            const tempDir = path.join(os.tmpdir(), 'kora_visual_diffs');
                            if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir, { recursive: true });

                            let baseName = targetFilePath ? path.basename(targetFilePath) : 'component.kora';
                            let leftUri = null;

                            if (targetFilePath && fs.existsSync(targetFilePath)) {
                                leftUri = vscode.Uri.file(targetFilePath);
                            } else {
                                // Create an empty original file so the left side is empty and right side is green additions
                                const emptyOriginal = path.join(tempDir, `original_empty_${baseName}`);
                                fs.writeFileSync(emptyOriginal, '', 'utf8');
                                leftUri = vscode.Uri.file(emptyOriginal);
                            }

                            const proposedPath = path.join(tempDir, `proposed_${baseName}`);
                            fs.writeFileSync(proposedPath, message.code, 'utf8');
                            const rightUri = vscode.Uri.file(proposedPath);

                            const diffTitle = `${baseName} (Original - Red) ↔ ${baseName} (AI Proposed - Green)`;
                            await vscode.commands.executeCommand('vscode.diff', leftUri, rightUri, diffTitle, { preview: false });
                            break;
                        }

                        await this.engine.reviewManager.openVisualDiff(message.filePath, message.proposalId);
                    } catch (e) {
                        vscode.window.showErrorMessage(`Failed to open visual diff: ${e.message}`);
                    }
                    break;
                case 'runInTerminal':
                    if (message.command) {
                        let terminal = vscode.window.terminals.find(t => t.name === 'Kora Terminal') || vscode.window.createTerminal('Kora Terminal');
                        terminal.show();
                        terminal.sendText(message.command.trim());
                    }
                    break;
                case 'resolveDroppedAttachments':
                    await this.handleResolveDroppedAttachments(message.rawPaths, message.droppedItems);
                    break;
                case 'removeAttachment':
                    if (message.id) {
                        this.activeAttachments.delete(message.id);
                        if (this.view) {
                            this.view.webview.postMessage({
                                type: 'attachmentsResolved',
                                attachments: Array.from(this.activeAttachments.values())
                            });
                        }
                    }
                    break;
                case 'clearAttachments':
                    this.activeAttachments.clear();
                    if (this.view) {
                        this.view.webview.postMessage({
                            type: 'attachmentsResolved',
                            attachments: []
                        });
                    }
                    break;
                case 'selectFilesToAttach':
                    try {
                        const uris = await vscode.window.showOpenDialog({
                            canSelectFiles: true,
                            canSelectFolders: true,
                            canSelectMany: true,
                            openLabel: 'Attach to Kora AI'
                        });
                        if (uris && uris.length > 0) {
                            const paths = uris.map(u => u.fsPath);
                            await this.handleResolveDroppedAttachments(paths, []);
                        }
                    } catch (e) {}
                    break;
                case 'openFile':
                    if (message.filePath) {
                        try {
                            let targetPath = String(message.filePath).replace(/^file:\/\//, '').trim();
                            try { targetPath = decodeURIComponent(targetPath); } catch (e) {}

                            let uri = null;
                            const isDirQuery = targetPath.endsWith('/') || targetPath.endsWith('/tests') || targetPath === 'tests';
                            const cleanTarget = targetPath.replace(/\/+$/, '');

                            if (path.isAbsolute(cleanTarget) && fs.existsSync(cleanTarget)) {
                                uri = vscode.Uri.file(cleanTarget);
                            } else {
                                const root = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0] ? vscode.workspace.workspaceFolders[0].uri.fsPath : '';
                                const candidate = root ? path.join(root, cleanTarget) : cleanTarget;
                                if (fs.existsSync(candidate)) {
                                    uri = vscode.Uri.file(candidate);
                                } else {
                                    const baseName = path.basename(cleanTarget);
                                    if (isDirQuery) {
                                        try {
                                            const sampleFile = await vscode.workspace.findFiles(`**/${baseName}/*`, '**/node_modules/**', 1);
                                            if (sampleFile && sampleFile.length > 0) {
                                                uri = vscode.Uri.file(path.dirname(sampleFile[0].fsPath));
                                            }
                                        } catch (e) {}
                                    }
                                    if (!uri) {
                                        let found = [];
                                        if (cleanTarget.includes('/')) {
                                            try {
                                                found = await vscode.workspace.findFiles(`**/${cleanTarget.replace(/^\/+/, '')}`, '**/node_modules/**', 1);
                                            } catch (e) {}
                                        }
                                        if ((!found || found.length === 0) && baseName) {
                                            try {
                                                found = await vscode.workspace.findFiles(`**/${baseName}`, '**/node_modules/**', 1);
                                            } catch (e) {}
                                        }
                                        if (found && found.length > 0) {
                                            uri = found[0];
                                        } else {
                                            uri = vscode.Uri.file(candidate);
                                        }
                                    }
                                }
                            }

                            if (uri) {
                                try {
                                    const stat = await vscode.workspace.fs.stat(uri);
                                    if (stat.type === vscode.FileType.Directory) {
                                        await vscode.commands.executeCommand('revealInExplorer', uri);
                                        break;
                                    }
                                } catch (e) {}
                                const doc = await vscode.workspace.openTextDocument(uri);
                                await vscode.window.showTextDocument(doc);
                            }
                        } catch (e) {
                            vscode.window.showErrorMessage(`Could not open file: ${e.message}`);
                        }
                    }
                    break;
                case 'applyFix':
                    {
                        let editor = vscode.window.activeTextEditor;
                        if (message.filePath && (!editor || editor.document.fileName !== message.filePath)) {
                            try {
                                const doc = await vscode.workspace.openTextDocument(vscode.Uri.file(message.filePath));
                                editor = await vscode.window.showTextDocument(doc);
                            } catch (e) {}
                        }
                        if (editor && message.code) {
                            const doc = editor.document;
                            const fullRange = new vscode.Range(
                                doc.positionAt(0),
                                doc.positionAt(doc.getText().length)
                            );
                            await editor.edit(editBuilder => {
                                editBuilder.replace(fullRange, message.code);
                            });
                            await doc.save();
                            vscode.window.showInformationMessage(`AI fix successfully applied to ${path.basename(doc.fileName)}!`);
                            vscode.commands.executeCommand('kora.checkFile', doc.fileName);
                        }
                    }
                    break;
                case 'updateProvider':
                    if (message.provider) {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        await cfg.update('provider', message.provider, vscode.ConfigurationTarget.Global);
                        this.syncProviderToWebview();
                    }
                    break;
                case 'requestGitHubAuth':
                    try {
                        const session = await vscode.authentication.getSession('github', ['read:user'], { createIfNone: true });
                        if (session && session.accessToken) {
                            const cfg = vscode.workspace.getConfiguration('kora.copilot');
                            await cfg.update('githubToken', session.accessToken, vscode.ConfigurationTarget.Global);
                            await cfg.update('provider', 'github', vscode.ConfigurationTarget.Global);
                            vscode.window.showInformationMessage(`GitHub Account linked for Kora Copilot (${session.account ? session.account.label : 'authenticated'})!`);
                            this.syncProviderToWebview();
                        }
                    } catch (err) {
                        vscode.window.showWarningMessage(`Could not link GitHub account: ${err.message}. You can manually paste a token.`);
                    }
                    break;
                case 'updateGitHubConfig':
                    {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        if (message.token) await cfg.update('githubToken', message.token, vscode.ConfigurationTarget.Global);
                        if (message.model) await cfg.update('model', message.model, vscode.ConfigurationTarget.Global);
                        await cfg.update('provider', 'github', vscode.ConfigurationTarget.Global);
                        vscode.window.showInformationMessage(`GitHub Models configured (${message.model || 'gpt-4o-mini'})!`);
                        this.syncProviderToWebview();
                    }
                    break;
                case 'updateOllamaConfig':
                    {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        const envModel = (typeof process !== 'undefined' && process.env && process.env.OLLAMA_MODEL) ? process.env.OLLAMA_MODEL : '';
                        const envEndpoint = (typeof process !== 'undefined' && process.env && (process.env.OLLAMA_ENDPOINT || process.env.OLLAMA_HOST)) ? (process.env.OLLAMA_ENDPOINT || process.env.OLLAMA_HOST) : '';
                        const targetEndpoint = message.endpoint || cfg.get('endpoint') || envEndpoint || 'http://localhost:11434';
                        let effectiveModel = message.model;
                        if (!effectiveModel || effectiveModel === 'auto') {
                            try {
                                const res = await this.engine.ollamaService.resolveBestModel('chat', null, targetEndpoint);
                                if (res && res.model) effectiveModel = res.model;
                            } catch (e) {}
                        }
                        if (!effectiveModel || effectiveModel === 'auto') {
                            effectiveModel = envModel || 'qwen2.5-coder:7b';
                        }
                        if (message.endpoint !== undefined) await cfg.update('endpoint', message.endpoint, vscode.ConfigurationTarget.Global);
                        await cfg.update('model', effectiveModel, vscode.ConfigurationTarget.Global);
                        await cfg.update('provider', 'ollama', vscode.ConfigurationTarget.Global);
                        vscode.window.showInformationMessage(`Local Ollama configured (${effectiveModel})!`);
                        this.syncProviderToWebview();
                    }
                    break;
                case 'fetchOllamaModels':
                    {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        const rawCandidate = message.endpoint || cfg.get('endpoint');
                        
                        // Smart auto-detection of alive endpoint
                        const detection = await this.engine.ollamaService.autoDetectEndpoint(rawCandidate);
                        const endpoint = detection.endpoint;

                        // If user had a stale or cloud endpoint saved for Ollama, auto-heal to the discovered local endpoint
                        if (detection.found && cfg.get('provider') === 'ollama' && cfg.get('endpoint') !== endpoint) {
                            await cfg.update('endpoint', endpoint, vscode.ConfigurationTarget.Global);
                        }

                        try {
                            const resolution = await this.engine.ollamaService.resolveBestModel('chat', cfg.get('model'), endpoint);
                            const ranked = (resolution && resolution.allModels) || (await this.engine.ollamaService.getRankedModels('chat', endpoint));
                            if (resolution && resolution.model && (!cfg.get('model') || cfg.get('model') === 'gemma:2b' || cfg.get('model') === 'default' || resolution.status === 'fallback')) {
                                await cfg.update('model', resolution.model, vscode.ConfigurationTarget.Global);
                            }
                            if (this.view) {
                                this.view.webview.postMessage({
                                    type: 'ollamaModelsLoaded',
                                    models: ranked.map(m => m.name),
                                    detailedModels: ranked,
                                    activeModel: resolution.model || 'qwen2.5-coder:7b',
                                    resolution: resolution,
                                    status: resolution.status,
                                    endpoint: endpoint,
                                    detectedAlive: detection.found
                                });
                            }
                        } catch (e) {
                            console.error('Kora Ollama model discovery error:', e);
                            if (this.view) {
                                this.view.webview.postMessage({
                                    type: 'ollamaModelsLoaded',
                                    models: [],
                                    detailedModels: [],
                                    activeModel: 'qwen2.5-coder:7b',
                                    resolution: {
                                        model: null,
                                        status: 'error',
                                        message: e.message,
                                        recommendation: 'ollama serve'
                                    },
                                    status: 'error',
                                    endpoint: endpoint,
                                    detectedAlive: false
                                });
                            }
                        }
                    }
                    break;
                case 'getAvailableContextOptions':
                    {
                        this.syncProviderToWebview();
                        if (this.currentAnalysis && this.view) {
                            this.view.webview.postMessage({
                                type: 'activeFileUpdated',
                                analysis: this.currentAnalysis
                            });
                        }
                        const wsRoot = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0)
                            ? vscode.workspace.workspaceFolders[0].uri.fsPath
                            : process.cwd();
                        await this.engine.workspaceEngine.scanWorkspace([wsRoot]);
                        const files = [];
                        for (const [fPath, meta] of this.engine.workspaceEngine.indexedFiles) {
                            files.push({
                                label: meta.relPath,
                                path: fPath,
                                role: meta.role || 'file',
                                isDirectory: false
                            });
                        }
                        const folders = [];
                        for (const [folderPath, meta] of this.engine.workspaceEngine.indexedFolders) {
                            folders.push({
                                label: meta.relPath,
                                path: folderPath,
                                role: 'folder',
                                isDirectory: true
                            });
                        }
                        const activeDocName = (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document)
                            ? path.basename(vscode.window.activeTextEditor.document.fileName)
                            : '';
                        if (this.view) {
                            this.view.webview.postMessage({
                                type: 'availableContextOptions',
                                files: files,
                                folders: folders,
                                activeFile: activeDocName
                            });
                        }
                    }
                    break;
                case 'attachActiveFile':
                    {
                        const editor = vscode.window.activeTextEditor;
                        if (editor && editor.document && editor.document.fileName) {
                            await this.handleResolveDroppedAttachments([editor.document.fileName], []);
                        } else {
                            vscode.window.showWarningMessage('No active editor file is currently open to attach.');
                        }
                    }
                    break;
                case 'stopGeneration':
                    this.isGenerationStopped = true;
                    if (this.view) {
                        this.view.webview.postMessage({
                            type: 'generationStopped'
                        });
                    }
                    break;
                case 'saveAllSettings':
                    {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        const s = message.settings || {};
                        if (s.provider) await cfg.update('provider', s.provider, vscode.ConfigurationTarget.Global);
                        if (s.githubToken !== undefined) await cfg.update('githubToken', s.githubToken, vscode.ConfigurationTarget.Global);
                        if (s.openaiKey !== undefined) await cfg.update('openaiKey', s.openaiKey, vscode.ConfigurationTarget.Global);
                        if (s.geminiKey !== undefined) await cfg.update('geminiKey', s.geminiKey, vscode.ConfigurationTarget.Global);
                        if (s.anthropicKey !== undefined) await cfg.update('anthropicKey', s.anthropicKey, vscode.ConfigurationTarget.Global);
                        if (s.model !== undefined) await cfg.update('model', s.model, vscode.ConfigurationTarget.Global);
                        if (s.endpoint !== undefined) await cfg.update('endpoint', s.endpoint, vscode.ConfigurationTarget.Global);
                        vscode.window.showInformationMessage(`Kora Copilot settings saved! Active Provider: ${s.provider || cfg.get('provider')}`);
                        this.syncProviderToWebview();
                    }
                    break;
                case 'testConnection':
                    {
                        const cfg = vscode.workspace.getConfiguration('kora.copilot');
                        const s = message.settings || {};
                        const testConfig = {
                            provider: s.provider || cfg.get('provider', 'embedded'),
                            model: s.model || cfg.get('model', 'gpt-4o-mini'),
                            endpoint: s.endpoint || cfg.get('endpoint', ''),
                            githubToken: s.githubToken || cfg.get('githubToken', ''),
                            openaiKey: s.openaiKey || cfg.get('openaiKey', ''),
                            geminiKey: s.geminiKey || cfg.get('geminiKey', ''),
                            anthropicKey: s.anthropicKey || cfg.get('anthropicKey', ''),
                            apiKey: cfg.get('apiKey', '')
                        };
                        const result = await this.engine.testConnection(testConfig);
                        if (this.view) {
                            this.view.webview.postMessage({
                                type: 'testConnectionResult',
                                result: result
                            });
                        }
                    }
                    break;
            }
        });

        if (vscode.window.activeTextEditor) {
            try {
                const analysis = analyzeKoraDocument(vscode.window.activeTextEditor.document);
                if (analysis) this.updateActiveFile(analysis);
            } catch (e) {}
        }

        this.syncProviderToWebview();
        } catch (err) {
            console.error('Error in resolveWebviewView:', err);
            try {
                if (webviewView && webviewView.webview) {
                    webviewView.webview.html = this.getHtmlForWebview(webviewView.webview);
                }
            } catch (e) {}
        }
    }

    async planWorkspaceRefactor(targetPath = null, goal = 'Auto-Fix and refactor Kora invariants', scope = 'workspace') {
        const onProgress = (step, allSteps) => {
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'agentProgress',
                    step: step,
                    allSteps: allSteps
                });
            }
        };

        const targetFiles = (targetPath && scope === 'active') ? [targetPath] : [];
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: `Kora AI: Analyzing architecture and planning refactor...`,
                cancellable: false
            }, async () => {
                const plan = await this.engine.planWorkspaceRefactor(goal, {
                    targetFiles: targetFiles.length > 0 ? targetFiles : null,
                    activeFilePath: targetPath,
                    scope: scope
                }, onProgress);

                if (this.view) {
                    this.view.webview.postMessage({
                        type: 'refactorPlanCreated',
                        plan: plan
                    });
                }
            });
        } catch (err) {
            vscode.window.showErrorMessage(`Kora AI Plan Error: ${err.message}`);
        }
    }

    async triggerWorkspaceRefactor(targetPath = null, goal = 'Auto-Fix and refactor Kora invariants') {
        const cfg = vscode.workspace.getConfiguration('kora.copilot');
        const runtimeConfig = {
            provider: cfg.get('provider', 'embedded'),
            model: cfg.get('model', 'gemma:2b'),
            endpoint: cfg.get('endpoint', ''),
            githubToken: cfg.get('githubToken', '')
        };

        const onProgress = (step, allSteps) => {
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'agentProgress',
                    step: step,
                    allSteps: allSteps
                });
            }
        };

        const targetFiles = targetPath ? [targetPath] : [];

        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: `Kora AI: Analyzing workspace and planning refactor...`,
                cancellable: false
            }, async () => {
                const result = await this.engine.generateWorkspaceRefactor(goal, {
                    targetFiles: targetFiles.length > 0 ? targetFiles : null,
                    activeFilePath: targetPath,
                    config: runtimeConfig
                }, onProgress);

                this.activeProposal = result.rawProposal;

                if (this.view) {
                    this.view.webview.postMessage({
                        type: 'proposalCreated',
                        proposal: result.proposal,
                        filesCount: result.filesCount
                    });
                }

                if (result.filesCount > 0) {
                    vscode.window.showInformationMessage(`Kora AI: Prepared changes for ${result.filesCount} file(s). Review diffs in Kora AI panel.`);
                } else {
                    vscode.window.showInformationMessage(`Kora AI: No rule violations found. All workspace invariants satisfied!`);
                    if (this.view) {
                        this.view.webview.postMessage({
                            type: 'assistantAnswer',
                            answer: `**Workspace Invariants Satisfied**\n\nNo violations or necessary refactors found for: *${goal}*. All scanned components strictly comply with Clean Architecture rules.`,
                            provider: 'embedded',
                            model: 'RefactorAgent'
                        });
                    }
                }
            });
        } catch (err) {
            vscode.window.showErrorMessage(`Kora AI Refactor Error: ${err.message}`);
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'assistantError',
                    error: err.message,
                    originalPrompt: goal
                });
            }
        }
    }

    async auditWorkspaceRules() {
        const auditText = this.engine.auditWorkspace();
        if (this.view) {
            this.view.webview.postMessage({
                type: 'auditResult',
                audit: {
                    passed: !auditText.includes('Rule Violation'),
                    violations: (auditText.match(/Violation/g) || []).length,
                    findings: [],
                    rawText: auditText
                }
            });
            this.view.webview.postMessage({
                type: 'assistantAnswer',
                answer: auditText,
                provider: 'embedded',
                model: 'RuleEngine'
            });
        }
    }

    async acceptActiveProposal(proposalId = null) {
        try {
            const result = await this.engine.reviewManager.applyApprovedChanges(proposalId);
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'proposalApplied',
                    proposalId: proposalId || (this.activeProposal ? this.activeProposal.id : null),
                    appliedCount: result.appliedCount
                });
            }
            vscode.window.showInformationMessage(`Successfully applied changes to ${result.appliedCount} file(s)!`);
            if (vscode.window.activeTextEditor) {
                vscode.commands.executeCommand('kora.checkFile', vscode.window.activeTextEditor.document.fileName);
            }
        } catch (err) {
            vscode.window.showErrorMessage(`Failed to apply changes: ${err.message}`);
        }
    }

    async rejectActiveProposal(proposalId = null) {
        this.engine.reviewManager.rejectAll(proposalId);
        if (this.view) {
            this.view.webview.postMessage({
                type: 'proposalRejected',
                proposalId: proposalId || (this.activeProposal ? this.activeProposal.id : null)
            });
        }
        vscode.window.showInformationMessage('AI proposed changes were rejected.');
    }

    setFileApprovalStatus(proposalId, filePath, status) {
        const updated = this.engine.reviewManager.setFileStatus(proposalId, filePath, status);
        if (this.view && updated) {
            this.view.webview.postMessage({
                type: 'proposalUpdated',
                proposal: this.engine.reviewManager.serializeProposal(proposalId)
            });
        }
    }

    updateActiveFile(analysis) {
        this.currentAnalysis = analysis;
        if (this.view) {
            this.view.webview.postMessage({
                type: 'fileUpdated',
                analysis: analysis
            });
        }
    }

    updateDiagnostics(diagnostics) {
        this.currentDiagnostics = diagnostics;
        if (this.view) {
            this.view.webview.postMessage({
                type: 'diagnosticsUpdated',
                diagnostics: diagnostics.map(d => ({
                    message: d.message,
                    severity: d.severity,
                    line: d.range.start.line + 1
                }))
            });
        }
    }

    displayAudit(audit) {
        if (this.view) {
            this.view.webview.postMessage({
                type: 'auditResult',
                audit: audit
            });
        }
    }

    triggerExplanation(document) {
        const text = document.getText();
        const analysis = analyzeKoraDocument(document);

        let explanation = `### Kora Code & Architecture Analysis: \`${analysis.fileName}\`\n\n`;
        explanation += `- **Classification**: **${analysis.classification}**\n`;
        explanation += `- **Runnable**: ${analysis.isRunnable ? 'Yes (Ready to run directly)' : 'No (Library / Service component)'}\n`;
        explanation += `- **Total Lines**: ${analysis.lineCount}\n`;
        explanation += `- **Declared Symbols**: ${analysis.symbols.length > 0 ? analysis.symbols.map(s => `\`${s.name} (${s.kind})\``).join(', ') : 'None'}\n`;
        explanation += `- **Imported Modules**: ${analysis.imports.length > 0 ? analysis.imports.map(i => `\`${i.path}\``).join(', ') : 'None'}\n\n`;

        if (analysis.isRunnable) {
            explanation += `> **Execution Advice**: This file contains an entrypoint or test runner. Click **Run Entrypoint** or use \`kora run "${analysis.fileName}"\`.\n\n`;
        } else {
            explanation += `> **Clean Architecture Advice**: In Kora Clean Architecture (\`.agent/rules/04-clean-architecture.md\`), domain logic is organized into bounded contexts. Inject this service into a controller or mount via \`Router\` in \`src/main.kora\`.\n\n`;
        }

        explanation += `#### Layer Breakdown:\n`;
        if (text.includes('@table') || text.includes('@entity')) {
            explanation += `- **Data Layer (@table)**: Persisted database entity (\`.agent/rules/07-database-multi-engine-orm.md\`).\n`;
        }
        if (text.includes('class') && text.includes('init(')) {
            explanation += `- **Service / DI Layer**: State and logic with acyclic dependency injection (\`init\`).\n`;
        }
        if (text.includes('channel') || text.includes('actor') || text.includes('spawn')) {
            explanation += `- **Concurrency Layer**: Executes over Tokio worker threads with mailboxes (\`.agent/rules/08-concurrency-actors-streams.md\`).\n`;
        }

        if (this.view) {
            this.view.webview.postMessage({
                type: 'explanationResult',
                explanation: explanation
            });
        }
    }

    async resolveContextMentions(prompt) {
        const mentions = [];
        const attachments = [];

        // 1. @selection
        if (/@selection\b/i.test(prompt)) {
            mentions.push('@selection');
            const editor = vscode.window.activeTextEditor;
            if (editor && !editor.selection.isEmpty) {
                const selectedText = editor.document.getText(editor.selection);
                const relFile = vscode.workspace.asRelativePath ? vscode.workspace.asRelativePath(editor.document.uri) : path.basename(editor.document.fileName);
                attachments.push({
                    type: 'selection',
                    title: `@selection in ${relFile} (Lines ${editor.selection.start.line + 1}-${editor.selection.end.line + 1})`,
                    content: selectedText
                });
            } else if (editor) {
                attachments.push({
                    type: 'selection',
                    title: `@selection in ${path.basename(editor.document.fileName)} (Full File)`,
                    content: editor.document.getText().slice(0, 5000)
                });
            }
        }

        // 2. @problems
        if (/@problems\b/i.test(prompt)) {
            mentions.push('@problems');
            const allDiags = vscode.languages.getDiagnostics();
            const problemsList = [];
            for (const [uri, diags] of allDiags) {
                const relPath = vscode.workspace.asRelativePath ? vscode.workspace.asRelativePath(uri) : (uri.fsPath || '');
                for (const d of diags) {
                    const sev = d.severity === 0 ? 'Error' : 'Warning';
                    problemsList.push(`- [${sev}] ${relPath}:${d.range.start.line + 1}: ${d.message}`);
                }
            }
            attachments.push({
                type: 'problems',
                title: `@problems (${problemsList.length} issue(s) found)`,
                content: problemsList.slice(0, 30).join('\n') || 'No compiler errors or warnings found in workspace.'
            });
        }

        // 3. @terminal
        if (/@terminal\b/i.test(prompt)) {
            mentions.push('@terminal');
            const terms = vscode.window.terminals;
            const termInfo = terms.map(t => `- Terminal: "${t.name}" (Active: ${t === vscode.window.activeTerminal})`).join('\n');
            attachments.push({
                type: 'terminal',
                title: `@terminal (Active terminals)`,
                content: termInfo || 'No active terminal sessions.'
            });
        }

        // 4. @workspace
        if (/@workspace\b/i.test(prompt)) {
            mentions.push('@workspace');
            const wsRoot = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0)
                ? vscode.workspace.workspaceFolders[0].uri.fsPath
                : process.cwd();
            if (this.engine.workspaceEngine.indexedFiles.size === 0) {
                await this.engine.workspaceEngine.scanWorkspace([wsRoot]);
            }
            const context = this.engine.workspaceEngine.buildContextForQuery(prompt, { workspaceRoots: [wsRoot] });
            attachments.push({
                type: 'workspace',
                title: `@workspace (${this.engine.workspaceEngine.indexedFiles.size} indexed files)`,
                content: this.engine.workspaceEngine.formatContextForPrompt(context)
            });
        }

        // 5. @file[: ]([^\s]+)
        const fileRegex = /@file(?::|\s+)([^\s]+)/gi;
        let fm;
        while ((fm = fileRegex.exec(prompt)) !== null) {
            const rawTarget = fm[1].replace(/['"]/g, '');
            mentions.push(`@file:${rawTarget}`);
            const wsRoot = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0)
                ? vscode.workspace.workspaceFolders[0].uri.fsPath
                : process.cwd();
            const fullPath = path.isAbsolute(rawTarget) ? rawTarget : path.resolve(wsRoot, rawTarget);
            if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
                try {
                    const content = fs.readFileSync(fullPath, 'utf8');
                    attachments.push({
                        type: 'file',
                        title: `@file ${rawTarget}`,
                        path: fullPath,
                        content: content.slice(0, 8000)
                    });
                } catch (e) {}
            }
        }

        // 6. @folder[: ]([^\s]+)
        const folderRegex = /@folder(?::|\s+)([^\s]+)/gi;
        let fdm;
        while ((fdm = folderRegex.exec(prompt)) !== null) {
            const rawFolder = fdm[1].replace(/['"]/g, '');
            mentions.push(`@folder:${rawFolder}`);
            const wsRoot = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0)
                ? vscode.workspace.workspaceFolders[0].uri.fsPath
                : process.cwd();
            const fullFolder = path.isAbsolute(rawFolder) ? rawFolder : path.resolve(wsRoot, rawFolder);
            if (fs.existsSync(fullFolder) && fs.statSync(fullFolder).isDirectory()) {
                try {
                    const files = fs.readdirSync(fullFolder);
                    attachments.push({
                        type: 'folder',
                        title: `@folder ${rawFolder}`,
                        content: `Files in directory ${rawFolder}:\n` + files.slice(0, 40).map(f => `- ${f}`).join('\n')
                    });
                } catch (e) {}
            }
        }

        return { mentions, attachments };
    }

    async handleResolveDroppedAttachments(rawPaths = [], droppedItems = []) {
        const wsRoot = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0)
            ? vscode.workspace.workspaceFolders[0].uri.fsPath
            : process.cwd();

        for (const raw of (rawPaths || [])) {
            if (!raw || typeof raw !== 'string') continue;
            let clean = raw.trim().replace(/^file:\/\//, '');
            try {
                clean = decodeURIComponent(clean);
            } catch (e) {}

            const fullPath = path.isAbsolute(clean) ? clean : path.resolve(wsRoot, clean);
            if (!fs.existsSync(fullPath)) continue;

            // Deduplicate if already attached by path
            const existing = Array.from(this.activeAttachments.values()).find(a => a.path === fullPath);
            if (existing) continue;

            const stat = fs.statSync(fullPath);
            const baseName = path.basename(fullPath);
            const relPath = path.relative(wsRoot, fullPath) || baseName;

            if (stat.isDirectory()) {
                const filesList = [];
                this._scanDirectoryRecursive(fullPath, fullPath, filesList, 80);
                const treeStr = this._buildDirectoryTreeString(fullPath, filesList.slice(0, 40));

                let keyContents = '';
                let readCount = 0;
                for (const f of filesList) {
                    if (readCount >= 5) break;
                    if (f.ext === '.kora' || f.ext === '.kr' || f.name === 'kora.toml' || f.ext === '.json' || f.ext === '.md') {
                        try {
                            const content = fs.readFileSync(f.fullPath, 'utf8');
                            keyContents += `\n--- File: ${f.relPath} ---\n${content.slice(0, 2000)}\n`;
                            readCount++;
                        } catch (e) {}
                    }
                }

                const id = 'folder_' + Date.now() + '_' + Math.random().toString(36).slice(2, 6);
                this.activeAttachments.set(id, {
                    id,
                    type: 'folder',
                    isDirectory: true,
                    name: baseName,
                    relPath: relPath,
                    path: fullPath,
                    fileCount: filesList.length,
                    tree: treeStr,
                    summary: `Directory ${relPath} containing ${filesList.length} file(s)`,
                    content: keyContents
                });
            } else if (stat.isFile()) {
                let content = '';
                try {
                    content = fs.readFileSync(fullPath, 'utf8');
                } catch (e) {}
                const id = 'file_' + Date.now() + '_' + Math.random().toString(36).slice(2, 6);
                this.activeAttachments.set(id, {
                    id,
                    type: 'file',
                    isDirectory: false,
                    name: baseName,
                    relPath: relPath,
                    path: fullPath,
                    size: stat.size,
                    lineCount: content.split('\n').length,
                    content: content.slice(0, 10000),
                    summary: `File ${relPath} (${(stat.size / 1024).toFixed(1)} KB, ${content.split('\n').length} lines)`
                });
            }
        }

        for (const item of (droppedItems || [])) {
            if (!item || !item.name) continue;
            const id = 'drop_' + Date.now() + '_' + Math.random().toString(36).slice(2, 6);
            this.activeAttachments.set(id, {
                id,
                type: item.isDirectory ? 'folder' : 'file',
                isDirectory: Boolean(item.isDirectory),
                name: item.name,
                path: item.path || item.name,
                size: item.size || 0,
                content: (item.content || '').slice(0, 10000),
                summary: `${item.isDirectory ? 'Directory' : 'File'} ${item.name}`
            });
        }

        if (this.view) {
            this.view.webview.postMessage({
                type: 'attachmentsResolved',
                attachments: Array.from(this.activeAttachments.values())
            });
        }
    }

    _scanDirectoryRecursive(currentDir, rootDir, results, maxFiles = 80) {
        if (results.length >= maxFiles) return;
        try {
            const entries = fs.readdirSync(currentDir, { withFileTypes: true });
            for (const entry of entries) {
                if (results.length >= maxFiles) break;
                if (entry.name.startsWith('.') || entry.name === 'node_modules' || entry.name === 'target') continue;
                const fullPath = path.join(currentDir, entry.name);
                const relPath = path.relative(rootDir, fullPath);
                if (entry.isDirectory()) {
                    this._scanDirectoryRecursive(fullPath, rootDir, results, maxFiles);
                } else if (entry.isFile()) {
                    results.push({
                        name: entry.name,
                        fullPath,
                        relPath,
                        ext: path.extname(entry.name).toLowerCase()
                    });
                }
            }
        } catch (e) {}
    }

    _buildDirectoryTreeString(rootDir, filesList) {
        const lines = [`${path.basename(rootDir)}/`];
        for (const f of filesList) {
            lines.push(`  ├── ${f.relPath}`);
        }
        return lines.join('\n');
    }

    async answerKoraPrompt(prompt, scope = 'workspace') {
        const doc = vscode.window.activeTextEditor ? vscode.window.activeTextEditor.document : null;
        const config = vscode.workspace.getConfiguration('kora.copilot');

        let provider = config.get('provider', 'embedded');
        let githubToken = config.get('githubToken', '');
        let openaiKey = config.get('openaiKey', '');
        let geminiKey = config.get('geminiKey', '');
        let anthropicKey = config.get('anthropicKey', '');
        let model = config.get('model', 'gpt-4o-mini');
        let endpoint = config.get('endpoint', '');
        let apiKey = config.get('apiKey', '');

        if (provider === 'github' && !githubToken && !apiKey && !process.env.GITHUB_TOKEN && !process.env.GH_TOKEN) {
            try {
                const session = await vscode.authentication.getSession('github', ['read:user'], { createIfNone: false });
                if (session && session.accessToken) {
                    githubToken = session.accessToken;
                }
            } catch (e) {}
        }

        const runtimeConfig = {
            provider,
            githubToken,
            openaiKey,
            geminiKey,
            anthropicKey,
            model,
            endpoint,
            apiKey
        };

        this.isGenerationStopped = false;

        const onProgress = (step, allSteps) => {
            if (this.isGenerationStopped) return;
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'agentProgress',
                    step: step,
                    allSteps: allSteps
                });
            }
        };

        const intent = this.engine.intentEngine.classify(prompt || '', {
            attachments: Array.from(this.activeAttachments.values()),
            activeDoc: doc
        });

        const lower = (prompt || '').toLowerCase();
        const isExplicitRefactor = (lower.startsWith('/fix') || lower.startsWith('/refactor') || scope === 'refactor') && !intent.isReadOnly;

        if (isExplicitRefactor) {
            await this.planWorkspaceRefactor(doc ? doc.fileName : null, prompt, scope);
            return;
        }

        const onChunk = (chunk, fullText) => {
            if (this.isGenerationStopped) return;
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'streamChunk',
                    chunk: chunk,
                    fullText: fullText
                });
            }
        };

        try {
            const contextData = await this.resolveContextMentions(prompt);
            let enrichedPrompt = prompt;
            if (contextData.attachments.length > 0) {
                const blocks = contextData.attachments.map(att => `[Context Reference: ${att.title}]\n\`\`\`\n${att.content}\n\`\`\``).join('\n\n');
                enrichedPrompt = `${prompt}\n\n${blocks}`;
            }

            const answer = await this.engine.answerPrompt(enrichedPrompt, doc, runtimeConfig, {
                scope,
                onProgress,
                onChunk,
                attachments: Array.from(this.activeAttachments.values())
            });

            if (this.isGenerationStopped) return;

            if (this.view) {
                this.view.webview.postMessage({
                    type: 'assistantAnswer',
                    answer: answer,
                    provider: provider,
                    model: model,
                    mentions: contextData.mentions
                });
            }
        } catch (err) {
            if (this.view) {
                this.view.webview.postMessage({
                    type: 'assistantError',
                    error: err.message,
                    originalPrompt: prompt
                });
            }
            vscode.window.showErrorMessage(`Kora AI Error: ${err.message}`);
        }
    }

    syncProviderToWebview() {
        if (this.view) {
            const config = vscode.workspace.getConfiguration('kora.copilot');
            const provider = config.get('provider', 'embedded');
            const envModel = (typeof process !== 'undefined' && process.env && process.env.OLLAMA_MODEL) ? process.env.OLLAMA_MODEL : '';
            const envEndpoint = (typeof process !== 'undefined' && process.env && (process.env.OLLAMA_ENDPOINT || process.env.OLLAMA_HOST)) ? (process.env.OLLAMA_ENDPOINT || process.env.OLLAMA_HOST) : '';
            
            let rawEndpoint = config.get('endpoint', '');
            if (provider === 'ollama' && rawEndpoint) {
                if (isCloudEndpoint(rawEndpoint)) {
                    rawEndpoint = '';
                }
            }
            const effectiveEndpoint = rawEndpoint || envEndpoint || 'http://localhost:11434';
            let effectiveModel = config.get('model', '');
            if (provider === 'ollama' && (!effectiveModel || effectiveModel === 'gemma:2b' || effectiveModel === 'default')) {
                effectiveModel = envModel || 'qwen2.5-coder:7b';
            } else if (!effectiveModel) {
                effectiveModel = envModel || 'qwen2.5-coder:7b';
            }

            this.view.webview.postMessage({
                type: 'syncConfig',
                provider: provider,
                model: effectiveModel,
                hasGitHubToken: Boolean(config.get('githubToken') || process.env.GITHUB_TOKEN),
                endpoint: effectiveEndpoint,
                envModel: envModel,
                settings: {
                    provider: provider,
                    githubToken: config.get('githubToken', ''),
                    openaiKey: config.get('openaiKey', ''),
                    geminiKey: config.get('geminiKey', ''),
                    anthropicKey: config.get('anthropicKey', ''),
                    model: effectiveModel,
                    endpoint: effectiveEndpoint
                }
            });
        }
    }

    switchTab(tabName) {
        if (!tabName) return;
        let effective = String(tabName).toLowerCase().trim();
        if (effective.startsWith('tab-')) effective = effective.substring(4);
        if (effective === 'learning') effective = 'learn';
        if (effective === 'architecture') effective = 'arch';
        if (effective === 'inspector') effective = 'copilot';
        if (effective === 'chat' || effective === 'agent') effective = 'ask';
        if (effective === 'example') effective = 'examples';
        if (effective === 'setting') effective = 'settings';
        if (this.view) {
            this.view.webview.postMessage({
                type: 'switchTab',
                tab: effective
            });
        }
    }

    displayWalkthrough(walkthrough) {
        if (this.view) {
            this.view.webview.postMessage({
                type: 'walkthroughResult',
                walkthrough: walkthrough
            });
        }
    }

    analyzeWorkspaceArchitecture(workspaceRoot) {
        if (!workspaceRoot || !fs.existsSync(workspaceRoot)) return;
        const counts = { controllers: 0, services: 0, repositories: 0, entities: 0, models: 0, tests: 0 };
        const filesFound = [];

        function scan(dir) {
            try {
                const entries = fs.readdirSync(dir, { withFileTypes: true });
                for (const e of entries) {
                    const full = path.join(dir, e.name);
                    if (e.isDirectory() && !e.name.startsWith('.') && e.name !== 'target' && e.name !== 'node_modules' && e.name !== 'dist') {
                        scan(full);
                    } else if (e.isFile() && (e.name.endsWith('.kora') || e.name.endsWith('.kr'))) {
                        const rel = path.relative(workspaceRoot, full);
                        filesFound.push(rel);
                        if (rel.includes('controller') || e.name.includes('controller')) counts.controllers++;
                        else if (rel.includes('service') || e.name.includes('service')) counts.services++;
                        else if (rel.includes('repository') || e.name.includes('repository')) counts.repositories++;
                        else if (rel.includes('entity') || e.name.includes('entity')) counts.entities++;
                        else if (rel.includes('model') || e.name.includes('model')) counts.models++;
                        else if (rel.includes('test') || e.name.includes('test')) counts.tests++;
                    }
                }
            } catch (err) {}
        }
        scan(workspaceRoot);

        if (this.view) {
            this.view.webview.postMessage({
                type: 'architectureResult',
                architecture: { counts, filesCount: filesFound.length, files: filesFound.slice(0, 30) }
            });
        }
    }

    getHtmlForWebview(webview) {
        const nonce = getNonce();
        const catalog = getDiscoveredCatalog(this.extensionUri);
        return getWebviewHtml(webview, this.extensionUri, catalog, nonce);
    }
}

module.exports = {
    KoraAssistantViewProvider,
    analyzeKoraDocument
};
