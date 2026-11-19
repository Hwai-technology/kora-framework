const fs = require('fs');
const path = require('path');
const vscode = require('vscode');

/**
 * Official 23 Examples Metadata Catalog
 */
const EXAMPLES_CATALOG = [
    { slug: "01_hello_world", name: "Hello World & Baseline", category: "Core", difficulty: "Beginner", desc: "Pure models, enums, pattern matching, OrderProcessor and Clean Architecture baseline" },
    { slug: "02_clean_architecture_users", name: "Clean Architecture Users", category: "Architecture", difficulty: "Beginner", desc: "Class-based DI, User controller, service, repository interface, and validation" },
    { slug: "03_crud_products_api", name: "CRUD Products API", category: "Web & API", difficulty: "Intermediate", desc: "RESTful HTTP endpoints, paginated lists, in-memory repository, and CRUD mutations" },
    { slug: "04_mysql_database_orm", name: "MySQL Database ORM", category: "Database", difficulty: "Intermediate", desc: "Type-safe SQL queries, ACID transactions, connection pool, and NoSQL audit trail" },
    { slug: "05_jwt_auth_argon2", name: "JWT Auth & Argon2", category: "Security", difficulty: "Intermediate", desc: "Argon2 password hashing, cryptographically signed JWT tokens, and RBAC guards" },
    { slug: "06_realtime_websockets", name: "Realtime WebSockets", category: "Realtime", difficulty: "Intermediate", desc: "WebSocket connection hub, room presence, broadcast channels, and event subscriptions" },
    { slug: "07_async_actors_concurrency", name: "Async Actors Concurrency", category: "Concurrency", difficulty: "Advanced", desc: "Tokio-backed stateful actors, mailboxes, fire-and-forget `tell`, and query `ask`" },
    { slug: "08_openapi_swagger", name: "OpenAPI & Swagger UI", category: "Documentation", difficulty: "Beginner", desc: "Declarative OpenAPI 3.1 schema extraction and embedded dark-themed Swagger UI" },
    { slug: "09_enterprise_clean_architecture", name: "Enterprise Clean Architecture", category: "Enterprise", difficulty: "Advanced", desc: "Enterprise layered architecture with events, repositories, domain errors, and DI" },
    { slug: "10_dto_validation_and_types", name: "DTO Validation & Types", category: "Validation", difficulty: "Intermediate", desc: "Custom decorators (@email, @min, @max, @required), nested DTOs, and RFC errors" },
    { slug: "11_operators_control_flow_and_resilience", name: "Operators & Resilience", category: "Core", difficulty: "Intermediate", desc: "Pipeline operators, loops, break/continue, and try/catch/finally resilience" },
    { slug: "12_microservice_event_driven", name: "Microservice Event-Driven", category: "Enterprise", difficulty: "Advanced", desc: "Asynchronous event bus, domain event storage, message serialization, and consumers" },
    { slug: "13_modular_monolith_ecommerce", name: "Modular Monolith E-Commerce", category: "Architecture", difficulty: "Advanced", desc: "Multi-bounded context (Catalog, Billing, Orders) within a single compile unit" },
    { slug: "14_realtime_iot_telemetry", name: "Realtime IoT Telemetry", category: "IoT", difficulty: "Advanced", desc: "High-throughput sensor ingestion, ring buffer windows, and anomaly alerting" },
    { slug: "15_data_structures_sequential", name: "Sequential Data Structures", category: "Data Structures", difficulty: "Intermediate", desc: "Native Vector, LinkedList, Stack (LIFO), Queue (FIFO), and CircularBuffer" },
    { slug: "16_data_structures_associative", name: "Associative Data Structures", category: "Data Structures", difficulty: "Intermediate", desc: "HashMap, Map literals, BTreeMap sorted indexing, and fast key-value lookups" },
    { slug: "17_data_structures_sets", name: "Set Data Structures", category: "Data Structures", difficulty: "Intermediate", desc: "HashSet, TreeSet, BitSet flag manipulation, and set algebra operations" },
    { slug: "18_data_structures_specialized", name: "Specialized Data Structures", category: "Data Structures", difficulty: "Advanced", desc: "MinHeap/MaxHeap, LRU Cache with eviction, Prefix Trie, and Graph algorithms" },
    { slug: "19_type_inference_and_types", name: "Type Inference & ADTs", category: "Language", difficulty: "Intermediate", desc: "Hindley-Milner type inference, Algebraic Data Types, and exhaustive matching" },
    { slug: "20_concurrency_and_streams", name: "Concurrency & Streams", category: "Concurrency", difficulty: "Advanced", desc: "Async stream pipelines, batch buffering, backpressure, and concurrent workers" },
    { slug: "21_caching_and_kv_store", name: "Caching & Key-Value Store", category: "Database", difficulty: "Intermediate", desc: "In-memory Key-Value store with TTL expiration, atomic counters, and rate limiting" },
    { slug: "22_nosql_document_store", name: "NoSQL Document Store", category: "Database", difficulty: "Intermediate", desc: "Document collection storage, JSON querying, indexing, and projection filters" },
    { slug: "23_enterprise_reporting_media_email", name: "Enterprise Reporting & Media", category: "Enterprise", difficulty: "Advanced", desc: "Streaming PDF generation, CSV tabular export, and TLS SMTP email dispatch" }
];

/**
 * Generate cryptographically secure nonce for CSP
 */
function getNonce() {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}

/**
 * Render the complete Webview HTML document
 */
function getWebviewHtml(webview, extensionUri, catalog, nonce) {
    const logoUri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'icon.png'));
    const getSvgDataUri = (filename) => {
        try {
            const p = path.join(extensionUri.fsPath, 'icons', filename);
            if (fs.existsSync(p)) {
                const buf = fs.readFileSync(p);
                return `data:image/svg+xml;base64,${buf.toString('base64')}`;
            }
        } catch (e) {}
        return '';
    };

    const iconMap = {
        file: getSvgDataUri('kora-file.svg'),
        service: getSvgDataUri('kora-service.svg'),
        model: getSvgDataUri('kora-model.svg'),
        controller: getSvgDataUri('kora-controller.svg'),
        repository: getSvgDataUri('kora-repository.svg'),
        routes: getSvgDataUri('kora-routes.svg'),
        entity: getSvgDataUri('kora-entity.svg'),
        dto: getSvgDataUri('kora-dto.svg'),
        interface: getSvgDataUri('kora-interface.svg'),
        enum: getSvgDataUri('kora-enum.svg'),
        middleware: getSvgDataUri('kora-middleware.svg'),
        validator: getSvgDataUri('kora-validator.svg'),
        event: getSvgDataUri('kora-event.svg'),
        test: getSvgDataUri('kora-test.svg'),
        seeder: getSvgDataUri('kora-seeder.svg'),
        config: getSvgDataUri('kora-config.svg'),
        module: getSvgDataUri('kora-module.svg')
    };
    const catalogList = catalog || EXAMPLES_CATALOG;
    const catalogHtml = catalogList.map(ex => `
        <div class="example-card" data-slug="${ex.slug}" data-name="${(ex.name || '').toLowerCase()}" data-category="${(ex.category || '').toLowerCase()}" data-desc="${(ex.desc || ex.description || '').toLowerCase()}" data-action="cloneExample">
            <div class="example-header">
                <span class="example-name">${ex.name}</span>
                <span style="display: flex; gap: 4px; align-items: center;">
                    ${ex.file_count ? `<span class="badge" style="background: rgba(255,255,255,0.08); font-size: 10px;">${ex.file_count} files</span>` : ''}
                    <span class="badge ${(ex.difficulty || 'intermediate').toLowerCase()}">${ex.difficulty || 'Intermediate'}</span>
                </span>
            </div>
            <div class="example-desc">${ex.desc || ex.description || ''}</div>
            <div class="example-footer">
                <span class="category-tag">${ex.category || 'Architecture'}</span>
                <button class="btn-clone" data-slug="${ex.slug}" data-action="cloneExample" title="Scaffold into current project">Scaffold Example</button>
            </div>
        </div>
    `).join('');

    
    const cssPath = path.join(__dirname, 'kora-webview.css');
    const jsPath = path.join(__dirname, 'kora-webview-main.js');
    const cssContent = fs.readFileSync(cssPath, 'utf8');
    const jsContent = fs.readFileSync(jsPath, 'utf8');

    const htmlBeforeScript = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src ${webview.cspSource} https: data:; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}' ${webview.cspSource}; font-src ${webview.cspSource}; connect-src https: http:;">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Kora AI Agent</title>
<style>
${cssContent}
</style>
</head>
<body>
    <!-- Global Native Drag & Drop Overlay Indicator -->
    <div id="drop-overlay" class="drop-overlay" style="display: none;">
        <div class="drop-overlay-content">
            <div class="drop-overlay-icon">📥</div>
            <div class="drop-overlay-title">Drop Files or Folders Here</div>
            <div class="drop-overlay-subtitle">Direct Explorer-to-AI Context Integration</div>
        </div>
    </div>

    <div class="header">
        <div class="logo">
            <img src="${logoUri}" width="20" height="20" class="logo-img" alt="Kora Logo" />
            <span>Kora AI Agent</span>
        </div>
        <div style="display: flex; align-items: center; gap: 6px;">
            <span class="active-model-pill" id="active-model-pill" data-tab="settings" data-action="switchTab" title="Active AI model. Click to configure settings.">Embedded</span>
            <span class="engine-badge" id="engine-indicator">Active</span>
        </div>
    </div>

    <div class="tabs" role="tablist">
        <button type="button" class="tab-btn active" data-tab="ask" id="tab-nav-ask" role="tab" aria-selected="true" aria-controls="tab-ask">AI Agent</button>
        <button type="button" class="tab-btn" data-tab="copilot" id="tab-nav-copilot" role="tab" aria-selected="false" aria-controls="tab-copilot">Inspector</button>
        <button type="button" class="tab-btn" data-tab="arch" id="tab-nav-arch" role="tab" aria-selected="false" aria-controls="tab-arch">Architecture</button>
        <button type="button" class="tab-btn" data-tab="learn" id="tab-nav-learn" role="tab" aria-selected="false" aria-controls="tab-learn">Learning</button>
        <button type="button" class="tab-btn" data-tab="examples" id="tab-nav-examples" role="tab" aria-selected="false" aria-controls="tab-examples">Examples (${catalogList.length})</button>
        <button type="button" class="tab-btn" data-tab="settings" id="tab-nav-settings" role="tab" aria-selected="false" aria-controls="tab-settings">Settings</button>
    </div>

    <!-- 1. AI Agent Chat Tab (Primary) -->
    <div id="tab-ask" class="section active">
        <div class="provider-bar">
            <div class="provider-select-group">
                <span style="font-size: 10px; opacity: 0.8; font-weight: 600;">Engine:</span>
                <select id="copilot-provider-select" class="provider-select">
                    <option value="embedded">Embedded Core (Offline Rules)</option>
                    <option value="copilot">GitHub Copilot (Native IDE)</option>
                    <option value="ollama">Local Ollama (Intelligent Discovery)</option>
                    <option value="github">GitHub Models API (PAT)</option>
                </select>
            </div>
            <div style="display: flex; align-items: center; gap: 4px;">
                <button class="btn-config-toggle btn-new-chat" data-action="newChat" title="Start a fresh chat conversation (Clear history)">+ New Chat</button>
                <button class="btn-config-toggle" data-action="toggleConfigPanel" title="Configure Engine Settings">Configure</button>
            </div>
        </div>

        <div id="ollama-alert-banner" style="display: none; background: rgba(245, 158, 11, 0.12); border: 1px solid rgba(245, 158, 11, 0.35); border-radius: 4px; padding: 6px 8px; margin-bottom: 6px; font-size: 10px; line-height: 1.4;">
            <div id="ollama-alert-text"></div>
        </div>

        <div id="provider-config-box" class="provider-config-box" style="display: none;">
            <div id="cfg-copilot" style="display: none;">
                <div style="font-weight: 600; margin-bottom: 4px; color: #60A5FA;">Native GitHub Copilot</div>
                <div style="font-size: 10.5px; opacity: 0.85; margin-bottom: 6px;">Zero token setup: Queries the Language Model API provided by your authenticated GitHub Copilot IDE session.</div>
                <div style="font-size: 10px; color: #10B981; font-weight: 600;">Ready to chat without manual API keys or endpoints.</div>
            </div>
            <div id="cfg-github" style="display: none;">
                <div style="font-weight: 600; margin-bottom: 4px;">GitHub Models Setup</div>
                <button class="action-btn primary" style="width: 100%; margin-bottom: 6px; padding: 4px;" data-action="requestGitHubLogin">Auto-Link GitHub Account</button>
                <div style="font-size: 10px; opacity: 0.7; margin-bottom: 2px;">Or paste Personal Access Token (PAT):</div>
                <input type="password" id="cfg-gh-token" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="ghp_... or github_pat_..." />
                <div style="font-size: 10px; opacity: 0.7; margin: 4px 0 2px 0;">Model:</div>
                <div style="display: flex; gap: 4px;">
                    <select id="cfg-gh-model" class="provider-select" style="flex: 1;">
                        <option value="gpt-4o-mini">gpt-4o-mini (Fast & Recommended)</option>
                        <option value="Phi-3.5-mini-instruct">Phi-3.5-mini-instruct (Small)</option>
                        <option value="Meta-Llama-3.1-8B-Instruct">Meta-Llama-3.1-8B-Instruct</option>
                    </select>
                    <button class="btn-send-main" data-action="saveGitHubConfig">Save</button>
                </div>
            </div>
            <div id="cfg-ollama" style="display: none;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
                    <span style="font-weight: 600; color: #34D399;">Local Ollama Models</span>
                    <button class="btn-prop-sm" data-action="refreshOllamaModels" title="Refresh models from local Ollama">Scan</button>
                </div>
                <div style="font-size: 10px; opacity: 0.7; margin-bottom: 2px;">Endpoint URL:</div>
                <input type="text" id="cfg-ollama-endpoint" class="chat-textarea" style="min-height: 28px; max-height: 28px;" value="http://localhost:11434" placeholder="http://localhost:11434 or http://192.168.x.x:11434" />
                <div style="font-size: 10px; opacity: 0.7; margin: 4px 0 2px 0;">Discovered / Selected Model:</div>
                <div style="display: flex; gap: 4px; margin-bottom: 4px;">
                    <select id="cfg-ollama-model" class="provider-select" style="flex: 1;">
                        <option value="auto">Auto-detect best model</option>
                        <option value="custom">Enter custom model name...</option>
                    </select>
                    <button class="btn-send-main" data-action="saveOllamaConfig">Save</button>
                </div>
                <input type="text" id="cfg-ollama-custom" class="chat-textarea" style="min-height: 28px; max-height: 28px; display: none;" placeholder="Type custom model (e.g. qwen2.5-coder:7b)..." />
            </div>
            <div id="cfg-embedded" style="display: none;">
                <div style="font-weight: 600; margin-bottom: 2px;">Embedded Knowledge Engine</div>
                <div style="font-size: 10px; opacity: 0.8;">Instant offline inference powered by Kora AST analyzer, .agent/rules/ invariants, and the official language documentation.</div>
            </div>
        </div>

        <!-- Compact Antigravity-Style Agent Status & Step Timeline -->
        <div id="agent-tracker" class="agent-tracker" style="display: none;">
            <div class="tracker-header" style="cursor: pointer;" title="Click to expand/collapse agent activity steps">
                <div style="display: flex; align-items: center; gap: 6px;">
                    <span class="tracker-badge" id="tracker-badge">Working</span>
                    <span id="tracker-state-title" style="color: #38bdf8;">Exploring Workspace</span>
                </div>
                <span id="tracker-chevron" style="font-size: 9px; opacity: 0.8;">v</span>
            </div>
            <div id="tracker-detail" class="tracker-detail">Scanning project files and module relationships...</div>
            <div id="tracker-steps" class="tracker-steps" style="display: none;"></div>
        </div>

        <div class="chat-scroll-area" id="chat-scroll-area">
            <div class="suggestion-chips">
                <span class="chip" data-chip="Auto-fix all rule violations across workspace">Auto-Fix Workspace</span>
                <span class="chip" data-chip="Analyze workspace Clean Architecture layers">Analyze Architecture</span>
                <span class="chip" data-chip="Audit active file against .agent rules">Audit Rules</span>
                <span class="chip" data-chip="Show Rule 08: Actors & Mailboxes">Rule 08 Concurrency</span>
                <span class="chip" data-chip="Explain Single-Null Design vs undefined">Single-Null vs Undefined</span>
            </div>

            <div class="chat-history" id="chat-history"></div>
        </div>

        <!-- Persistent Fixed Bottom Chat Input Container -->
        <div class="chat-input-box" style="position: relative;">
            <div id="mention-menu" class="mention-menu" style="display: none;">
                <div class="mention-menu-header">Context References (click or Enter to insert):</div>
                <div class="mention-items-list" id="mention-items-list"></div>
            </div>

            <!-- Attached Files/Folders Bar -->
            <div id="attached-files-bar" class="attached-files-bar" style="display: none;"></div>

            <div class="chat-scope-bar">
                <div style="display: flex; align-items: center; gap: 4px;">
                    <span style="opacity: 0.8;">Scope:</span>
                    <select id="chat-scope" class="scope-select">
                        <option value="workspace">Entire Workspace</option>
                        <option value="active">Active File Only</option>
                        <option value="refactor">Refactor Mode</option>
                    </select>
                </div>
                <div style="display: flex; align-items: center; gap: 4px; overflow-x: auto;">
                    <button class="btn-mention-pill" title="Reference entire project architecture">@workspace</button>
                    <button class="btn-mention-pill" title="Reference active editor code selection">@selection</button>
                    <button class="btn-mention-pill" title="Reference specific project file">@file:</button>
                    <button class="btn-mention-pill" title="Reference project directory/folder">@folder:</button>
                    <button class="btn-mention-pill" title="Reference current compiler diagnostics">@problems</button>
                </div>
            </div>
            <textarea id="chat-input" class="chat-textarea" placeholder="Ask Kora AI Agent, drag & drop files/folders here, type @ for context references, or /fix... (Enter to send, Shift+Enter for new line)"></textarea>
            <div class="chat-actions-row">
                <button class="btn-refactor-quick" data-action="requestWorkspaceRefactor" title="Workspace-wide AI Auto-Fix & Refactor">
                    <span>Auto-Fix & Refactor</span>
                </button>
                <div style="display: flex; align-items: center; gap: 4px;">
                    <button class="btn-attach" id="btn-attach-active" data-action="attachActiveFile" title="Attach currently open editor file" style="display: none; border-color: rgba(56, 189, 248, 0.4); color: #38bdf8;">+ Active File</button>
                    <button class="btn-attach" data-action="newChat" title="Clear conversation & start new chat">New Chat</button>
                    <button class="btn-attach" id="btn-attach" data-action="triggerFilePicker" title="Attach File or Folder (or Hold Shift + Drag from Explorer)">Attach</button>
                    <button class="btn-stop-gen" id="btn-stop" style="display: none;" data-action="stopGeneration" title="Stop Generation">
                        <span>Stop</span>
                    </button>
                    <button class="btn-send-main" id="btn-send" data-action="sendPrompt">
                        <span>Send</span>
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- 2. Active File Inspector Tab -->
    <div id="tab-copilot" class="section">
        <div class="file-status-box">
            <div class="file-title">
                <span id="active-filename">No active file</span>
                <span id="active-pill" class="pill library">Library</span>
            </div>
            <div id="active-meta" style="font-size: 11px; opacity: 0.8; margin-bottom: 6px;">Open a .kora or .kr file to analyze</div>
            
            <div class="action-grid">
                <button class="action-btn primary" id="btn-run" data-action="smartRun">Smart Run / Test</button>
                <button class="action-btn" data-action="auditFile" title="Audit file against .agent rules">Audit Rules</button>
                <button class="action-btn" data-action="checkFile" title="Run type checks and linter">Check Types</button>
                <button class="action-btn" data-action="runTests" title="Execute test suites">Test Suite</button>
                <button class="action-btn" data-action="explainCode" title="Explain code architecture">Explain Code</button>
                <button class="action-btn" data-action="generateTests" title="Scaffold tests for active symbols">Scaffold Tests</button>
                <button class="action-btn" data-tab="arch" data-action="switchTab" title="Inspect clean architecture layers">Architecture</button>
                <button class="action-btn" data-action="requestWorkspaceRefactor" style="grid-column: span 2; background: linear-gradient(135deg, #0284c7, #2563eb); color: #fff; font-weight: 600;" title="AI analyzes workspace/file and proposes coordinated code fixes with Diff Review">AI Auto-Fix & Refactor (Workspace)</button>
            </div>
        </div>

        <div id="audit-container" style="display: none;">
            <div style="font-weight: 600; margin-bottom: 4px;">Rule Compliance Audit:</div>
            <div id="audit-text" class="audit-box"></div>
        </div>

        <div id="diagnostics-panel" style="display: none;">
            <div style="font-weight: 600; margin-bottom: 4px; color: #ef4444;">Compiler Diagnostics:</div>
            <div id="diagnostics-list"></div>
        </div>

        <div id="explanation-container" style="display: none;">
            <div style="font-weight: 600; margin-bottom: 4px;">Architecture Explanation:</div>
            <div id="explanation-text" class="explanation-box"></div>
        </div>
    </div>

    <!-- 3. Architecture Explorer Tab -->
    <div id="tab-arch" class="section">
        <div style="font-weight: 600; font-size: 13px; margin-bottom: 6px;">Workspace Architecture Explorer</div>
        <div style="font-size: 11px; opacity: 0.8; margin-bottom: 10px;">Visual representation of Clean Architecture layers across your workspace:</div>
        
        <div id="arch-stats" class="action-grid" style="grid-template-columns: 1fr 1fr 1fr; margin-bottom: 12px;">
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #3B82F6;" id="count-ctrl">0</div>
                <div style="font-size: 9px; opacity: 0.7;">Controllers</div>
            </div>
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #10B981;" id="count-svc">0</div>
                <div style="font-size: 9px; opacity: 0.7;">Services</div>
            </div>
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #F59E0B;" id="count-repo">0</div>
                <div style="font-size: 9px; opacity: 0.7;">Repositories</div>
            </div>
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #EC4899;" id="count-entity">0</div>
                <div style="font-size: 9px; opacity: 0.7;">Entities</div>
            </div>
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #8B5CF6;" id="count-model">0</div>
                <div style="font-size: 9px; opacity: 0.7;">DTO Models</div>
            </div>
            <div style="background: var(--vscode-editor-background); padding: 6px; border-radius: 4px; text-align: center;">
                <div style="font-size: 16px; font-weight: 700; color: #06B6D4;" id="count-test">0</div>
                <div style="font-size: 9px; opacity: 0.7;">Tests</div>
            </div>
        </div>

        <button class="action-btn primary" style="width: 100%; margin-bottom: 12px;" data-action="analyzeWorkspace">Refresh Workspace Map</button>

        <div style="font-weight: 600; margin-bottom: 6px;">Layer Data Flow (Clean Architecture):</div>
        <div style="background: var(--vscode-editor-background); padding: 8px; border-radius: 4px; font-size: 11px; line-height: 1.5; border-left: 3px solid #10B981;">
            <b>1. HTTP Request</b> -> <code>Controllers (Presentation)</code><br>
            <b>2. Validation</b> -> <code>DTO Models (Boundary)</code><br>
            <b>3. Injected DI</b> -> <code>Services (Application Logic)</code><br>
            <b>4. Interface Contract</b> -> <code>Repositories (Data Access)</code><br>
            <b>5. Storage Mapping</b> -> <code>Entities (@table SQL / NoSQL)</code>
        </div>
    </div>

    <!-- 4. Learning Center Tab -->
    <div id="tab-learn" class="section">
        <div style="font-weight: 600; font-size: 13px; margin-bottom: 6px;">Kora Interactive Learning Center</div>
        <div style="font-size: 11px; opacity: 0.8; margin-bottom: 10px;">Master Kora through hands-on lessons with runnable code:</div>

        <div class="lesson-card" style="background: var(--vscode-editor-background); padding: 8px; border-radius: 6px; margin-bottom: 8px;">
            <div style="font-weight: 600; color: var(--vscode-textLink-foreground);">Lesson 1: Pure Models & DTOs</div>
            <div style="font-size: 11px; margin: 4px 0;">Models define immutable payloads without runtime overhead:</div>
            <pre style="background: rgba(0,0,0,0.3); padding: 6px; border-radius: 4px; font-size: 10px;">model User {\n    name: String\n    age: Int\n}</pre>
            <button class="action-btn" data-action="insertSnippet" data-snippet="lesson1">Insert into Editor</button>
        </div>

        <div class="lesson-card" style="background: var(--vscode-editor-background); padding: 8px; border-radius: 6px; margin-bottom: 8px;">
            <div style="font-weight: 600; color: var(--vscode-textLink-foreground);">Lesson 2: Clean Architecture & DI</div>
            <div style="font-size: 11px; margin: 4px 0;">Services receive dependencies via <code>init(...)</code> constructor injection:</div>
            <pre style="background: rgba(0,0,0,0.3); padding: 6px; border-radius: 4px; font-size: 10px;">class UserService {\n    repo: UserRepository\n    init(repo: UserRepository) {\n        this.repo = repo\n    }\n}</pre>
            <button class="action-btn" data-action="insertSnippet" data-snippet="lesson2">Insert into Editor</button>
        </div>

        <div class="lesson-card" style="background: var(--vscode-editor-background); padding: 8px; border-radius: 6px; margin-bottom: 8px;">
            <div style="font-weight: 600; color: var(--vscode-textLink-foreground);">Lesson 3: Concurrency Actors & Mailboxes</div>
            <div style="font-size: 11px; margin: 4px 0;">Stateful actors run on Tokio threads with <code>tell</code> and <code>ask</code>:</div>
            <pre style="background: rgba(0,0,0,0.3); padding: 6px; border-radius: 4px; font-size: 10px;">class WorkerActor {\n    var count: Int\n    init() { this.count = 0 }\n    fn handle(msg: String) -> Int {\n        this.count += 1\n        return this.count\n    }\n}</pre>
            <button class="action-btn" data-action="insertSnippet" data-snippet="lesson3">Insert into Editor</button>
        </div>

        <div class="lesson-card" style="background: var(--vscode-editor-background); padding: 8px; border-radius: 6px; margin-bottom: 8px;">
            <div style="font-weight: 600; color: var(--vscode-textLink-foreground);">Lesson 4: Load Test with Teardown</div>
            <div style="font-size: 11px; margin: 4px 0;">High throughput stress testing with guaranteed resource teardown (Rule 10):</div>
            <pre style="background: rgba(0,0,0,0.3); padding: 6px; border-radius: 4px; font-size: 10px;">class LoadTestSuite {\n    fn run(n: Int) -> Int {\n        var total = 0\n        for i in 0..n { total += 1 }\n        return total\n    }\n    fn teardown() -> Unit {\n        print(\"Teardown complete\")\n    }\n}</pre>
            <button class="action-btn" data-action="insertSnippet" data-snippet="lesson4">Insert into Editor</button>
        </div>
    </div>

    <!-- 5. Examples Catalog -->
    <div id="tab-examples" class="section">
        <div style="margin-bottom: 8px; font-size: 11px; opacity: 0.8;">Explore <b>${catalogList.length}</b> dynamically discovered production architectures:</div>
        <div style="margin-bottom: 10px;">
            <input type="text" id="example-search-input" placeholder="🔍 Search examples by name, category, or slug..." class="chat-textarea" style="min-height: 28px; max-height: 28px; font-size: 11px; border-radius: 4px; padding: 4px 8px; width: 100%; box-sizing: border-box;" />
        </div>
        <div id="catalog-list">
            ${catalogHtml}
        </div>
    </div>

    <!-- 6. Settings Section -->
    <div id="tab-settings" class="section">
        <div style="font-weight: 600; font-size: 12.5px; margin-bottom: 4px;">AI Models & API Keys Settings</div>
        <div style="font-size: 10.5px; opacity: 0.8; margin-bottom: 10px;">Configure AI engine providers, API keys, custom endpoints, and models:</div>

        <div class="settings-group">
            <label class="settings-label">Active Engine Provider:</label>
            <select id="cfg-active-provider" class="provider-select">
                <option value="embedded">Embedded Core (Offline Rules & Docs)</option>
                <option value="copilot">GitHub Copilot (Native IDE - Zero Config)</option>
                <option value="ollama">Local Ollama (qwen2.5-coder / local models)</option>
                <option value="github">GitHub Models API (PAT / Azure)</option>
                <option value="openai">OpenAI (gpt-4o / gpt-4o-mini)</option>
                <option value="gemini">Google Gemini (gemini-1.5-flash)</option>
                <option value="anthropic">Anthropic Claude (claude-3-5-sonnet)</option>
                <option value="custom">Custom OpenAI-compatible Endpoint</option>
            </select>
        </div>

        <div class="settings-group" id="cfg-field-copilot" style="display: none; background: rgba(59, 130, 246, 0.1); border: 1px solid rgba(59, 130, 246, 0.3); padding: 8px; border-radius: 4px; margin-bottom: 8px;">
            <div style="font-weight: 600; color: #60A5FA; margin-bottom: 4px;">Native GitHub Copilot Mode</div>
            <div style="font-size: 10.5px; opacity: 0.85; line-height: 1.4;">Zero tokens required: Directly connects to GitHub Copilot's Language Model in your IDE. No external PAT or setup needed!</div>
        </div>

        <div class="settings-group" id="cfg-field-ollama" style="display: none; background: rgba(16, 185, 129, 0.08); border: 1px solid rgba(16, 185, 129, 0.25); padding: 8px; border-radius: 4px; margin-bottom: 8px;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
                <span style="font-weight: 600; color: #34D399;">Local Ollama Integration</span>
                <button class="btn-prop-sm" data-action="refreshOllamaModels">Scan Models</button>
            </div>
            <div style="font-size: 10.5px; opacity: 0.85; line-height: 1.4; margin-bottom: 6px;">Private offline execution on your machine or LAN. Auto-detects local models (<b>qwen2.5-coder:7b</b>, etc.) with custom endpoint/URL support.</div>
            <div id="ollama-scan-status" style="font-size: 10px; color: #34D399; margin-bottom: 4px; display: none;"></div>
            <div style="font-size: 10px; opacity: 0.8; margin-bottom: 2px;">Available Local Models:</div>
            <select id="cfg-settings-ollama-select" class="provider-select" style="width: 100%;">
                <option value="auto">Auto-detecting local models...</option>
            </select>
        </div>

        <div class="settings-group" id="cfg-field-github">
            <label class="settings-label">GitHub Token / PAT:</label>
            <div style="display: flex; gap: 4px; margin-bottom: 4px;">
                <input type="password" id="cfg-input-github" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="ghp_... or github_pat_..." />
                <button class="btn-prop-sm" data-action="togglePassword" data-target="cfg-input-github">Show</button>
            </div>
            <button class="action-btn" style="width: 100%;" data-action="requestGitHubLogin">Auto-Link GitHub Account</button>
        </div>

        <div class="settings-group" id="cfg-field-openai">
            <label class="settings-label">OpenAI API Key:</label>
            <div style="display: flex; gap: 4px;">
                <input type="password" id="cfg-input-openai" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="sk-..." />
                <button class="btn-prop-sm" data-action="togglePassword" data-target="cfg-input-openai">Show</button>
            </div>
        </div>

        <div class="settings-group" id="cfg-field-gemini">
            <label class="settings-label">Google Gemini API Key:</label>
            <div style="display: flex; gap: 4px;">
                <input type="password" id="cfg-input-gemini" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="AIzaSy..." />
                <button class="btn-prop-sm" data-action="togglePassword" data-target="cfg-input-gemini">Show</button>
            </div>
        </div>

        <div class="settings-group" id="cfg-field-anthropic">
            <label class="settings-label">Anthropic Claude API Key:</label>
            <div style="display: flex; gap: 4px;">
                <input type="password" id="cfg-input-anthropic" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="sk-ant-..." />
                <button class="btn-prop-sm" data-action="togglePassword" data-target="cfg-input-anthropic">Show</button>
            </div>
        </div>

        <div class="settings-group">
            <label class="settings-label">Model Name:</label>
            <input type="text" id="cfg-input-model" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="e.g. gpt-4o-mini, phi3:mini, claude-3-5-sonnet" />
        </div>

        <div class="settings-group">
            <label class="settings-label">Endpoint URL (Optional):</label>
            <input type="text" id="cfg-input-endpoint" class="chat-textarea" style="min-height: 28px; max-height: 28px;" placeholder="e.g. http://localhost:11434, http://192.168.x.x:11434, or https://api.openai.com/v1" />
        </div>

        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 6px; margin-top: 12px;">
            <button class="action-btn primary" data-action="saveAllSettings">Save Settings</button>
            <button class="action-btn" data-action="testConnection">Test Connection</button>
        </div>

        <div id="test-connection-status" style="margin-top: 10px; display: none; padding: 8px; border-radius: 5px; font-size: 11px; line-height: 1.4;"></div>
    </div>

    
    
    <script nonce="${nonce}">
        window.KORA_ICONS = ${JSON.stringify(iconMap)};
`;

    const htmlAfterScript = `
    </script>
</body>
</html>`;

    return htmlBeforeScript + jsContent + htmlAfterScript;
}

module.exports = {
    EXAMPLES_CATALOG,
    getNonce,
    getWebviewHtml
};
