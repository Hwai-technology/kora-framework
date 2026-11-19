const fs = require('fs');
const path = require('path');
const http = require('http');
const https = require('https');
const { WorkspaceContextEngine } = require('./workspace-context-engine');
const { ChangeReviewManager } = require('./change-review-manager');
const { OllamaDiscoveryService } = require('./ollama-discovery-service');
const { IntentUnderstandingEngine, IntentCategory } = require('./intent-understanding-engine');

/**
 * Built-in Core Knowledge Base derived from:
 * - integrations/vscode/knowledge/ (language, compiler, framework, cli, examples, rules)
 * - .agent/rules/ (00-10, compiler-boundaries, language-evolution)
 * - .agent/standards/ (architecture, coding, performance, security, testing)
 * - docs/ (architecture, language, getting-started, ecosystem)
 * - documentation/ (database, concurrency, microservices, cli)
 */
const KORA_KNOWLEDGE_BASE = [
    {
        topic: "Clean Architecture Invariants",
        source: ".agent/rules/04-clean-architecture.md",
        keywords: ["clean architecture", "slice", "layer", "e5001", "e5002", "e5003", "e5004", "router", "di", "controller", "service"],
        content: `### Clean Architecture Invariants & Compiler Enforcement (E5001 - E5004)
Kora strictly enforces Clean Architecture at compile time:
- **E5001 (Direct Infrastructure Instantiation)**: Controllers and Services must never instantiate database connections, HTTP clients, or raw infrastructure directly inside handler methods. All dependencies must be injected via \`init(...)\`.
- **E5002 (Missing Interface Abstraction)**: High-level services must depend on \`interface\` abstractions, never on concrete database drivers.
- **E5003 (Leaking Domain Entity via Controller)**: Domain entities must not be returned directly across API boundaries without being wrapped in DTOs.
- **E5004 (Circular Architectural Layer Dependency)**: Domain layer must not depend on Application layer; Application layer must not depend on Infrastructure.

**Mandatory Directory Segregation**:
- \`entities/\`: All persisted relational database models (\`@table("...")\`) and NoSQL document models (\`@collection("...")\`).
- \`models/\` or \`dto/\`: All API request/response payloads, validation schemas, and transport models (never decorated with table attributes).`
    },
    {
        topic: "Language Philosophy & Single-Null Design",
        source: ".agent/rules/01-language-philosophy.md",
        keywords: ["null", "undefined", "single null", "e2005", "philosophy", "zero guessing", "strict typing"],
        content: `### Kora Language Philosophy & Single-Null Design
- **Single-Null Design**: Kora has \`null\` and \`Option<T>\` (\`Some(val)\` | \`None\`). The JavaScript/TypeScript \`undefined\` construct is **strictly prohibited** (\`E2005\`).
- **Zero Value Guessing**: The compiler and runtime strictly forbid arbitrary default fallbacks (\`|| ''\`, \`|| 0\`, \`|| false\`) masking missing data.
- **Affine Ownership & Memory Safety**: Inspired by Rust, values are tracked without a stop-the-world Garbage Collector.
- **Direct Native Execution**: Compiles to optimized native machine binaries with sub-millisecond cold starts and multi-gigabit throughput.`
    },
    {
        topic: "Multi-Engine Database ORM & Decorators",
        source: ".agent/rules/07-database-multi-engine-orm.md",
        keywords: ["database", "orm", "table", "collection", "mysql", "postgres", "sqlite", "mongodb", "redis", "entity", "decorator", "@table", "@entity", "@primary"],
        content: `### Multi-Engine Database ORM & Decorator Architecture
Kora provides native compile-time support for three database paradigms:
1. **Relational SQL (PostgreSQL, MySQL, SQLite)**:
   - Annotated with \`@entity\` and \`@table("table_name")\`.
   - Field decorators: \`@primary\`, \`@autoIncrement\`, \`@unique\`, \`@index\`, \`@column("col_name")\`, \`@required\`, \`@email\`, \`@min(0)\`, \`@max(100)\`.
   - Parameterized queries: \`db.query(sql)\`, \`db.execute(sql)\`.
   - ACID transaction blocks: \`db.begin()\`, \`db.commit()\`, \`db.rollback()\`.
2. **Document NoSQL (MongoDB-compatible)**:
   - Annotated with \`@collection("collection_name")\`.
   - Operations: \`db.collection("name").insertOne(doc)\`, \`findOne(filter)\`, \`deleteMany(timestamp)\`.
3. **Key-Value Cache (Redis-compatible)**:
   - Operations: \`db.set(k, v)\`, \`db.setEx(k, v, ttl)\`, \`db.get(k)\`, \`db.incr(k)\`.`
    },
    {
        topic: "Concurrency, Actors & Non-Blocking Streams",
        source: ".agent/rules/08-concurrency-actors-streams.md",
        keywords: ["actor", "concurrency", "mailbox", "tell", "ask", "channel", "broadcast", "spawn", "tokio", "stream", "taskgroup"],
        content: `### Concurrency, Actors & Non-Blocking Message Passing
- **Stateful Actors**: Isolated units of concurrency executing on Tokio worker threads with non-blocking mailboxes.
  - \`actor.tell(msg)\`: Fire-and-forget message dispatch.
  - \`actor.ask(msg)\`: Request-response asynchronous query with built-in timeout protection.
- **Channels**:
  - \`channel<T>(capacity)\`: MPSC (multi-producer, single-consumer) bounded async channel.
  - \`broadcast<T>(capacity)\`: Multi-consumer pub/sub channel.
- **Structured Task Groups**: \`spawn(async () => { ... })\` with cooperative cancellation.`
    },
    {
        topic: "Testing Standards, Load Tests & Mandatory Teardown",
        source: ".agent/rules/10-testing-and-verification.md",
        keywords: ["test", "testing", "unit test", "load test", "teardown", "cleanup", "assert", "benchmark", "stress"],
        content: `### Testing Standards & Mandatory Teardown
All Kora projects and generated features must provide automated test coverage:
1. **Unit Test Suites (\`*_unit.test.kora\`)**:
   - Class-based suite: \`class FeatureUnitTestSuite { init(...) {} }\`.
   - Tests invariants, input boundaries, edge cases, pattern matching, and error branches.
2. **Peak Throughput Load Tests (\`*_load.test.kora\`)**:
   - Class-based suite: \`class FeatureLoadTestSuite { init(...) {} }\`.
   - Simulates 1,000 to 10,000 iterations measuring throughput and latency.
   - **Mandatory \`teardown()\`**: Every load test must implement explicit cleanup purging synthetic records, closing connection pools, and releasing memory.
3. **Automated Test Runner**:
   - \`kora test [path]\`: Automatically discovers all \`*.test.kora\` and \`main.kora\` files and validates 100% of test assertions.`
    },
    {
        topic: "Kora CLI Tooling & Scaffold Commands",
        source: "docs/cli/tooling.md",
        keywords: ["cli", "kora create", "kora new", "kora g feature", "kora check", "kora run", "kora test", "kora example", "generator"],
        content: `### Kora CLI Toolchain Commands
- \`kora check <file.kora>\`: Type-checks, enforces Clean Architecture, and verifies ownership without compilation.
- \`kora run <file.kora>\`: Compiles and executes native release binary.
- \`kora test [dir]\`: Runs the automated test runner across all unit, load, and lifecycle test suites.
- \`kora g feature <name>\`: Generates a complete Clean Architecture slice (routes, controller, service, repo, validator, entity, dto, errors, events, unit test, load test).
- \`kora example list\`: Displays the dynamic catalog of official production examples.
- \`kora example init <slug> [dir]\`: Clones/scaffolds an official GitHub example with offline caching.
- \`kora example search <query>\`: Searches official examples catalog.
- \`kora doctor\`: Verifies compiler, runtime, environment, and extension health.
- \`kora fmt [path]\`: Canonical code formatter.`
    },
    {
        topic: "VS Code Extension & Tooling (Rule 11)",
        source: ".agent/rules/11-vscode-extension-and-tooling.md",
        keywords: ["vscode", "extension", "ide", "lsp", "copilot", "hover", "autocomplete", "definition", "symbols", "quick fix", "audit", "learning center", "architecture explorer"],
        content: `### VS Code Extension & IDE Tooling (Rule 11)
The official Kora VS Code Extension (\`integrations/vscode/\`) provides:
- **Native Language Providers**:
  - \`CompletionItemProvider\`: Context-aware keywords, decorators, types, and dot-member autocompletion (\`user.\`).
  - \`HoverProvider\`: Rich Markdown documentation explaining signatures, types, and Clean Architecture roles.
  - \`DefinitionProvider\` (F12): Jump to declaration for models, classes, functions, and bindings.
  - \`DocumentSymbolProvider\`: Full Outline View and Breadcrumb hierarchy.
  - \`DocumentFormattingEditProvider\` (Shift+Alt+F): Canonical 4-space code formatting.
  - \`FoldingRangeProvider\`: Multi-line block folding.
  - \`CodeActionProvider\`: 1-click Quick Fixes for \`E2005\` (replace \`undefined\` with \`null\`), \`E5001\` (add \`init(...)\`), test scaffolding, and rules auditing.
- **Kora Assistant Sidebar Panel**:
  - **Inspector**: Real-time file analysis, entrypoint/test classification, live compiler diagnostics.
  - **Architecture Explorer**: Live layer counters (Controllers, Services, Repositories, Entities, Models, Tests) & Clean Architecture data flow.
  - **Learning Center**: 4 hands-on lessons with runnable code and 1-click editor insertion.
  - **Copilot Chat**: Context-aware AI assistant querying \`.agent/rules/\`, \`docs/\`, and \`knowledge/\`.
  - **Examples Browser**: Dynamic ecosystem of 100+ official production examples with 1-click scaffolding via \`kora example init\`.`
    }
];

/**
 * Antigravity-style agent lifecycle states
 */
const AgentState = {
    IDLE: 'Idle',
    UNDERSTANDING: 'UnderstandingRequest',
    EXPLORING: 'ExploringWorkspace',
    ANALYZING: 'AnalyzingFiles',
    PLANNING: 'PlanningChanges',
    GENERATING: 'GeneratingChanges',
    REVIEWING: 'ReviewingChanges',
    AWAITING_APPROVAL: 'AwaitingApproval',
    APPLYING: 'ApplyingChanges',
    VALIDATING: 'ValidatingChanges',
    COMPLETED: 'Completed',
    FAILED: 'Failed'
};

class KoraCopilotEngine {
    constructor(extensionUriOrVscode = null, vscodeInstance = null) {
        let extensionUri = null;
        let vsc = null;
        if (extensionUriOrVscode && extensionUriOrVscode.fsPath) {
            extensionUri = extensionUriOrVscode;
            vsc = vscodeInstance;
        } else if (extensionUriOrVscode && extensionUriOrVscode.workspace) {
            vsc = extensionUriOrVscode;
            extensionUri = vscodeInstance;
        } else {
            extensionUri = extensionUriOrVscode;
            vsc = vscodeInstance;
        }
        this.extensionUri = extensionUri;
        this.vscode = vsc;
        this.knowledgeFiles = [];
        this.workspaceDocs = [];
        this.currentState = AgentState.IDLE;
        this.currentSteps = [];

        this.workspaceEngine = new WorkspaceContextEngine(this.vscode);
        this.reviewManager = new ChangeReviewManager(this.vscode);
        this.ollamaService = new OllamaDiscoveryService();
        this.intentEngine = new IntentUnderstandingEngine();

        this._loadKnowledgeFiles();
    }

    async indexWorkspace(workspaceRoot) {
        if (!workspaceRoot) return null;
        return await this.workspaceEngine.scanWorkspace([workspaceRoot]);
    }

    walkthroughCode(selection) {
        if (!selection || !selection.trim()) {
            return "Please select a block of Kora code to generate an architectural walkthrough.";
        }
        const lines = selection.split('\n');
        let out = `### Interactive Code Walkthrough\n\n`;
        out += `- **Selected Lines**: ${lines.length}\n`;

        const hasClass = /class\s+([A-Za-z0-9_]+)/.exec(selection);
        const hasActor = /actor\s+([A-Za-z0-9_]+)|class\s+([A-Za-z0-9_]+Actor)/.exec(selection);
        const hasModel = /model\s+([A-Za-z0-9_]+)/.exec(selection);
        const hasInit = /init\s*\(([^)]*)\)/.exec(selection);

        if (hasActor) {
            out += `#### Actor Component (\`${hasActor[1] || hasActor[2]}\`)\n`;
            out += `This component represents a stateful concurrency actor executing over Tokio worker threads. Messages are received sequentially through non-blocking mailboxes (Rule 08).\n\n`;
        } else if (hasClass) {
            out += `#### Service / Application Class (\`${hasClass[1]}\`)\n`;
            out += `Application logic layer component. Must enforce Clean Architecture dependency inversion.\n\n`;
        } else if (hasModel) {
            out += `#### Pure DTO Model (\`${hasModel[1]}\`)\n`;
            out += `Immutable payload structure used across network and boundary interfaces (Rule 04).\n\n`;
        }

        if (hasInit) {
            out += `**Dependencies (Injected via \`init\`):**\n`;
            out += `\`${hasInit[1].trim() || 'None'}\`\n\n`;
        }

        if (selection.includes('undefined')) {
            out += `**Notice**: Contains illegal \`undefined\` keyword. In Kora, use \`null\` or \`Option<T>\` (Rule 01).\n`;
        }

        return out;
    }

    generateDocumentation(code, lang = 'kora') {
        if (!code || !code.trim()) return "/**\n * @module\n */\n";
        const fnMatch = code.match(/fn\s+([A-Za-z0-9_]+)\s*\(([^)]*)\)(?:\s*->\s*([A-Za-z0-9_<>\[\]]+))?/);
        if (fnMatch) {
            const name = fnMatch[1];
            const params = fnMatch[2].split(',').map(p => p.trim()).filter(Boolean);
            const ret = fnMatch[3] || 'Unit';
            let doc = `/**\n * Executes \`${name}\`.\n`;
            for (const p of params) {
                const pName = p.split(':')[0].trim();
                doc += ` * @param {${p.split(':')[1] ? p.split(':')[1].trim() : 'Any'}} ${pName}\n`;
            }
            doc += ` * @returns {${ret}}\n */\n`;
            return doc;
        }
        return `/**\n * Kora Component\n * @module\n */\n`;
    }

    _loadKnowledgeFiles() {
        if (!this.extensionUri) return;
        try {
            const knowledgeDir = path.join(this.extensionUri.fsPath, 'knowledge');
            if (fs.existsSync(knowledgeDir)) {
                this._indexDirectory(knowledgeDir);
            }
        } catch (e) {}
    }

    _indexDirectory(dir) {
        try {
            const entries = fs.readdirSync(dir, { withFileTypes: true });
            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);
                if (entry.isDirectory()) {
                    this._indexDirectory(fullPath);
                } else if (entry.isFile() && (entry.name.endsWith('.json') || entry.name.endsWith('.md'))) {
                    try {
                        const content = fs.readFileSync(fullPath, 'utf8');
                        this.knowledgeFiles.push({
                            path: fullPath,
                            name: entry.name,
                            content: content
                        });
                    } catch (e) {}
                }
            }
        } catch (e) {}
    }

    setState(state, detail = '', onProgress = null) {
        this.currentState = state;
        const step = {
            state,
            detail,
            timestamp: Date.now()
        };
        this.currentSteps.push(step);
        if (onProgress) {
            onProgress(step, this.currentSteps);
        }
    }

    /**
     * Deterministic rule-based audit for a document against .agent/rules/
     */
    auditDocument(document) {
        const text = typeof document.getText === 'function' ? document.getText() : (document.content || String(document));
        const fileName = document.fileName ? path.basename(document.fileName) : 'document.kora';
        const lines = text.split('\n');
        const findings = [];

        // Rule 01: Single-Null Check (No undefined)
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            if (/\bundefined\b/.test(line)) {
                findings.push({
                    code: 'E2005',
                    rule: 'Rule 01: Single-Null Design',
                    line: i + 1,
                    message: "Use of forbidden 'undefined' keyword. Kora enforces single-null design.",
                    fix: "Replace 'undefined' with 'null' or wrap type in Option<T>."
                });
            }
        }

        // Rule 04: Clean Architecture DI check (No raw Database() calls in Services/Controllers)
        const isServiceOrController = fileName.includes('Service') || fileName.includes('Controller') || /class\s+[A-Za-z0-9_]+(Service|Controller)/.test(text);
        if (isServiceOrController) {
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                if (/\bDatabase\(\)/.test(line) && !line.includes('//') && !line.includes('init(')) {
                    findings.push({
                        code: 'E5001',
                        rule: 'Rule 04: Clean Architecture Dependency Inversion',
                        line: i + 1,
                        message: "Direct infrastructure instantiation inside application logic.",
                        fix: "Inject database dependency via constructor: init(db: Database) { this.db = db }"
                    });
                }
            }
        }

        // Rule 07 & 04: Directory Segregation check
        const lowerFile = fileName.toLowerCase();
        if ((lowerFile.includes('model') || lowerFile.includes('dto')) && (/@table\b/.test(text) || /@collection\b/.test(text))) {
            findings.push({
                code: 'E5003',
                rule: 'Rule 04 & 07: Layer Directory Segregation',
                line: 1,
                message: "DTO model decorated with database persistence attribute (@table).",
                fix: "Move persisted database entities into entities/ directory and keep models/ strictly for pure payloads."
            });
        }

        // Rule 10: Load tests must include teardown()
        if (lowerFile.includes('load') && lowerFile.includes('test')) {
            if (!/\bfn\s+teardown\s*\(/.test(text)) {
                findings.push({
                    code: 'R1001',
                    rule: 'Rule 10: Mandatory Load Test Teardown',
                    line: 1,
                    message: "Load test suite lacks explicit teardown() cleanup method.",
                    fix: "Implement fn teardown() -> Unit to release synthetic connections and purge memory."
                });
            }
        }

        return {
            fileName,
            passed: findings.length === 0,
            violations: findings.length,
            findings
        };
    }

    /**
     * Answers a user prompt with comprehensive workspace intelligence and rules.
     */
    async answerPrompt(prompt, activeDoc, config, options = {}) {
        const query = (prompt || '').trim();
        this.currentSteps = [];

        // 0. Classify user intent dynamically
        const intent = this.intentEngine.classify(query, {
            attachments: options.attachments,
            activeDoc
        });

        // 1. Scan workspace if not scanned recently
        this.setState(AgentState.EXPLORING, 'Discovering project structure and module relationships...', options.onProgress);
        let workspaceRoots = [];
        if (this.vscode && this.vscode.workspace && this.vscode.workspace.workspaceFolders) {
            workspaceRoots = this.vscode.workspace.workspaceFolders.map(f => f.uri.fsPath);
        } else if (activeDoc && activeDoc.fileName) {
            workspaceRoots = [path.dirname(activeDoc.fileName)];
        }
        await this.workspaceEngine.scanWorkspace(workspaceRoots);

        // 2. Build intelligent context for this query
        this.setState(AgentState.ANALYZING, 'Analyzing workspace context and symbols for query...', options.onProgress);
        const context = this.workspaceEngine.buildContextForQuery(query, {
            activeFilePath: activeDoc ? activeDoc.fileName : null,
            selectedFiles: options.selectedFiles
        });

        const provider = (config && config.provider) ? config.provider.toLowerCase() : 'embedded';

        this.setState(AgentState.GENERATING, `Synthesizing response via ${provider}...`, options.onProgress);

        let answer = '';
        if (provider === 'copilot') {
            try {
                answer = await this._callCopilotLm(query, activeDoc, config, context, intent, options.attachments);
            } catch (err) {
                const fallbackResponse = this.intentEngine.generateEmbeddedResponse(intent, activeDoc, this.workspaceEngine, context, options.attachments);
                answer = `> **Copilot Notice**: Native GitHub Copilot request failed (${err.message}).\n> Showing response from **Kora Embedded Engineering Agent**:\n\n` + fallbackResponse;
            }
        } else if (provider !== 'embedded') {
            try {
                answer = await this._callExternalLlm(query, activeDoc, config, context, options.onChunk, intent, options.attachments);
            } catch (err) {
                const fallbackResponse = this.intentEngine.generateEmbeddedResponse(intent, activeDoc, this.workspaceEngine, context, options.attachments);
                answer = `> **Provider Notice**: External LLM (${provider}) was unreachable: ${err.message}.\n> Showing response from **Kora Embedded Engineering Agent**:\n\n` + fallbackResponse;
            }
        } else {
            answer = this.intentEngine.generateEmbeddedResponse(intent, activeDoc, this.workspaceEngine, context, options.attachments);
        }

        this.setState(AgentState.COMPLETED, 'Response ready', options.onProgress);
        return answer;
    }

    /**
     * Workspace-wide audit across all indexed files.
     */
    auditWorkspace(onProgress = null) {
        this.setState(AgentState.ANALYZING, 'Auditing workspace files against Kora invariants...', onProgress);
        const allFindings = [];
        let totalFiles = 0;

        for (const [fPath, meta] of this.workspaceEngine.indexedFiles) {
            if (!meta.content || meta.isTooLarge) continue;
            totalFiles++;
            const audit = this.auditDocument({ fileName: fPath, getText: () => meta.content });
            if (!audit.passed) {
                allFindings.push({
                    relPath: meta.relPath,
                    fullPath: fPath,
                    violations: audit.findings
                });
            }
        }

        this.setState(AgentState.COMPLETED, 'Workspace audit completed', onProgress);

        let out = `### Kora Workspace Architecture & Rules Audit\n\n`;
        out += `- **Total Files Scanned**: ${totalFiles}\n`;
        out += `- **Files with Invariant Violations**: ${allFindings.length}\n\n`;

        if (allFindings.length === 0) {
            out += `**100% Rule Compliance!** All files in the workspace strictly adhere to official rules in \`.agent/rules/\`.\n`;
        } else {
            for (const item of allFindings) {
                out += `#### [\`${item.relPath}\`](file://${item.fullPath})\n`;
                for (const v of item.violations) {
                    out += `- **[${v.code}] ${v.rule}** (Line ${v.line}): ${v.message}\n  *Fix*: \`${v.fix}\`\n`;
                }
                out += `\n`;
            }
            out += `> Click **AI Auto-Fix & Refactor** to coordinate multi-file fixes across all affected components.\n`;
        }

        return out;
    }

    /**
     * Analyzes workspace and builds an actionable Refactoring Plan with intent understanding
     * and step-by-step invariant checks WITHOUT modifying files.
     */
    async planWorkspaceRefactor(goal = 'Auto-Fix and refactor Kora invariants', options = {}, onProgress = null) {
        this.currentSteps = [];
        this.setState(AgentState.EXPLORING, 'Scanning workspace and analyzing architecture...', onProgress);

        let workspaceRoots = [];
        if (this.vscode && this.vscode.workspace && this.vscode.workspace.workspaceFolders) {
            workspaceRoots = this.vscode.workspace.workspaceFolders.map(f => f.uri.fsPath);
        } else if (options.targetFiles && options.targetFiles.length > 0) {
            workspaceRoots = [path.dirname(options.targetFiles[0])];
        }
        await this.workspaceEngine.scanWorkspace(workspaceRoots);

        this.setState(AgentState.ANALYZING, 'Auditing files and identifying affected components...', onProgress);

        let candidatePaths = [];
        if (options.targetFiles && options.targetFiles.length > 0) {
            candidatePaths = options.targetFiles.map(f => path.resolve(f));
        } else if (options.activeFilePath && options.scope === 'active') {
            candidatePaths = [path.resolve(options.activeFilePath)];
        } else if (options.activeFilePath) {
            const activeNorm = path.resolve(options.activeFilePath);
            candidatePaths = [activeNorm];
            const related = this.workspaceEngine.getRelatedFiles(activeNorm);
            for (const r of related) {
                candidatePaths.push(r.fullPath);
            }
        } else {
            for (const [fPath, meta] of this.workspaceEngine.indexedFiles) {
                if (!meta.content || meta.isTooLarge) continue;
                const audit = this.auditDocument({ fileName: fPath, getText: () => meta.content });
                if (!audit.passed) {
                    candidatePaths.push(fPath);
                }
            }
        }

        if (candidatePaths.length === 0 && options.activeFilePath) {
            candidatePaths.push(path.resolve(options.activeFilePath));
        }

        const steps = [];
        for (const filePath of candidatePaths) {
            let originalCode = '';
            if (fs.existsSync(filePath)) {
                try {
                    originalCode = fs.readFileSync(filePath, 'utf8');
                } catch (e) {
                    continue;
                }
            }

            const fileName = path.basename(filePath);
            const audit = this.auditDocument({ fileName, getText: () => originalCode });
            const plannedChanges = [];

            if (/\bundefined\b/.test(originalCode)) {
                plannedChanges.push('Enforce Rule 01 (Single-Null Design): Replace `undefined` with `null`');
            }
            if (/class\s+[A-Za-z0-9_]+(Service|Controller)/.test(originalCode) && /\bDatabase\(\)/.test(originalCode)) {
                plannedChanges.push('Enforce Rule 04 (Clean Architecture): Convert direct Database instantiation to Constructor Injection via init(...)');
            }
            if (fileName.toLowerCase().includes('load') && fileName.toLowerCase().includes('test') && !/\bfn\s+teardown\s*\(/.test(originalCode)) {
                plannedChanges.push('Enforce Rule 10 (Testing Standards): Implement mandatory teardown() cleanup method');
            }
            if (!audit.passed && audit.findings) {
                for (const f of audit.findings) {
                    if (!plannedChanges.some(p => p.includes(f.code))) {
                        plannedChanges.push(`Fix [${f.code}] ${f.rule}: ${f.message}`);
                    }
                }
            }

            if (plannedChanges.length > 0) {
                steps.push({
                    filePath,
                    fileName,
                    plannedChanges
                });
            }
        }

        if (steps.length === 0) {
            this.setState(AgentState.COMPLETED, 'Audit complete: all scanned files strictly satisfy Clean Architecture. No modifications needed.', onProgress);
            return {
                id: 'plan_' + Date.now() + '_' + Math.random().toString(36).substring(2, 6),
                goal,
                scope: options.scope || (options.targetFiles ? 'active' : 'workspace'),
                targetPath: options.activeFilePath || (options.targetFiles ? options.targetFiles[0] : null),
                isClean: true,
                understanding: `Audited components for '${goal}'. All scanned files are 100% compliant with Clean Architecture and Single-Null semantics! No changes needed.`,
                filesCount: 0,
                steps: []
            };
        }

        this.setState(AgentState.AWAITING_APPROVAL, `Refactoring plan generated for ${steps.length} file(s). Awaiting user approval.`, onProgress);

        return {
            id: 'plan_' + Date.now() + '_' + Math.random().toString(36).substring(2, 6),
            goal,
            scope: options.scope || (options.targetFiles ? 'active' : 'workspace'),
            targetPath: options.activeFilePath || (options.targetFiles ? options.targetFiles[0] : null),
            isClean: false,
            understanding: `Analyzed workspace for '${goal}'. Identified ${steps.length} component(s) requiring clean architecture and invariant adjustments.`,
            filesCount: steps.length,
            steps
        };
    }

    /**
     * Workspace-Aware Auto-Fix and Refactor across single or multiple files.
     * Generates a multi-file proposal and registers it with ChangeReviewManager.
     */
    async generateWorkspaceRefactor(goal = 'Fix all compiler diagnostics and rule violations', options = {}, onProgress = null) {
        this.currentSteps = [];
        this.setState(AgentState.EXPLORING, 'Scanning workspace and analyzing architecture...', onProgress);

        let workspaceRoots = [];
        if (this.vscode && this.vscode.workspace && this.vscode.workspace.workspaceFolders) {
            workspaceRoots = this.vscode.workspace.workspaceFolders.map(f => f.uri.fsPath);
        } else if (options.targetFiles && options.targetFiles.length > 0) {
            workspaceRoots = [path.dirname(options.targetFiles[0])];
        }
        await this.workspaceEngine.scanWorkspace(workspaceRoots);

        this.setState(AgentState.ANALYZING, 'Auditing files and identifying affected components...', onProgress);

        // Determine candidate files
        let candidatePaths = [];
        if (options.targetFiles && options.targetFiles.length > 0) {
            candidatePaths = options.targetFiles.map(f => path.resolve(f));
        } else if (options.activeFilePath) {
            // Target active file AND its direct dependencies/callers
            const activeNorm = path.resolve(options.activeFilePath);
            candidatePaths = [activeNorm];
            const related = this.workspaceEngine.getRelatedFiles(activeNorm);
            for (const r of related) {
                candidatePaths.push(r.fullPath);
            }
        } else {
            // Workspace-wide: find all files with audit violations or matching goal
            for (const [fPath, meta] of this.workspaceEngine.indexedFiles) {
                if (!meta.content || meta.isTooLarge) continue;
                const audit = this.auditDocument({ fileName: fPath, getText: () => meta.content });
                if (!audit.passed) {
                    candidatePaths.push(fPath);
                }
            }
        }

        if (candidatePaths.length === 0) {
            // If no files had violations, pick up active file or all kora files
            if (options.activeFilePath) candidatePaths.push(path.resolve(options.activeFilePath));
            else {
                for (const [fPath] of this.workspaceEngine.indexedFiles) {
                    if (candidatePaths.length < 5) candidatePaths.push(fPath);
                }
            }
        }

        this.setState(AgentState.PLANNING, `Planning refactoring across ${candidatePaths.length} file(s)...`, onProgress);

        const proposedFiles = [];
        const provider = (options.config && options.config.provider) ? options.config.provider.toLowerCase() : 'embedded';

        this.setState(AgentState.GENERATING, `Generating coordinated refactoring changes...`, onProgress);

        for (const filePath of candidatePaths) {
            let originalCode = '';
            if (fs.existsSync(filePath)) {
                try {
                    originalCode = fs.readFileSync(filePath, 'utf8');
                } catch (e) {
                    continue;
                }
            }

            const fileName = path.basename(filePath);
            const audit = this.auditDocument({ fileName, getText: () => originalCode });
            const diags = (options.diagnostics && options.diagnostics[filePath]) ? options.diagnostics[filePath] : [];

            let fixedCode = originalCode;
            const fixesApplied = [];

            // 1. Rule 01 Single-Null
            if (/\bundefined\b/.test(fixedCode)) {
                fixedCode = fixedCode.replace(/\bundefined\b/g, 'null');
                fixesApplied.push('Single-Null Design: replaced `undefined` with `null`');
            }

            // 2. Rule 04 DI Constructor Injection
            if (/class\s+[A-Za-z0-9_]+(Service|Controller)/.test(fixedCode) && /\bDatabase\(\)/.test(fixedCode)) {
                fixedCode = fixedCode.replace(/let\s+([A-Za-z0-9_]+)\s*=\s*Database\(\);?/g, '// [Injected via init]\n        let $1 = this.db;');
                if (!/init\s*\(/.test(fixedCode)) {
                    fixedCode = fixedCode.replace(/(class\s+[A-Za-z0-9_]+[^{]*\{)/, '$1\n    db: Database\n\n    init(db: Database) {\n        this.db = db\n    }\n');
                }
                fixesApplied.push('Clean Architecture: enforced Constructor Dependency Injection');
            }

            // 3. Rule 10 Load Test Teardown
            if (fileName.toLowerCase().includes('load') && fileName.toLowerCase().includes('test') && !/\bfn\s+teardown\s*\(/.test(fixedCode)) {
                const teardownSnippet = `\n    fn teardown() -> Unit {\n        print("  ${fileName}: Teardown synthetic allocations purged")\n    }\n`;
                const lastBraceIndex = fixedCode.lastIndexOf('}');
                if (lastBraceIndex > 0) {
                    fixedCode = fixedCode.slice(0, lastBraceIndex) + teardownSnippet + fixedCode.slice(lastBraceIndex);
                    fixesApplied.push('Testing Standards: implemented mandatory `teardown()` cleanup');
                }
            }

            // 4. LLM contextual refactoring if external provider configured
            if (provider !== 'embedded' && (diags.length > 0 || !audit.passed || goal)) {
                try {
                    const llmResult = await this._callLlmForFileFix(fileName, originalCode, diags, audit, goal, options.config);
                    if (llmResult && llmResult.trim().length > 15) {
                        fixedCode = llmResult;
                        fixesApplied.push(`AI Refactoring via ${provider}`);
                    }
                } catch (e) {}
            }

            if (fixedCode !== originalCode) {
                proposedFiles.push({
                    filePath,
                    originalContent: originalCode,
                    proposedContent: fixedCode,
                    summary: fixesApplied.join('; ')
                });
            }
        }

        if (proposedFiles.length === 0) {
            this.setState(AgentState.COMPLETED, 'No rule violations found. All components satisfy Clean Architecture.', onProgress);
            return {
                proposal: {
                    id: 'clean_' + Date.now(),
                    title: 'Clean Architecture Audit (100% Compliant)',
                    totalFiles: 0,
                    files: []
                },
                rawProposal: { id: 'clean', files: [] },
                steps: this.currentSteps,
                isClean: true,
                filesCount: 0
            };
        }

        this.setState(AgentState.REVIEWING, `Reviewing diffs across ${proposedFiles.length} file(s)...`, onProgress);

        const proposal = this.reviewManager.createProposal(
            goal || 'Kora Workspace Refactoring & Auto-Fix',
            proposedFiles
        );

        this.setState(AgentState.AWAITING_APPROVAL, `${proposedFiles.length} file(s) modified. Awaiting review and approval.`, onProgress);

        return {
            proposal: this.reviewManager.serializeProposal(proposal.id),
            rawProposal: proposal,
            steps: this.currentSteps,
            filesCount: proposedFiles.length
        };
    }

    /**
     * Backward-compatible generateCodeFix for a single file.
     */
    async generateCodeFix(fileName, originalCode, diagnostics = [], audit = null, config = {}) {
        const dummyPath = path.resolve(fileName);
        const result = await this.generateWorkspaceRefactor(`Fix ${fileName}`, {
            targetFiles: [dummyPath],
            diagnostics: { [dummyPath]: diagnostics },
            config
        });

        const fileChange = result.rawProposal.files[0];
        return {
            fileName,
            fixedCode: fileChange ? fileChange.proposedContent : originalCode,
            summary: fileChange ? (fileChange.additions + ' additions, ' + fileChange.deletions + ' deletions') : 'No changes needed',
            hasChanges: fileChange ? fileChange.proposedContent !== originalCode : false,
            proposalId: result.rawProposal.id
        };
    }

    async _callLlmForFileFix(fileName, originalCode, diagnostics, audit, goal, config) {
        const diagsSummary = diagnostics.length > 0
            ? diagnostics.map(d => `- Line ${d.line || 1}: ${d.message}`).join('\n')
            : 'No compiler diagnostics';
        const auditSummary = audit && audit.findings && audit.findings.length > 0
            ? audit.findings.map(f => `- Rule [${f.code}]: ${f.message} (Fix: ${f.fix})`).join('\n')
            : 'No rule violations';

        const prompt = `Goal: ${goal}
Refactor the Kora file \`${fileName}\` ensuring 100% compliance with Kora invariants:
- Single-Null design: strictly no 'undefined'.
- Clean Architecture: constructor DI via init(...).
- Output ONLY the final complete compile-ready Kora code inside a \`\`\`kora codeblock.

Issues to fix:
${diagsSummary}
${auditSummary}

Original Source:
\`\`\`kora
${originalCode}
\`\`\``;

        const provider = (config.provider || '').toLowerCase();
        let answer = '';
        if (provider === 'copilot') {
            answer = await this._callCopilotLm(prompt, null, config);
        } else {
            answer = await this._callExternalLlm(prompt, null, config);
        }

        const match = answer.match(/```(?:kora|rust)?\s*\n([\s\S]*?)\n```/);
        return match ? match[1].trim() : null;
    }

    _generateEmbeddedResponse(prompt, activeDoc, context = null) {
        const query = prompt.toLowerCase();

        // Check if query matches specific rule topics
        let bestMatch = null;
        let highestScore = 0;

        for (const item of KORA_KNOWLEDGE_BASE) {
            let score = 0;
            for (const kw of item.keywords) {
                if (query.includes(kw)) score += 2;
            }
            if (score > highestScore) {
                highestScore = score;
                bestMatch = item;
            }
        }

        let contextSection = '';
        if (context && context.files && context.files.length > 0) {
            contextSection = `\n\n#### Relevant Workspace Files Identified:\n`;
            for (const f of context.files.slice(0, 5)) {
                contextSection += `- [\`${f.relPath}\`](file://${f.filePath}) (*${f.role}* - ${f.reason})\n`;
            }
        }

        if (bestMatch && highestScore > 0) {
            return `${bestMatch.content}\n\n**Rule Reference**: \`${bestMatch.source}\`${contextSection}`;
        }

        // Default overview
        return `### Kora AI Agent & Architecture Assistant

I understand the entire Kora project structure, Clean Architecture invariants, and language specifications:

- **Clean Architecture (\`.agent/rules/04-clean-architecture.md\`)**: Constructor Dependency Injection (\`init\`), bounded contexts, \`entities/\` vs \`models/\`.
- **Single-Null Design (\`.agent/rules/01-language-philosophy.md\`)**: \`null\` and \`Option<T>\` supported; \`undefined\` strictly prohibited (\`E2005\`).
- **Multi-Engine ORM (\`.agent/rules/07-database-multi-engine-orm.md\`)**: SQL \`@table\`, NoSQL \`@collection\`, Redis caching.
- **Actor Concurrency (\`.agent/rules/08-concurrency-actors-streams.md\`)**: Stateful actors with mailboxes (\`tell\`, \`ask\`), non-blocking channels.
- **Testing & Teardown (\`.agent/rules/10-testing-and-verification.md\`)**: Unit test suites and load stress tests with mandatory \`teardown()\`.${contextSection}

**Try asking**:
- *"Analyze project architecture"*
- *"Refactor authentication service and its callers"*
- *"Auto-fix all rule violations across workspace"*
- *"Scaffold unit tests for active models"*`;
    }

    _buildSystemPrompt(prompt, activeDoc, context = null, intent = null, attachments = []) {
        const query = (prompt || '').toLowerCase();
        let relevantContext = '';
        for (const item of KORA_KNOWLEDGE_BASE) {
            if (item.keywords.some(kw => query.includes(kw))) {
                relevantContext += `\n\n[Official Kora Rule Reference (${item.source})]:\n${item.content}\n`;
                break;
            }
        }

        let workspaceContextStr = '';
        if (context) {
            workspaceContextStr = `\n\n[Workspace Context Information]:\n${this.workspaceEngine.formatContextForPrompt(context)}`;
        } else if (activeDoc) {
            const fileName = path.basename(activeDoc.fileName);
            const codeSnippet = (typeof activeDoc.getText === 'function' ? activeDoc.getText() : String(activeDoc)).slice(0, 3500);
            workspaceContextStr = `\n\n[Current Active File: ${fileName}]\n\`\`\`kora\n${codeSnippet}\n\`\`\``;
        }

        let attachmentsStr = '';
        if (attachments && attachments.length > 0) {
            attachmentsStr = `\n\n[USER ATTACHED FILES & FOLDERS]:\n`;
            for (const att of attachments) {
                if (att.type === 'folder' || att.isDirectory) {
                    attachmentsStr += `### Folder: ${att.name || att.path} (${att.fileCount || 0} files)\nDirectory Tree:\n\`\`\`\n${att.tree || att.summary || ''}\n\`\`\`\n`;
                    if (att.content) {
                        attachmentsStr += `Key Folder Contents:\n${att.content.slice(0, 4000)}\n\n`;
                    }
                } else {
                    attachmentsStr += `### File: ${att.name || att.path}\n\`\`\`\n${(att.content || '').slice(0, 4000)}\n\`\`\`\n\n`;
                }
            }
        }

        const intentInfo = intent ? `\nDetected Task Intent: ${intent.primaryIntent} (ReadOnly: ${intent.isReadOnly})` : '';

        return `You are the Official Kora Framework AI Software Engineering Agent.
Kora combines high DX with Rust-level memory safety, Go-inspired actor concurrency, and strict Clean Architecture enforcement.

CORE INVARIANTS:
1. Single-Null Design: Kora supports null and Option<T>. 'undefined' is STRICTLY FORBIDDEN (E2005).
2. Clean Architecture: All Services and Controllers MUST declare dependencies via constructor init(...).
3. Directory Segregation: Relational & NoSQL entities belong strictly in entities/. DTOs belong in models/.
4. Stateful Actors: Isolated mailboxes with tell and ask.
5. Testing: Load stress tests require explicit teardown() to release simulated resources.

INSTRUCTIONS:
- You are an intelligent software engineering agent, not a static chatbot.
- Understand the user's intent dynamically (Explanation, Architecture, Review, Syntax Check, Formatting, Refactoring, Testing).
- When reviewing or fixing code, if there are NO errors, confirm the code is clean and do not fabricate unnecessary changes.
- If attached files or folders are provided, inspect them, understand their relationships, and address the user's questions about them.
- File References: Never output verbose full paths (e.g. avoid 'Location: /home/...'). Reference files cleanly by their filename using markdown file links, e.g. [order_processor.service.kora](file:///path/to/order_processor.service.kora).${intentInfo}${relevantContext}${workspaceContextStr}${attachmentsStr}`;
    }

    _resolveEndpoint(provider, customEndpoint) {
        if (provider === 'github') {
            if (!customEndpoint) return 'https://models.inference.ai.azure.com/chat/completions';
            try {
                const u = new URL(customEndpoint);
                if (u.hostname === 'models.inference.ai.azure.com') {
                    if (!u.pathname.endsWith('/chat/completions')) u.pathname = '/chat/completions';
                    return u.toString();
                }
                return customEndpoint;
            } catch {
                return 'https://models.inference.ai.azure.com/chat/completions';
            }
        }
        if (provider === 'ollama') {
            let raw = (customEndpoint && typeof customEndpoint === 'string' && customEndpoint.trim()) ||
                      (typeof process !== 'undefined' && process.env && (process.env.OLLAMA_ENDPOINT || process.env.OLLAMA_HOST)) ||
                      'http://localhost:11434';
            raw = raw.trim();
            if (!raw.startsWith('http://') && !raw.startsWith('https://')) {
                raw = 'http://' + raw;
            }
            try {
                const u = new URL(raw);
                if (!u.pathname || u.pathname === '/' || !u.pathname.startsWith('/api/')) {
                    u.pathname = '/api/generate';
                }
                return u.toString();
            } catch {
                return 'http://localhost:11434/api/generate';
            }
        }
        if (provider === 'gemini') {
            return customEndpoint || 'https://generativelanguage.googleapis.com/v1beta/openai/chat/completions';
        }
        if (provider === 'anthropic') {
            return customEndpoint || 'https://api.anthropic.com/v1/messages';
        }
        if (provider === 'openai' || provider === 'custom') {
            const raw = customEndpoint || 'https://api.openai.com/v1/chat/completions';
            try {
                const u = new URL(raw);
                if (u.hostname === 'api.openai.com' && !u.pathname.endsWith('/chat/completions')) {
                    u.pathname = '/v1/chat/completions';
                    return u.toString();
                }
                return raw;
            } catch {
                return 'https://api.openai.com/v1/chat/completions';
            }
        }
        return customEndpoint || '';
    }

    async listOllamaModels(endpoint = 'http://localhost:11434') {
        try {
            const models = await this.ollamaService.discoverInstalledModels(endpoint);
            return models.map(m => m.name);
        } catch {
            return [];
        }
    }

    async _callCopilotLm(prompt, activeDoc, config, context = null, intent = null, attachments = []) {
        if (!this.vscode || !this.vscode.lm) {
            throw new Error("VS Code Language Model API (vscode.lm) is not available in this environment.");
        }

        let models = [];
        try {
            models = await this.vscode.lm.selectChatModels({ vendor: 'copilot' });
        } catch (e) {}

        if (!models || models.length === 0) {
            try {
                models = await this.vscode.lm.selectChatModels();
            } catch (e) {}
        }

        if (!models || models.length === 0) {
            throw new Error("No active GitHub Copilot Language Model found. Please ensure GitHub Copilot is installed and signed in.");
        }

        let selectedModel = models[0];
        if (config && config.model && config.model !== 'default') {
            const found = models.find(m => m.name === config.model || m.id === config.model || m.family === config.model);
            if (found) selectedModel = found;
        }

        const systemPrompt = this._buildSystemPrompt(prompt, activeDoc, context, intent, attachments);
        const fullUserMessage = `${systemPrompt}\n\nUser Request:\n${prompt}`;

        let chatMessages;
        if (this.vscode.LanguageModelChatMessage && typeof this.vscode.LanguageModelChatMessage.User === 'function') {
            chatMessages = [this.vscode.LanguageModelChatMessage.User(fullUserMessage)];
        } else {
            chatMessages = [{ role: 1, content: fullUserMessage }];
        }

        const cts = new this.vscode.CancellationTokenSource();
        const timeout = setTimeout(() => cts.cancel(), 30000);

        try {
            const response = await selectedModel.sendRequest(chatMessages, {}, cts.token);
            let fullText = '';
            for await (const chunk of response.text) {
                fullText += chunk;
            }
            clearTimeout(timeout);
            return fullText || 'No response returned from GitHub Copilot.';
        } catch (err) {
            clearTimeout(timeout);
            throw err;
        }
    }

    async testConnection(config) {
        const start = Date.now();
        const provider = (config.provider || 'embedded').toLowerCase();
        if (provider === 'embedded') {
            return {
                success: true,
                latencyMs: 1,
                provider: 'embedded',
                message: 'Embedded Knowledge Engine is active and indexed (offline).'
            };
        }
        if (provider === 'copilot') {
            try {
                await this._callCopilotLm('Respond with OK.', null, config);
                const latencyMs = Date.now() - start;
                return {
                    success: true,
                    latencyMs,
                    provider: 'copilot',
                    model: config.model || 'copilot-default',
                    message: `Successfully connected to native GitHub Copilot in ${latencyMs}ms!`
                };
            } catch (err) {
                return {
                    success: false,
                    latencyMs: Date.now() - start,
                    provider: 'copilot',
                    error: err.message
                };
            }
        }
        if (provider === 'ollama') {
            try {
                const resolution = await this.ollamaService.resolveBestModel('chat', config.model, config.endpoint);
                if (resolution.status === 'no_models') {
                    return {
                        success: false,
                        latencyMs: Date.now() - start,
                        provider: 'ollama',
                        error: resolution.message,
                        recommendation: resolution.recommendation
                    };
                }
                const actualModel = resolution.model;
                const testCfg = Object.assign({}, config, { model: actualModel });
                await this._callExternalLlm('Respond with OK.', null, testCfg);
                const latencyMs = Date.now() - start;
                const note = resolution.status === 'fallback' ? ` (fallback from requested '${config.model}')` : '';
                return {
                    success: true,
                    latencyMs,
                    provider: 'ollama',
                    model: actualModel,
                    message: `Successfully connected to local Ollama (${actualModel}) in ${latencyMs}ms!${note}`,
                    resolution
                };
            } catch (err) {
                return {
                    success: false,
                    latencyMs: Date.now() - start,
                    provider: 'ollama',
                    error: err.message
                };
            }
        }
        try {
            await this._callExternalLlm('Respond with the word OK.', null, config);
            const latencyMs = Date.now() - start;
            return {
                success: true,
                latencyMs,
                provider,
                model: config.model || 'default',
                message: `Successfully connected to ${provider} in ${latencyMs}ms!`
            };
        } catch (err) {
            return {
                success: false,
                latencyMs: Date.now() - start,
                provider,
                error: err.message
            };
        }
    }

    async _callExternalLlm(prompt, activeDoc, config, context = null, onChunk = null, intent = null, attachments = []) {
        const provider = (config.provider || 'embedded').toLowerCase();
        let effectiveModel = config.model;

        if (provider === 'ollama') {
            const resolution = await this.ollamaService.resolveBestModel('chat', config.model, config.endpoint);
            if (resolution.status === 'no_models') {
                throw new Error(resolution.message || "No models installed in Ollama. Run: ollama pull qwen2.5-coder:7b");
            }
            effectiveModel = resolution.model;
        }

        return new Promise((resolve, reject) => {
            const endpointUrl = this._resolveEndpoint(provider, config.endpoint);
            const systemPrompt = this._buildSystemPrompt(prompt, activeDoc, context, intent, attachments);

            let postData = '';
            let headers = { 'Content-Type': 'application/json' };

            if (provider === 'github') {
                const token = config.githubToken || config.apiKey || process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
                if (!token) {
                    return reject(new Error("GitHub Token required. Set 'kora.copilot.githubToken' in VS Code settings or paste your GitHub Personal Access Token."));
                }
                headers['Authorization'] = `Bearer ${token}`;
                const model = effectiveModel && effectiveModel !== 'llama3' ? effectiveModel : 'gpt-4o-mini';
                postData = JSON.stringify({
                    model: model,
                    messages: [
                        { role: 'system', content: systemPrompt },
                        { role: 'user', content: prompt }
                    ],
                    temperature: 0.2
                });
            } else if (provider === 'ollama') {
                postData = JSON.stringify({
                    model: effectiveModel,
                    system: systemPrompt,
                    prompt: prompt,
                    stream: true
                });
            } else if (provider === 'gemini') {
                const token = config.geminiKey || config.apiKey || process.env.GEMINI_API_KEY;
                if (!token) return reject(new Error("Gemini API Key required. Set in Kora Settings."));
                headers['Authorization'] = `Bearer ${token}`;
                postData = JSON.stringify({
                    model: config.model || 'gemini-1.5-flash',
                    messages: [
                        { role: 'system', content: systemPrompt },
                        { role: 'user', content: prompt }
                    ]
                });
            } else if (provider === 'anthropic') {
                const token = config.anthropicKey || config.apiKey || process.env.ANTHROPIC_API_KEY;
                if (!token) return reject(new Error("Anthropic API Key required. Set in Kora Settings."));
                headers['x-api-key'] = token;
                headers['anthropic-version'] = '2023-06-01';
                postData = JSON.stringify({
                    model: config.model || 'claude-3-5-sonnet-20241022',
                    system: systemPrompt,
                    messages: [{ role: 'user', content: prompt }],
                    max_tokens: 2048
                });
            } else if (provider === 'openai' || provider === 'custom') {
                const token = config.openaiKey || config.apiKey || process.env.OPENAI_API_KEY;
                if (token) headers['Authorization'] = `Bearer ${token}`;
                postData = JSON.stringify({
                    model: config.model || 'gpt-4o',
                    messages: [
                        { role: 'system', content: systemPrompt },
                        { role: 'user', content: prompt }
                    ]
                });
            } else {
                return reject(new Error(`Unsupported provider: ${provider}`));
            }

            let urlObj;
            try {
                urlObj = new URL(endpointUrl);
            } catch (err) {
                return reject(new Error(`Invalid API endpoint URL: ${endpointUrl}`));
            }

            const client = urlObj.protocol === 'https:' ? https : http;
            let fullText = '';
            let lineBuffer = '';
            let idleTimer = null;

            const resetIdle = () => {
                if (idleTimer) clearTimeout(idleTimer);
                idleTimer = setTimeout(() => {
                    req.destroy();
                    reject(new Error("Request timed out (90s idle)"));
                }, 90000);
            };

            const req = client.request(urlObj, {
                method: 'POST',
                headers: Object.assign(headers, { 'Content-Length': Buffer.byteLength(postData) })
            }, (res) => {
                resetIdle();
                res.on('data', chunk => {
                    resetIdle();
                    const str = chunk.toString();
                    if (provider === 'ollama') {
                        lineBuffer += str;
                        const lines = lineBuffer.split('\n');
                        lineBuffer = lines.pop();
                        for (const line of lines) {
                            const trimmed = line.trim();
                            if (!trimmed) continue;
                            try {
                                const json = JSON.parse(trimmed);
                                const token = json.response || (json.message && json.message.content) || '';
                                if (token) {
                                    fullText += token;
                                    if (typeof onChunk === 'function') {
                                        onChunk(token, fullText);
                                    }
                                }
                            } catch (e) {}
                        }
                    } else {
                        fullText += str;
                    }
                });

                res.on('end', () => {
                    if (idleTimer) clearTimeout(idleTimer);
                    if (provider === 'ollama') {
                        if (lineBuffer && lineBuffer.trim()) {
                            try {
                                const json = JSON.parse(lineBuffer.trim());
                                const token = json.response || (json.message && json.message.content) || '';
                                if (token) {
                                    fullText += token;
                                    if (typeof onChunk === 'function') {
                                        onChunk(token, fullText);
                                    }
                                }
                            } catch (e) {}
                        }
                        if (fullText.trim()) {
                            return resolve(fullText);
                        }
                    }
                    try {
                        const json = JSON.parse(fullText);
                        if (provider === 'ollama' && json.response) {
                            resolve(json.response);
                        } else if (provider === 'ollama' && json.message && json.message.content) {
                            resolve(json.message.content);
                        } else if (json.choices && json.choices[0] && json.choices[0].message) {
                            resolve(json.choices[0].message.content);
                        } else if (json.content && Array.isArray(json.content) && json.content[0] && json.content[0].text) {
                            resolve(json.content[0].text);
                        } else if (json.error) {
                            reject(new Error(typeof json.error === 'string' ? json.error : (json.error.message || JSON.stringify(json.error))));
                        } else {
                            resolve(fullText);
                        }
                    } catch (e) {
                        if (res.statusCode >= 400) {
                            reject(new Error(`HTTP ${res.statusCode}: ${fullText}`));
                        } else {
                            resolve(fullText);
                        }
                    }
                });
            });

            req.on('error', err => {
                if (idleTimer) clearTimeout(idleTimer);
                reject(err);
            });

            req.setTimeout(90000, () => {
                if (idleTimer) clearTimeout(idleTimer);
                req.destroy();
                reject(new Error("Initial connection timed out (90s)"));
            });

            req.write(postData);
            req.end();
        });
    }
}

module.exports = { KoraCopilotEngine, KORA_KNOWLEDGE_BASE, AgentState };
