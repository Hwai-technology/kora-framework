//! Library driver for the `kora` CLI and end-to-end compiler frontend sessions.

use std::fmt::Write;
use std::fs;
use std::path::Path;

use kora_ast::Module;
use kora_diagnostics::DiagnosticBag;
use kora_hir::{HirModule, lower_and_resolve};
use kora_lexer::{Token, tokenize};
use kora_parser::parse_module;
use kora_span::SourceMap;
use kora_ty::{TypeCheckOutput, check_module};

/// Result of running the full Phase 1 frontend pipeline on a `.kora` source file.
pub struct FrontendSession {
    pub source_map: SourceMap,
    pub ast: Module,
    pub hir: HirModule,
    pub types: TypeCheckOutput,
    pub diagnostics: DiagnosticBag,
}

impl FrontendSession {
    pub fn check_source(name: impl Into<String>, src: impl Into<String>) -> Self {
        let mut source_map = SourceMap::new();
        let file_id = source_map.add_file(name, src);
        let src_ref = source_map
            .get_file(file_id)
            .expect("source file registered")
            .src
            .clone();

        let mut diagnostics = DiagnosticBag::new();
        let ast = parse_module(file_id, &src_ref, &mut diagnostics);
        let hir = lower_and_resolve(ast.clone(), &mut diagnostics);
        let types = check_module(&hir, &mut diagnostics);

        Self {
            source_map,
            ast,
            hir,
            types,
            diagnostics,
        }
    }

    pub fn lex_only(
        name: impl Into<String>,
        src: impl Into<String>,
    ) -> (SourceMap, Vec<Token>, DiagnosticBag) {
        let mut source_map = SourceMap::new();
        let file_id = source_map.add_file(name, src);
        let src_ref = source_map
            .get_file(file_id)
            .expect("source file registered")
            .src
            .clone();
        let mut diagnostics = DiagnosticBag::new();
        let tokens = tokenize(file_id, &src_ref, &mut diagnostics);
        (source_map, tokens, diagnostics)
    }
}

/// Formats verified declarations and inferred variable types into a human-readable report.
pub fn format_type_report(types: &TypeCheckOutput) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "=== Kora Static Type & DI Verification Report ===");

    if !types.structs.is_empty() {
        let _ = writeln!(out, "\n[Structs]");
        let mut names: Vec<_> = types.structs.keys().collect();
        names.sort();
        for name in names {
            let _ = writeln!(out, "  struct {name}");
        }
    }

    if !types.models.is_empty() {
        let _ = writeln!(out, "\n[Models]");
        let mut names: Vec<_> = types.models.keys().collect();
        names.sort();
        for name in names {
            let fields = &types.models[name];
            let mut fnames: Vec<_> = fields.keys().collect();
            fnames.sort();
            let summary: Vec<String> = fnames
                .into_iter()
                .map(|f| format!("{f}: {}", fields[f]))
                .collect();
            let kind = if types.entity_models.contains(name) {
                let tbl = types.model_tables.get(name).map(|t| format!(" (table: \"{t}\")")).unwrap_or_default();
                format!("@entity model {name}{tbl}")
            } else {
                format!("model {name} [DTO]")
            };
            let _ = writeln!(out, "  {kind} {{ {} }}", summary.join(", "));
        }
    }

    if !types.interfaces.is_empty() {
        let _ = writeln!(out, "\n[Interfaces]");
        let mut names: Vec<_> = types.interfaces.keys().collect();
        names.sort();
        for name in names {
            let _ = writeln!(out, "  interface {name}");
        }
    }

    if !types.classes.is_empty() {
        let _ = writeln!(out, "\n[Service Classes (Compile-Time DI)]");
        let mut names: Vec<_> = types.classes.keys().collect();
        names.sort();
        for name in names {
            let cls = &types.classes[name];
            let impls = if cls.implements.is_empty() {
                String::new()
            } else {
                format!(" implements {}", cls.implements.join(", "))
            };
            let deps: Vec<String> = cls
                .init_params
                .iter()
                .map(|(p, t)| format!("{p}: {t}"))
                .collect();
            let _ = writeln!(out, "  class {name}{impls} | init({})", deps.join(", "));
        }
    }

    if !types.enums.is_empty() {
        let _ = writeln!(out, "\n[Enums]");
        let mut names: Vec<_> = types.enums.keys().collect();
        names.sort();
        for name in names {
            let variants: Vec<String> = types.enums[name]
                .iter()
                .map(|(v, p)| {
                    if p.is_empty() {
                        v.clone()
                    } else {
                        let pts: Vec<String> = p.iter().map(ToString::to_string).collect();
                        format!("{v}({})", pts.join(", "))
                    }
                })
                .collect();
            let _ = writeln!(out, "  enum {name} [{}]", variants.join(" | "));
        }
    }

    if !types.functions.is_empty() {
        let _ = writeln!(out, "\n[Functions]");
        let mut names: Vec<_> = types.functions.keys().collect();
        names.sort();
        for name in names {
            let f = &types.functions[name];
            let params: Vec<String> = f.params.iter().map(|(p, t)| format!("{p}: {t}")).collect();
            let _ = writeln!(out, "  fn {name}({}) -> {}", params.join(", "), f.ret);
        }
    }

    if !types.bindings.is_empty() {
        let _ = writeln!(out, "\n[Inferred Bindings]");
        for b in &types.bindings {
            let kw = if b.mutable { "var" } else { "let" };
            let _ = writeln!(out, "  ({}) {kw} {}: {}", b.scope, b.name, b.ty);
        }
    }

    out
}

/// Initializes a new Kora project directory with `kora.toml` and `src/main.kora`.
pub fn scaffold_project(dir: &Path) -> Result<(), String> {
    if dir.exists() {
        return Err(format!(
            "target directory `{}` already exists",
            dir.display()
        ));
    }

    let project_name = dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("kora-app");

    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| format!("failed to create directory: {e}"))?;

    let manifest = format!(
        r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2026"
entry = "src/main.kora"

[server]
host = "127.0.0.1"
port = 8080
"#
    );

    let main_kora = r#"model HealthStatus {
    service: String
    healthy: Bool
}

let app = run()

get("/health", () => {
    return HealthStatus {
        service: "kora-service",
        healthy: true,
    }
})
"#;

    fs::write(dir.join("kora.toml"), manifest)
        .map_err(|e| format!("failed to write `kora.toml`: {e}"))?;
    fs::write(src_dir.join("main.kora"), main_kora)
        .map_err(|e| format!("failed to write `src/main.kora`: {e}"))?;
    fs::write(dir.join(".gitignore"), "/.kora-build/\n/dist/\n")
        .map_err(|e| format!("failed to write `.gitignore`: {e}"))?;

    Ok(())
}
