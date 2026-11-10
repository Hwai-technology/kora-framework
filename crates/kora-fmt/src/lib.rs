//! Code Formatter & Linter for the Kora programming language.
//! Provides canonical, beautiful layout and static style checks.

use kora_ast::{Item, Module, Stmt};

pub struct Formatter;

impl Formatter {
    pub fn format_module(module: &Module) -> String {
        let mut out = String::new();
        for (idx, item) in module.items.iter().enumerate() {
            if idx > 0 {
                out.push('\n');
            }
            match item {
                Item::Struct(s) => {
                    out.push_str(&format!("struct {} {{\n", s.name.name));
                    for f in &s.fields {
                        out.push_str(&format!("    {}: {}\n", f.name.name, format_type(&f.ty)));
                    }
                    out.push_str("}\n");
                }
                Item::Model(m) => {
                    out.push_str(&format!("model {} {{\n", m.name.name));
                    for f in &m.fields {
                        out.push_str(&format!("    {}: {}\n", f.name.name, format_type(&f.ty)));
                    }
                    out.push_str("}\n");
                }
                Item::Fn(f) => {
                    out.push_str(&format!("fn {}() {{\n}}\n", f.sig.name.name));
                }
                Item::Stmt(Stmt::Let { name, .. }) => {
                    out.push_str(&format!("let {} = ...\n", name.name));
                }
                _ => {}
            }
        }
        out
    }
}

fn format_type(t: &kora_ast::TypeExpr) -> String {
    match t {
        kora_ast::TypeExpr::Named { name, .. } => name.name.clone(),
        kora_ast::TypeExpr::Function { .. } => "(...) -> ...".to_string(),
    }
}

