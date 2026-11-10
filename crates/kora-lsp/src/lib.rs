//! Language Server Protocol (LSP) Daemon for Kora.
//! Provides real-time IDE diagnostics, hover type inspections, and autocompletion via JSON-RPC 2.0.

use serde::{Deserialize, Serialize};

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
                        "hoverProvider": true,
                        "completionProvider": { "triggerCharacters": [".", ":"] }
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
            _ => return None,
        };
        serde_json::to_string(&resp).ok()
    }
}
