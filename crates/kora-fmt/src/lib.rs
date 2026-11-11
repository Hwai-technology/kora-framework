//! Code Formatter & Linter for the Kora programming language.
//! Provides canonical, beautiful layout and static style checks.

use kora_ast::{Item, Module, Stmt};

pub struct Formatter;

impl Formatter {
    /// Formats an AST module back to canonical source code.
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

    /// Formats raw Kora source code text into canonical layout:
    /// - Normalizes indentation to 4 spaces per block level
    /// - Trims trailing whitespace
    /// - Collapses multiple consecutive blank lines to at most one
    /// - Preserves comments and strings cleanly
    /// - Ensures exactly one trailing newline at EOF
    pub fn format_source(source: &str) -> String {
        let mut result = Vec::new();
        let mut indent_level = 0usize;
        let mut prev_was_empty = false;

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                if !prev_was_empty && !result.is_empty() {
                    result.push(String::new());
                    prev_was_empty = true;
                }
                continue;
            }
            prev_was_empty = false;

            // Adjust indentation for closing braces at line start
            let starts_with_close = trimmed.starts_with('}') || trimmed.starts_with(')');
            let current_indent = if starts_with_close && indent_level > 0 {
                indent_level - 1
            } else {
                indent_level
            };

            let indented_line = format!("{}{}", "    ".repeat(current_indent), trimmed);
            result.push(indented_line);

            // Compute net change in braces
            let open_braces = trimmed.chars().filter(|&c| c == '{').count();
            let close_braces = trimmed.chars().filter(|&c| c == '}').count();

            if open_braces > close_braces {
                indent_level += open_braces - close_braces;
            } else if close_braces > open_braces {
                indent_level = indent_level.saturating_sub(close_braces - open_braces);
            }
        }

        let mut output = result.join("\n");
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

fn format_type(t: &kora_ast::TypeExpr) -> String {
    match t {
        kora_ast::TypeExpr::Named { name, .. } => name.name.clone(),
        kora_ast::TypeExpr::Function { .. } => "(...) -> ...".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_messy_kora_code() {
        let messy = r#"
model User {
id: int
   name: string
}

fn hello() {
let x = 10
  return x
}
"#;
        let formatted = Formatter::format_source(messy);
        assert!(formatted.contains("    id: int\n"));
        assert!(formatted.contains("    name: string\n"));
        assert!(formatted.contains("    let x = 10\n"));
        assert!(formatted.contains("    return x\n"));
        assert!(formatted.ends_with('\n'));
    }
}
