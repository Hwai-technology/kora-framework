//! Language Server Protocol (LSP) Daemon for Kora.
//! Provides real-time IDE diagnostics, semantic symbol index, Go To Definition,
//! Hover type documentation, Find References, Document Symbols (Outline),
//! intelligent Autocompletion, and Code Formatting via JSON-RPC 2.0.

#![allow(clippy::collapsible_if)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use kora_ast::{Item, Module};
use kora_diagnostics::DiagnosticBag;
use kora_fmt::Formatter;
use kora_parser::parse_module;
use kora_span::{FileId, LineCol, SourceFile, Span};
use serde::{Deserialize, Serialize};

// Thread-safe in-memory document cache: URI -> Source text
static DOCUMENT_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn with_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut HashMap<String, String>) -> R,
{
    let mut guard = DOCUMENT_CACHE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    f(guard.as_mut().unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LspRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LspResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

pub struct LanguageServer;

impl LanguageServer {
    pub fn handle_message(raw_json: &str) -> Option<String> {
        let req: LspRequest = serde_json::from_str(raw_json).ok()?;
        let resp = match req.method.as_str() {
            "initialize" => LspResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: Some(serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": 1, // Full document sync
                        "hoverProvider": true,
                        "completionProvider": {
                            "triggerCharacters": [".", "@", ":", "<", "\""]
                        },
                        "definitionProvider": true,
                        "referencesProvider": true,
                        "documentSymbolProvider": true,
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {
                        "name": "kora-lsp",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            },
            "shutdown" => LspResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: Some(serde_json::Value::Null),
                error: None,
            },
            "textDocument/didOpen" => {
                if let Some(params) = req.params {
                    if let Some(doc) = params.get("textDocument") {
                        if let (Some(uri), Some(text)) = (
                            doc.get("uri").and_then(|u| u.as_str()),
                            doc.get("text").and_then(|t| t.as_str()),
                        ) {
                            with_cache(|cache| {
                                cache.insert(uri.to_string(), text.to_string());
                            });
                        }
                    }
                }
                return None;
            }
            "textDocument/didChange" => {
                if let Some(params) = req.params {
                    let uri = params
                        .get("textDocument")
                        .and_then(|d| d.get("uri"))
                        .and_then(|u| u.as_str());
                    let text = params
                        .get("contentChanges")
                        .and_then(|c| c.get(0))
                        .and_then(|ch| ch.get("text"))
                        .and_then(|t| t.as_str());
                    if let (Some(uri), Some(text)) = (uri, text) {
                        with_cache(|cache| {
                            cache.insert(uri.to_string(), text.to_string());
                        });
                    }
                }
                return None;
            }
            "textDocument/completion" => {
                let mut completions = get_static_completions();

                // Dynamic completion: add symbols declared in this document
                if let Some(params) = &req.params {
                    if let Some(uri) = params
                        .get("textDocument")
                        .and_then(|d| d.get("uri"))
                        .and_then(|u| u.as_str())
                    {
                        let doc_text = get_doc_text(uri);
                        if let Some(src) = doc_text {
                            let mut diags = DiagnosticBag::new();
                            let module = parse_module(FileId(0), &src, &mut diags);
                            append_module_symbols_to_completions(&module, &mut completions);
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(serde_json::Value::Array(completions)),
                    error: None,
                }
            }
            "textDocument/hover" => {
                let mut hover_val = serde_json::Value::Null;
                if let Some(params) = &req.params {
                    if let (Some(uri), Some(pos)) = (
                        params
                            .get("textDocument")
                            .and_then(|d| d.get("uri"))
                            .and_then(|u| u.as_str()),
                        params.get("position"),
                    ) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let character =
                            pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
                        if let Some(src) = get_doc_text(uri) {
                            if let Some(markdown) = compute_hover(&src, line, character) {
                                hover_val = serde_json::json!({
                                    "contents": {
                                        "kind": "markdown",
                                        "value": markdown
                                    }
                                });
                            }
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(hover_val),
                    error: None,
                }
            }
            "textDocument/definition" => {
                let mut location = serde_json::Value::Null;
                if let Some(params) = &req.params {
                    if let (Some(uri), Some(pos)) = (
                        params
                            .get("textDocument")
                            .and_then(|d| d.get("uri"))
                            .and_then(|u| u.as_str()),
                        params.get("position"),
                    ) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let character =
                            pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
                        if let Some(src) = get_doc_text(uri) {
                            if let Some(loc) = find_definition(uri, &src, line, character) {
                                location = loc;
                            }
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(location),
                    error: None,
                }
            }
            "textDocument/references" => {
                let mut refs = Vec::new();
                if let Some(params) = &req.params {
                    if let (Some(uri), Some(pos)) = (
                        params
                            .get("textDocument")
                            .and_then(|d| d.get("uri"))
                            .and_then(|u| u.as_str()),
                        params.get("position"),
                    ) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let character =
                            pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
                        if let Some(src) = get_doc_text(uri) {
                            refs = find_references(uri, &src, line, character);
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(serde_json::Value::Array(refs)),
                    error: None,
                }
            }
            "textDocument/documentSymbol" => {
                let mut symbols = Vec::new();
                if let Some(params) = &req.params {
                    if let Some(uri) = params
                        .get("textDocument")
                        .and_then(|d| d.get("uri"))
                        .and_then(|u| u.as_str())
                    {
                        if let Some(src) = get_doc_text(uri) {
                            let source_file = SourceFile::new(FileId(0), uri, &src);
                            let mut diags = DiagnosticBag::new();
                            let module = parse_module(FileId(0), &src, &mut diags);
                            symbols = extract_document_symbols(&module, &source_file);
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(serde_json::Value::Array(symbols)),
                    error: None,
                }
            }
            "textDocument/formatting" => {
                let mut edits = Vec::new();
                if let Some(params) = &req.params {
                    if let Some(uri) = params
                        .get("textDocument")
                        .and_then(|d| d.get("uri"))
                        .and_then(|u| u.as_str())
                    {
                        if let Some(src) = get_doc_text(uri) {
                            let formatted = Formatter::format_source(&src);
                            if formatted != src {
                                let lines: Vec<&str> = src.lines().collect();
                                let last_line = lines.len().saturating_sub(1);
                                let last_char = lines.last().map(|l| l.len()).unwrap_or(0);
                                edits.push(serde_json::json!({
                                    "range": {
                                        "start": { "line": 0, "character": 0 },
                                        "end": { "line": last_line, "character": last_char }
                                    },
                                    "newText": formatted
                                }));
                            }
                        }
                    }
                }

                LspResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(serde_json::Value::Array(edits)),
                    error: None,
                }
            }
            _ => return None,
        };
        serde_json::to_string(&resp).ok()
    }
}

/// Retrieve text either from in-memory cache or by reading from local disk if it's a file:// URI.
fn get_doc_text(uri: &str) -> Option<String> {
    if let Some(cached) = with_cache(|c| c.get(uri).cloned()) {
        return Some(cached);
    }
    if let Some(path_str) = uri.strip_prefix("file://") {
        return fs::read_to_string(path_str).ok();
    }
    None
}

/// Converts a 1-based `LineCol` into a 0-based LSP position JSON object.
fn line_col_to_lsp_pos(lc: LineCol) -> serde_json::Value {
    serde_json::json!({
        "line": lc.line.saturating_sub(1),
        "character": lc.col.saturating_sub(1)
    })
}

/// Converts an AST span into an LSP range JSON object.
fn span_to_lsp_range(source_file: &SourceFile, span: Span) -> serde_json::Value {
    let start_lc = source_file.line_col(span.start);
    let end_lc = source_file.line_col(span.end);
    serde_json::json!({
        "start": line_col_to_lsp_pos(start_lc),
        "end": line_col_to_lsp_pos(end_lc)
    })
}

/// Extracts full DocumentSymbol hierarchy from parsed Module AST.
fn extract_document_symbols(module: &Module, sf: &SourceFile) -> Vec<serde_json::Value> {
    let mut symbols = Vec::new();

    for item in &module.items {
        match item {
            Item::Model(m) => {
                let range = span_to_lsp_range(sf, m.span);
                let sel_range = span_to_lsp_range(sf, m.name.span);
                let mut children = Vec::new();
                for f in &m.fields {
                    children.push(serde_json::json!({
                        "name": f.name.name,
                        "detail": "Field",
                        "kind": 8, // Field
                        "range": span_to_lsp_range(sf, f.span),
                        "selectionRange": span_to_lsp_range(sf, f.name.span)
                    }));
                }
                symbols.push(serde_json::json!({
                    "name": m.name.name,
                    "detail": "model (DTO / Entity)",
                    "kind": 23, // Struct
                    "range": range,
                    "selectionRange": sel_range,
                    "children": children
                }));
            }
            Item::Class(c) => {
                let range = span_to_lsp_range(sf, c.span);
                let sel_range = span_to_lsp_range(sf, c.name.span);
                let mut children = Vec::new();
                for f in &c.fields {
                    children.push(serde_json::json!({
                        "name": f.name.name,
                        "detail": "Field",
                        "kind": 8, // Field
                        "range": span_to_lsp_range(sf, f.span),
                        "selectionRange": span_to_lsp_range(sf, f.name.span)
                    }));
                }
                for m in &c.methods {
                    children.push(serde_json::json!({
                        "name": m.sig.name.name,
                        "detail": "Method",
                        "kind": 6, // Method
                        "range": span_to_lsp_range(sf, m.span),
                        "selectionRange": span_to_lsp_range(sf, m.sig.name.span)
                    }));
                }
                symbols.push(serde_json::json!({
                    "name": c.name.name,
                    "detail": "class (Service / Controller)",
                    "kind": 5, // Class
                    "range": range,
                    "selectionRange": sel_range,
                    "children": children
                }));
            }
            Item::Interface(i) => {
                let range = span_to_lsp_range(sf, i.span);
                let sel_range = span_to_lsp_range(sf, i.name.span);
                let mut children = Vec::new();
                for m in &i.methods {
                    children.push(serde_json::json!({
                        "name": m.name.name,
                        "detail": "Method Signature",
                        "kind": 6, // Method
                        "range": span_to_lsp_range(sf, m.span),
                        "selectionRange": span_to_lsp_range(sf, m.name.span)
                    }));
                }
                symbols.push(serde_json::json!({
                    "name": i.name.name,
                    "detail": "interface (Contract)",
                    "kind": 11, // Interface
                    "range": range,
                    "selectionRange": sel_range,
                    "children": children
                }));
            }
            Item::Enum(e) => {
                let range = span_to_lsp_range(sf, e.span);
                let sel_range = span_to_lsp_range(sf, e.name.span);
                let mut children = Vec::new();
                for v in &e.variants {
                    children.push(serde_json::json!({
                        "name": v.name.name,
                        "detail": "Variant",
                        "kind": 22, // EnumMember
                        "range": span_to_lsp_range(sf, v.span),
                        "selectionRange": span_to_lsp_range(sf, v.name.span)
                    }));
                }
                symbols.push(serde_json::json!({
                    "name": e.name.name,
                    "detail": "enum",
                    "kind": 10, // Enum
                    "range": range,
                    "selectionRange": sel_range,
                    "children": children
                }));
            }
            Item::Fn(f) => {
                symbols.push(serde_json::json!({
                    "name": f.sig.name.name,
                    "detail": "Function",
                    "kind": 12, // Function
                    "range": span_to_lsp_range(sf, f.span),
                    "selectionRange": span_to_lsp_range(sf, f.sig.name.span)
                }));
            }
            _ => {}
        }
    }

    symbols
}

/// Extracts word under 0-based line/character cursor in a source text.
fn extract_word_at_pos(src: &str, line_idx: usize, char_idx: usize) -> Option<String> {
    let line = src.lines().nth(line_idx)?;
    let chars: Vec<char> = line.chars().collect();
    if char_idx > chars.len() {
        return None;
    }

    let is_ident_char = |c: char| c.is_alphanumeric() || c == '_' || c == '@';

    let mut start = char_idx;
    while start > 0 && is_ident_char(chars[start - 1]) {
        start -= 1;
    }

    let mut end = char_idx;
    while end < chars.len() && is_ident_char(chars[end]) {
        end += 1;
    }

    if start == end {
        None
    } else {
        Some(chars[start..end].iter().collect())
    }
}

/// Computes Markdown hover card for token under cursor.
fn compute_hover(src: &str, line: usize, char_idx: usize) -> Option<String> {
    let word = extract_word_at_pos(src, line, char_idx)?;

    let doc = match word.as_str() {
        "model" => {
            "**Kora `model` Declaration**\n\nPure, immutable Data Transfer Object (DTO) or database entity. Models never hold behavior or stateful methods and compile to memory-efficient native Rust structs with zero GC overhead."
        }
        "class" => {
            "**Kora `class` Declaration**\n\nService, Controller, or Repository class with compile-time Dependency Injection (DI). All parameters in `init(...)` are resolved automatically at compile time."
        }
        "interface" => {
            "**Kora `interface` Declaration**\n\nBehavioral contract defining method signatures. Enforced strictly at compile time; classes declare conformance using `implements`."
        }
        "enum" => {
            "**Kora `enum` Declaration**\n\nType-safe enumerated type or algebraic data type. Variants can carry typed payloads and are checked exhaustively in `match` expressions."
        }
        "actor" => {
            "**Kora `actor` Concurrency**\n\nIsolated stateful concurrency unit with a non-blocking mailbox. State mutations occur sequentially in an isolated Tokio worker thread via `tell` and `ask`."
        }
        "channel" => {
            "**Kora `channel<T>(capacity)`**\n\nNon-blocking MPSC channel for asynchronous message passing between concurrent tasks."
        }
        "broadcast" => {
            "**Kora `broadcast<T>(capacity)`**\n\nMulti-producer, multi-consumer broadcast channel for pub/sub domain events."
        }
        "spawn" => {
            "**Kora `spawn(async () => { ... })`**\n\nSpawns an asynchronous concurrent task executed on high-performance native OS worker threads."
        }
        "try" | "catch" | "finally" | "throw" => {
            "**Kora Error Resilience**\n\nStructured error handling syntax. `try { ... } catch (err) { ... } finally { ... }` ensures reliable resource cleanup and controlled error propagation."
        }
        "for" | "in" => {
            "**Kora `for .. in` Loop**\n\nIterate over collections, arrays, or numeric ranges (e.g. `for i in 1..10`). Supports `break` and `continue` control flow."
        }
        "const" => {
            "**Kora `const` Declaration**\n\nImmutable compile-time constant evaluated during compilation."
        }
        "let" => "**Kora `let` Declaration**\n\nDeclares an immutable variable binding.",
        "var" => "**Kora `var` Declaration**\n\nDeclares a mutable variable binding.",
        "match" => {
            "**Kora `match` Expression**\n\nExhaustive pattern matching construct. Evaluates variants and ensures 100% case coverage at compile time."
        }
        "null" => {
            "**Kora `null`**\n\nSingle-null design: represents absent optional values (`Option<T>`). Note: `undefined` is strictly forbidden in Kora (`E2005`)."
        }
        "@entity" => {
            "**Kora `@entity` Decorator**\n\nMarks a model as a persistent database entity managed by Kora ORM."
        }
        "@table" => {
            "**Kora `@table(\"name\")` Decorator**\n\nMaps a model to a specific SQL table name in PostgreSQL, MySQL, or SQLite."
        }
        "@primary" => {
            "**Kora `@primary` Decorator**\n\nMarks a model field as the primary key of the database table."
        }
        "@autoIncrement" => {
            "**Kora `@autoIncrement` Decorator**\n\nConfigures integer primary key to automatically sequence upon insert."
        }
        "@unique" => {
            "**Kora `@unique` Decorator**\n\nEnforces a unique database constraint on this column."
        }
        "@index" => {
            "**Kora `@index` Decorator**\n\nGenerates a database B-Tree index for accelerated query lookups."
        }
        "@required" => {
            "**Kora `@required` Decorator**\n\nEnforces non-empty, non-null validation on incoming API requests."
        }
        "@email" => {
            "**Kora `@email` Decorator**\n\nValidates that the string matches standard RFC email formatting."
        }
        "@min" => {
            "**Kora `@min(value)` Decorator**\n\nValidates minimum numeric value or minimum string length constraint."
        }
        "@max" => {
            "**Kora `@max(value)` Decorator**\n\nValidates maximum numeric value or maximum string length constraint."
        }
        _ => {
            // Check if it's a declared symbol in this source file
            let search_patterns = [
                format!("model {}", word),
                format!("class {}", word),
                format!("interface {}", word),
                format!("enum {}", word),
                format!("fn {}(", word),
            ];
            for (idx, line) in src.lines().enumerate() {
                for pat in &search_patterns {
                    if line.contains(pat) {
                        return Some(format!(
                            "**Kora Symbol Definition**\n\n```kora\n{}\n```\nDefined on line {}",
                            line.trim(),
                            idx + 1
                        ));
                    }
                }
            }
            return None;
        }
    };

    Some(doc.to_string())
}

/// Resolves Go To Definition for symbol under cursor.
fn find_definition(
    current_uri: &str,
    src: &str,
    line_idx: usize,
    char_idx: usize,
) -> Option<serde_json::Value> {
    let word = extract_word_at_pos(src, line_idx, char_idx)?;

    // 1. Check local file declarations
    let search_patterns = [
        format!("model {} ", word),
        format!("model {}{{", word),
        format!("class {} ", word),
        format!("class {}{{", word),
        format!("interface {} ", word),
        format!("interface {}{{", word),
        format!("enum {} ", word),
        format!("enum {}{{", word),
        format!("fn {}(", word),
        format!("let {} ", word),
        format!("var {} ", word),
        format!("const {} ", word),
    ];

    for (l_idx, line) in src.lines().enumerate() {
        for pat in &search_patterns {
            if let Some(col_idx) = line.find(pat) {
                return Some(serde_json::json!({
                    "uri": current_uri,
                    "range": {
                        "start": { "line": l_idx, "character": col_idx },
                        "end": { "line": l_idx, "character": col_idx + pat.len() }
                    }
                }));
            }
        }
    }

    // 2. Check if imported from another module: import { ... word ... } from "rel/path"
    let current_path = current_uri.strip_prefix("file://")?;
    let parent_dir = Path::new(current_path).parent()?;

    for line in src.lines() {
        if line.contains("import") && line.contains(&word) {
            if let Some(start_quote) = line.find('"') {
                if let Some(end_quote) = line[start_quote + 1..].find('"') {
                    let rel_path = &line[start_quote + 1..start_quote + 1 + end_quote];
                    let candidate_paths = [
                        parent_dir.join(format!("{}.kora", rel_path)),
                        parent_dir.join(format!("{}.kr", rel_path)),
                        parent_dir.join(rel_path).join("index.kora"),
                        parent_dir.join(rel_path).join("index.kr"),
                        parent_dir.join(rel_path),
                    ];

                    for cand in &candidate_paths {
                        if cand.is_file() {
                            if let Ok(imported_content) = fs::read_to_string(cand) {
                                for (target_line, imp_line) in imported_content.lines().enumerate()
                                {
                                    for pat in &search_patterns {
                                        if let Some(col) = imp_line.find(pat) {
                                            return Some(serde_json::json!({
                                                "uri": format!("file://{}", cand.display()),
                                                "range": {
                                                    "start": { "line": target_line, "character": col },
                                                    "end": { "line": target_line, "character": col + pat.len() }
                                                }
                                            }));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Finds all references to word under cursor in current file.
fn find_references(
    current_uri: &str,
    src: &str,
    line_idx: usize,
    char_idx: usize,
) -> Vec<serde_json::Value> {
    let mut refs = Vec::new();
    let Some(word) = extract_word_at_pos(src, line_idx, char_idx) else {
        return refs;
    };

    for (l_idx, line) in src.lines().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(&word) {
            let actual_col = start + pos;
            refs.push(serde_json::json!({
                "uri": current_uri,
                "range": {
                    "start": { "line": l_idx, "character": actual_col },
                    "end": { "line": l_idx, "character": actual_col + word.len() }
                }
            }));
            start = actual_col + word.len();
        }
    }

    refs
}

/// Appends top-level symbols declared in parsed module to completions list.
fn append_module_symbols_to_completions(module: &Module, completions: &mut Vec<serde_json::Value>) {
    for item in &module.items {
        match item {
            Item::Model(m) => {
                completions.push(serde_json::json!({
                    "label": m.name.name,
                    "kind": 23, // Struct
                    "detail": "Local Model"
                }));
            }
            Item::Class(c) => {
                completions.push(serde_json::json!({
                    "label": c.name.name,
                    "kind": 5, // Class
                    "detail": "Local Class"
                }));
            }
            Item::Interface(i) => {
                completions.push(serde_json::json!({
                    "label": i.name.name,
                    "kind": 11, // Interface
                    "detail": "Local Interface"
                }));
            }
            Item::Enum(e) => {
                completions.push(serde_json::json!({
                    "label": e.name.name,
                    "kind": 10, // Enum
                    "detail": "Local Enum"
                }));
            }
            Item::Fn(f) => {
                completions.push(serde_json::json!({
                    "label": f.sig.name.name,
                    "kind": 12, // Function
                    "detail": "Local Function"
                }));
            }
            _ => {}
        }
    }
}

/// Pre-populated static keywords, types, decorators, and snippets.
fn get_static_completions() -> Vec<serde_json::Value> {
    vec![
        // Declarations & Keywords
        serde_json::json!({ "label": "model", "kind": 23, "detail": "Pure immutable DTO / Entity", "insertText": "model ${1:Name} {\n    ${2:field}: ${3:String},\n}" }),
        serde_json::json!({ "label": "class", "kind": 5, "detail": "Service class with compile-time DI", "insertText": "class ${1:Name}Service implements ${2:Interface} {\n    db: Database,\n\n    init(db: Database) {\n        this.db = db\n    }\n\n    fn ${3:execute}() -> ${4:Unit} {\n        ${0}\n    }\n}" }),
        serde_json::json!({ "label": "interface", "kind": 11, "detail": "Behavioral contract interface", "insertText": "interface ${1:Name} {\n    fn ${2:method}(${3:param}: ${4:Type}) -> ${5:ReturnType}\n}" }),
        serde_json::json!({ "label": "enum", "kind": 10, "detail": "Type-safe enumerated variants", "insertText": "enum ${1:Name} {\n    ${2:VariantA},\n    ${3:VariantB},\n}" }),
        serde_json::json!({ "label": "actor", "kind": 5, "detail": "Isolated stateful actor", "insertText": "class ${1:Name}Actor {\n    var state: ${2:Int}\n\n    init() {\n        this.state = 0\n    }\n\n    fn handle(msg: ${3:String}) -> ${4:Int} {\n        ${0}\n    }\n}" }),
        serde_json::json!({ "label": "fn", "kind": 3, "detail": "Function declaration", "insertText": "fn ${1:name}(${2:params}) -> ${3:Unit} {\n    ${0}\n}" }),
        serde_json::json!({ "label": "let", "kind": 14, "detail": "Immutable variable binding", "insertText": "let ${1:name} = ${2:value}" }),
        serde_json::json!({ "label": "var", "kind": 14, "detail": "Mutable variable binding", "insertText": "var ${1:name} = ${2:value}" }),
        serde_json::json!({ "label": "const", "kind": 14, "detail": "Compile-time constant", "insertText": "const ${1:NAME} = ${2:value}" }),
        serde_json::json!({ "label": "for", "kind": 14, "detail": "For-in loop", "insertText": "for ${1:item} in ${2:collection} {\n    ${0}\n}" }),
        serde_json::json!({ "label": "try", "kind": 14, "detail": "Try-catch error resilience block", "insertText": "try {\n    ${1}\n} catch (${2:err}) {\n    ${0}\n}" }),
        serde_json::json!({ "label": "match", "kind": 14, "detail": "Pattern matching expression", "insertText": "match ${1:expr} {\n    Some(${2:val}) => ${3:val},\n    None => ${4:fallback},\n}" }),
        serde_json::json!({ "label": "spawn", "kind": 3, "detail": "Spawn a concurrent task", "insertText": "spawn(async () => {\n    ${0}\n})" }),
        serde_json::json!({ "label": "channel", "kind": 3, "detail": "Create non-blocking MPSC channel", "insertText": "channel<${1:Type}>(${2:100})" }),
        serde_json::json!({ "label": "broadcast", "kind": 3, "detail": "Create pub/sub broadcast channel", "insertText": "broadcast<${1:Type}>(${2:100})" }),
        // Built-in Types
        serde_json::json!({ "label": "Int", "kind": 25, "detail": "64-bit signed integer" }),
        serde_json::json!({ "label": "Float", "kind": 25, "detail": "64-bit IEEE floating-point number" }),
        serde_json::json!({ "label": "String", "kind": 25, "detail": "UTF-8 heap string" }),
        serde_json::json!({ "label": "Bool", "kind": 25, "detail": "Boolean (true | false)" }),
        serde_json::json!({ "label": "Unit", "kind": 25, "detail": "Empty unit type ()" }),
        serde_json::json!({ "label": "Option", "kind": 25, "detail": "Nullable wrapper: Some(T) | None" }),
        serde_json::json!({ "label": "Result", "kind": 25, "detail": "Result wrapper: Ok(T) | Err(E)" }),
        serde_json::json!({ "label": "List", "kind": 25, "detail": "Dynamic array list: List<T>" }),
        serde_json::json!({ "label": "null", "kind": 21, "detail": "Absence of value (Option::None)" }),
        // Decorators
        serde_json::json!({ "label": "@entity", "kind": 5, "detail": "Database persistent entity" }),
        serde_json::json!({ "label": "@table", "kind": 5, "detail": "Map model to SQL table name", "insertText": "@table(\"${1:table_name}\")" }),
        serde_json::json!({ "label": "@primary", "kind": 5, "detail": "Primary key constraint" }),
        serde_json::json!({ "label": "@autoIncrement", "kind": 5, "detail": "Auto-incrementing integer sequence" }),
        serde_json::json!({ "label": "@unique", "kind": 5, "detail": "Enforce unique column constraint" }),
        serde_json::json!({ "label": "@index", "kind": 5, "detail": "Create database index" }),
        serde_json::json!({ "label": "@required", "kind": 5, "detail": "Enforce non-empty validation" }),
        serde_json::json!({ "label": "@email", "kind": 5, "detail": "RFC email format validation" }),
        serde_json::json!({ "label": "@min", "kind": 5, "detail": "Minimum value or length constraint", "insertText": "@min(${1:1})" }),
        serde_json::json!({ "label": "@max", "kind": 5, "detail": "Maximum value or length constraint", "insertText": "@max(${1:100})" }),
        serde_json::json!({ "label": "@default", "kind": 5, "detail": "Default database column value", "insertText": "@default(\"${1:val}\")" }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_initialize_and_completion_requests() {
        let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let init_res = LanguageServer::handle_message(init_req).expect("must handle initialize");
        assert!(init_res.contains("hoverProvider"));
        assert!(init_res.contains("completionProvider"));
        assert!(init_res.contains("documentFormattingProvider"));
        assert!(init_res.contains("definitionProvider"));
        assert!(init_res.contains("documentSymbolProvider"));

        let comp_req = r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{}}"#;
        let comp_res = LanguageServer::handle_message(comp_req).expect("must handle completion");
        assert!(comp_res.contains("model"));
        assert!(comp_res.contains("class"));
        assert!(comp_res.contains("actor"));
        assert!(comp_res.contains("try"));
    }

    #[test]
    fn handles_document_symbols_and_hover() {
        let sample_kora = r#"
model User {
    id: Int,
    name: String,
}

class UserService {
    fn get_user() -> Int {
        return 42
    }
}
"#;
        with_cache(|c| {
            c.insert("file:///sample.kora".into(), sample_kora.into());
        });

        let sym_req = r#"{"jsonrpc":"2.0","id":3,"method":"textDocument/documentSymbol","params":{"textDocument":{"uri":"file:///sample.kora"}}}"#;
        let sym_res = LanguageServer::handle_message(sym_req).expect("must handle symbols");
        assert!(sym_res.contains("User"));
        assert!(sym_res.contains("UserService"));
        assert!(sym_res.contains("get_user"));

        let hover_req = r#"{"jsonrpc":"2.0","id":4,"method":"textDocument/hover","params":{"textDocument":{"uri":"file:///sample.kora"},"position":{"line":1,"character":2}}}"#;
        let hover_res = LanguageServer::handle_message(hover_req).expect("must handle hover");
        assert!(hover_res.contains("model"));
    }
}
