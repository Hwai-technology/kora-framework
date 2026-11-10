//! Structured compiler diagnostics and source-snippet formatting for Kora.

use std::fmt::Write;

use kora_span::{SourceMap, Span};

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Hint,
}

impl Severity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Hint => "hint",
        }
    }
}

/// An annotated source span attached to a [`Diagnostic`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: Span,
    pub message: String,
    pub is_primary: bool,
}

/// A rich compiler diagnostic referencing `.kora` source locations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    pub labels: Vec<Label>,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            labels: Vec::new(),
            help: None,
        }
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: message.into(),
            labels: Vec::new(),
            help: None,
        }
    }

    pub fn with_primary_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            is_primary: true,
        });
        self
    }

    pub fn with_secondary_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            is_primary: false,
        });
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
}

/// Accumulator for diagnostics across Lexer, Parser, HIR, and Type Checker passes.
#[derive(Debug, Default, Clone)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn items(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn render_all(&self, source_map: &SourceMap, use_color: bool) -> String {
        let mut out = String::new();
        for (idx, diag) in self.diagnostics.iter().enumerate() {
            if idx > 0 {
                out.push('\n');
            }
            render_diagnostic(&mut out, diag, source_map, use_color);
        }
        out
    }
}

fn render_diagnostic(out: &mut String, diag: &Diagnostic, source_map: &SourceMap, use_color: bool) {
    let (sev_start, reset, blue) = if use_color {
        let sev_col = match diag.severity {
            Severity::Error => "\x1b[1;31m",
            Severity::Warning => "\x1b[1;33m",
            Severity::Hint => "\x1b[1;36m",
        };
        (sev_col, "\x1b[0m", "\x1b[1;34m")
    } else {
        ("", "", "")
    };

    let _ = writeln!(
        out,
        "{}{}[{}]{}: {}",
        sev_start,
        diag.severity.label(),
        diag.code,
        reset,
        diag.message
    );

    for label in &diag.labels {
        if label.span.is_dummy() {
            continue;
        }
        if let Some(file) = source_map.get_file(label.span.file_id) {
            let start_lc = file.line_col(label.span.start);
            let end_lc = file.line_col(label.span.end);
            let width = start_lc.line.to_string().len();

            let _ = writeln!(
                out,
                "{:>width$} {}-->{} {}:{}:{}",
                "",
                blue,
                reset,
                file.name,
                start_lc.line,
                start_lc.col,
                width = width
            );

            if let Some(line_str) = file.line_text(start_lc.line) {
                let _ = writeln!(out, "{:>width$} {}|{}", "", blue, reset, width = width);
                let _ = writeln!(
                    out,
                    "{}{:>width$} |{} {}",
                    blue,
                    start_lc.line,
                    reset,
                    line_str,
                    width = width
                );

                let caret_char = if label.is_primary { '^' } else { '-' };
                let underline_len = if start_lc.line == end_lc.line && end_lc.col > start_lc.col {
                    end_lc.col - start_lc.col
                } else {
                    1
                };
                let pad = start_lc.col.saturating_sub(1);
                let marker: String = std::iter::repeat_n(caret_char, underline_len).collect();

                if label.message.is_empty() {
                    let _ = writeln!(
                        out,
                        "{:>width$} {}|{} {:>pad$}{}{}{}",
                        "",
                        blue,
                        reset,
                        "",
                        sev_start,
                        marker,
                        reset,
                        width = width,
                        pad = pad
                    );
                } else {
                    let _ = writeln!(
                        out,
                        "{:>width$} {}|{} {:>pad$}{}{} {}{}",
                        "",
                        blue,
                        reset,
                        "",
                        sev_start,
                        marker,
                        label.message,
                        reset,
                        width = width,
                        pad = pad
                    );
                }
            }
        }
    }

    if let Some(help) = &diag.help {
        let _ = writeln!(out, "  {}= help:{} {}", blue, reset, help);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_diagnostic_with_source_snippet() {
        let mut sm = SourceMap::new();
        let fid = sm.add_file("demo.kora", "let age: Int = \"twenty\"\n");
        let mut bag = DiagnosticBag::new();
        bag.emit(
            Diagnostic::error("E4001", "type mismatch: expected `Int`, found `String`")
                .with_primary_label(kora_span::Span::new(fid, 15, 23), "expected `Int`")
                .with_help("convert the string to an integer or change the type annotation"),
        );

        let rendered = bag.render_all(&sm, false);
        assert!(rendered.contains("error[E4001]"));
        assert!(rendered.contains("demo.kora:1:16"));
        assert!(rendered.contains("^^^^^^^^ expected `Int`"));
        assert!(rendered.contains("= help:"));
    }
}

