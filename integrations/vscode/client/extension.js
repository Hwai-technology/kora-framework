const vscode = require('vscode');
const { exec } = require('child_process');

let diagnosticCollection;
let validationTimeouts = new Map();

/**
 * Activates the Kora Language Extension.
 * Provides real-time compiler diagnostics, intelligent autocompletion,
 * and contextual hover documentation for `.kora` and `.kr` files.
 */
function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('kora');
    context.subscriptions.push(diagnosticCollection);

    // 1. Real-time Syntax & Type Checking (with Debounce)
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(doc => scheduleValidation(doc, 0)),
        vscode.workspace.onDidChangeTextDocument(e => scheduleValidation(e.document, 350)),
        vscode.workspace.onDidSaveTextDocument(doc => scheduleValidation(doc, 0)),
        vscode.workspace.onDidCloseTextDocument(doc => {
            diagnosticCollection.delete(doc.uri);
            validationTimeouts.delete(doc.uri.toString());
        })
    );

    if (vscode.window.activeTextEditor) {
        scheduleValidation(vscode.window.activeTextEditor.document, 0);
    }

    // 2. Intelligent Autocompletion Provider (Ctrl+Space, typing, and trigger characters)
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'kora',
        {
            provideCompletionItems(document, position, token, context) {
                const completions = [];

                // A. Kora Keywords
                const keywords = [
                    { label: 'model', detail: 'Pure immutable Data Transfer Object / Domain Model', kind: vscode.CompletionItemKind.Class, snippet: 'model ${1:Name} {\n\t${2:field}: ${3:String}\n}' },
                    { label: 'class', detail: 'Stateful service class with Dependency Injection', kind: vscode.CompletionItemKind.Class, snippet: 'class ${1:Name}Service implements ${2:Interface} {\n\tdb: Database\n\n\tinit(db: Database) {\n\t\tthis.db = db\n\t}\n\n\tfn ${3:execute}() -> ${4:Unit} {\n\t\t${0}\n\t}\n}' },
                    { label: 'interface', detail: 'Behavioral contract interface', kind: vscode.CompletionItemKind.Interface, snippet: 'interface ${1:Name} {\n\tfn ${2:method}(${3:param}: ${4:Type}) -> ${5:ReturnType}\n}' },
                    { label: 'enum', detail: 'Type-safe enumerated variants', kind: vscode.CompletionItemKind.Enum, snippet: 'enum ${1:Name} {\n\t${2:Active},\n\t${3:Inactive}\n}' },
                    { label: 'actor', detail: 'Isolated stateful actor concurrency construct', kind: vscode.CompletionItemKind.Class, snippet: 'class ${1:Name}Actor {\n\tvar state: ${2:Int}\n\n\tinit() {\n\t\tthis.state = 0\n\t}\n\n\tfn handle(msg: ${3:String}) -> ${4:Int} {\n\t\t${0}\n\t}\n}' },
                    { label: 'let', detail: 'Declare an immutable binding', kind: vscode.CompletionItemKind.Keyword, snippet: 'let ${1:name} = ${2:value}' },
                    { label: 'var', detail: 'Declare a mutable binding', kind: vscode.CompletionItemKind.Keyword, snippet: 'var ${1:name} = ${2:value}' },
                    { label: 'fn', detail: 'Function declaration', kind: vscode.CompletionItemKind.Function, snippet: 'fn ${1:name}(${2:params}) -> ${3:Unit} {\n\t${0}\n}' },
                    { label: 'async', detail: 'Asynchronous function modifier', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'await', detail: 'Await an asynchronous operation', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'spawn', detail: 'Spawn a concurrent background task', kind: vscode.CompletionItemKind.Function, snippet: 'spawn(async () => {\n\t${0}\n})' },
                    { label: 'channel', detail: 'Create a non-blocking MPSC channel', kind: vscode.CompletionItemKind.Function, snippet: 'channel<${1:Type}>(${2:100})' },
                    { label: 'broadcast', detail: 'Create a multi-consumer broadcast channel', kind: vscode.CompletionItemKind.Function, snippet: 'broadcast<${1:Type}>(${2:100})' },
                    { label: 'match', detail: 'Exhaustive pattern matching expression', kind: vscode.CompletionItemKind.Keyword, snippet: 'match ${1:expr} {\n\tSome(${2:val}) => ${3:val},\n\tNone => ${4:fallback},\n}' },
                    { label: 'return', detail: 'Return from function', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'implements', detail: 'Interface conformance clause', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'import', detail: 'Import symbols from another module', kind: vscode.CompletionItemKind.Keyword, snippet: 'import { ${1:Symbol} } from "${2:./module}"' },
                    { label: 'pub', detail: 'Public visibility modifier', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'get', detail: 'Register an HTTP GET route', kind: vscode.CompletionItemKind.Method, snippet: 'get("${1:/path}", (${2:svc}) => {\n\treturn ${0}\n})' },
                    { label: 'post', detail: 'Register an HTTP POST route', kind: vscode.CompletionItemKind.Method, snippet: 'post("${1:/path}", (request: Request, ${2:svc}) => {\n\treturn ${0}\n})' },
                    { label: 'put', detail: 'Register an HTTP PUT route', kind: vscode.CompletionItemKind.Method, snippet: 'put("${1:/path}", (request: Request, ${2:svc}) => {\n\treturn ${0}\n})' },
                    { label: 'delete', detail: 'Register an HTTP DELETE route', kind: vscode.CompletionItemKind.Method, snippet: 'delete("${1:/path}", (${2:svc}) => {\n\treturn ${0}\n})' },
                    { label: 'send', detail: 'Send message over channel', kind: vscode.CompletionItemKind.Method, snippet: 'send(${1:msg})' },
                    { label: 'recv', detail: 'Receive message from channel', kind: vscode.CompletionItemKind.Method, snippet: 'recv()' },
                    { label: 'tell', detail: 'Fire-and-forget actor message', kind: vscode.CompletionItemKind.Method, snippet: 'tell(${1:msg})' },
                    { label: 'ask', detail: 'Request-response actor query', kind: vscode.CompletionItemKind.Method, snippet: 'ask(${1:msg})' }
                ];

                for (const kw of keywords) {
                    const item = new vscode.CompletionItem(kw.label, kw.kind);
                    item.detail = `Kora: ${kw.detail}`;
                    if (kw.snippet) {
                        item.insertText = new vscode.SnippetString(kw.snippet);
                    }
                    completions.push(item);
                }

                // B. Built-in Types
                const types = [
                    { label: 'Int', detail: '64-bit signed integer' },
                    { label: 'Float', detail: '64-bit IEEE floating-point number' },
                    { label: 'Bool', detail: 'Boolean truth value (true | false)' },
                    { label: 'String', detail: 'UTF-8 encoded heap string' },
                    { label: 'Unit', detail: 'Void / empty return unit type ()' },
                    { label: 'Option', detail: 'Nullable wrapper: Some(T) | None', snippet: 'Option<${1:Type}>' },
                    { label: 'Result', detail: 'Error handling wrapper: Ok(T) | Err(E)', snippet: 'Result<${1:Type}, ${2:String}>' },
                    { label: 'Channel', detail: 'MPSC async message channel', snippet: 'Channel<${1:Type}>' },
                    { label: 'BroadcastChannel', detail: 'Multi-consumer broadcast channel', snippet: 'BroadcastChannel<${1:Type}>' },
                    { label: 'TaskGroup', detail: 'Structured concurrency task group', snippet: 'TaskGroup<${1:Type}>' },
                    { label: 'Database', detail: 'Connection pool & ORM interface' },
                    { label: 'Request', detail: 'Incoming HTTP request payload' },
                    { label: 'Response', detail: 'Outgoing HTTP response payload' },
                    { label: 'App', detail: 'Kora server application instance' }
                ];

                for (const ty of types) {
                    const item = new vscode.CompletionItem(ty.label, vscode.CompletionItemKind.TypeParameter);
                    item.detail = `Kora Type: ${ty.detail}`;
                    if (ty.snippet) {
                        item.insertText = new vscode.SnippetString(ty.snippet);
                    }
                    completions.push(item);
                }

                // C. ORM & Validation Decorators (@...)
                const decorators = [
                    { label: '@entity', detail: 'Mark model as a database persistent entity' },
                    { label: '@table', detail: 'Map model to a specific SQL table name', snippet: '@table("${1:table_name}")' },
                    { label: '@primary', detail: 'Mark field as primary key' },
                    { label: '@autoIncrement', detail: 'Auto-incrementing integer sequence' },
                    { label: '@unique', detail: 'Enforce unique database constraint' },
                    { label: '@index', detail: 'Create database index for fast query lookups' },
                    { label: '@required', detail: 'Enforce non-empty validation constraint' },
                    { label: '@email', detail: 'Enforce RFC email validation' },
                    { label: '@min', detail: 'Enforce minimum numeric or string length value', snippet: '@min(${1:1})' },
                    { label: '@max', detail: 'Enforce maximum numeric or string length value', snippet: '@max(${1:100})' },
                    { label: '@default', detail: 'Default database column value', snippet: '@default("${1:val}")' }
                ];

                for (const dec of decorators) {
                    const item = new vscode.CompletionItem(dec.label, vscode.CompletionItemKind.Property);
                    item.detail = `Kora Decorator: ${dec.detail}`;
                    if (dec.snippet) {
                        item.insertText = new vscode.SnippetString(dec.snippet);
                    }
                    completions.push(item);
                }

                // D. Dynamic Document Symbol Scraping (methods, classes, models in file)
                const docText = document.getText();
                const symbolRegex = /\b(model|class|interface|enum|fn)\s+([A-Za-z0-9_]+)/g;
                let smatch;
                while ((smatch = symbolRegex.exec(docText)) !== null) {
                    const kindStr = smatch[1];
                    const name = smatch[2];
                    const kind = kindStr === 'fn' ? vscode.CompletionItemKind.Function : vscode.CompletionItemKind.Class;
                    const item = new vscode.CompletionItem(name, kind);
                    item.detail = `Defined in this file (${kindStr})`;
                    completions.push(item);
                }

                return completions;
            }
        },
        '.', '@' // Trigger characters
    );
    context.subscriptions.push(completionProvider);

    // 3. Hover Documentation Provider
    const hoverProvider = vscode.languages.registerHoverProvider('kora', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);

            const docs = {
                model: '**Kora `model`**\n\nPure, immutable Data Transfer Object (DTO) or database entity. Models never hold behavior or stateful methods and are compiled with zero-GC overhead.',
                class: '**Kora `class`**\n\nService or Controller class with compile-time Dependency Injection (DI). All parameters in `init(...)` are resolved automatically.',
                interface: '**Kora `interface`**\n\nPure behavioral contract defining method signatures. Enforced statically by the Kora compiler.',
                actor: '**Kora `actor`**\n\nIsolated stateful concurrency unit executing on Tokio worker threads with non-blocking mailboxes (`tell` / `ask`).',
                channel: '**Kora `channel<T>(capacity)`**\n\nNon-blocking MPSC channel for asynchronous message passing.',
                broadcast: '**Kora `broadcast<T>(capacity)`**\n\nMulti-producer, multi-consumer broadcast channel for pub/sub events.',
                spawn: '**Kora `spawn(async () => { ... })`**\n\nSpawns an asynchronous concurrent task executed on high-performance native threads.',
                null: '**Kora `null`**\n\nSingle-null design: represents absent optional values (`Option<T>`). Note: `undefined` is strictly forbidden in Kora (`E2005`).'
            };

            if (docs[word]) {
                return new vscode.Hover(new vscode.MarkdownString(docs[word]));
            }
            return null;
        }
    });
    context.subscriptions.push(hoverProvider);
}

/**
 * Debounced wrapper to trigger compilation and diagnostics.
 */
function scheduleValidation(document, delayMs) {
    if (!isKoraFile(document)) {
        return;
    }

    const key = document.uri.toString();
    if (validationTimeouts.has(key)) {
        clearTimeout(validationTimeouts.get(key));
    }

    if (delayMs === 0) {
        validateKoraDocument(document);
    } else {
        const timeout = setTimeout(() => {
            validateKoraDocument(document);
            validationTimeouts.delete(key);
        }, delayMs);
        validationTimeouts.set(key, timeout);
    }
}

/**
 * Checks whether a document is a Kora source file (`.kora` or `.kr`).
 */
function isKoraFile(document) {
    return document.languageId === 'kora' ||
           document.fileName.endsWith('.kora') ||
           document.fileName.endsWith('.kr');
}

/**
 * Validates a Kora document by executing `kora check`.
 */
function validateKoraDocument(document) {
    if (!isKoraFile(document)) {
        return;
    }

    const filePath = document.fileName;
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
    validationTimeouts.clear();
}

module.exports = { activate, deactivate };
