//! Lexical analyzer for the Kora programming language.
//!
//! Tracks line breaks (`had_newline_before`) on every [`Token`] to enable clean,
//! ambiguity-free optional semicolons without cluttering expressions.

use kora_diagnostics::{Diagnostic, DiagnosticBag};
use kora_span::{FileId, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals & Identifiers
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),

    // Keywords
    Struct,
    Model,
    Interface,
    Class,
    Implements,
    Init,
    Enum,
    Fn,
    Async,
    Let,
    Var,
    Return,
    If,
    Else,
    While,
    Match,
    True,
    False,
    This,

    // Punctuation & Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Underscore,
    ThinArrow, // ->
    FatArrow,  // =>

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,    // =
    EqEq,  // ==
    Bang,  // !
    NotEq, // !=
    Lt,    // <
    LtEq,  // <=
    Gt,    // >
    GtEq,  // >=
    AndAnd,
    OrOr,

    Eof,
}

impl TokenKind {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Ident(_) => "identifier",
            Self::IntLit(_) => "integer literal",
            Self::FloatLit(_) => "float literal",
            Self::StringLit(_) => "string literal",
            Self::Struct => "`struct`",
            Self::Model => "`model`",
            Self::Interface => "`interface`",
            Self::Class => "`class`",
            Self::Implements => "`implements`",
            Self::Init => "`init`",
            Self::Enum => "`enum`",
            Self::Fn => "`fn`",
            Self::Async => "`async`",
            Self::Let => "`let`",
            Self::Var => "`var`",
            Self::Return => "`return`",
            Self::If => "`if`",
            Self::Else => "`else`",
            Self::While => "`while`",
            Self::Match => "`match`",
            Self::True => "`true`",
            Self::False => "`false`",
            Self::This => "`this`",
            Self::LParen => "`(`",
            Self::RParen => "`)`",
            Self::LBrace => "`{`",
            Self::RBrace => "`}`",
            Self::LBracket => "`[`",
            Self::RBracket => "`]`",
            Self::Colon => "`:`",
            Self::Semicolon => "`;`",
            Self::Comma => "`,`",
            Self::Dot => "`.`",
            Self::Underscore => "`_`",
            Self::ThinArrow => "`->`",
            Self::FatArrow => "`=>`",
            Self::Plus => "`+`",
            Self::Minus => "`-`",
            Self::Star => "`*`",
            Self::Slash => "`/`",
            Self::Percent => "`%`",
            Self::Eq => "`=`",
            Self::EqEq => "`==`",
            Self::Bang => "`!`",
            Self::NotEq => "`!=`",
            Self::Lt => "`<`",
            Self::LtEq => "`<=`",
            Self::Gt => "`>`",
            Self::GtEq => "`>=`",
            Self::AndAnd => "`&&`",
            Self::OrOr => "`||`",
            Self::Eof => "end of file",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    /// True if at least one `\n` separated the previous token from this token.
    pub had_newline_before: bool,
}

/// Tokenizes `src` into a stream ending with [`TokenKind::Eof`].
pub fn tokenize(file_id: FileId, src: &str, diagnostics: &mut DiagnosticBag) -> Vec<Token> {
    Lexer::new(file_id, src, diagnostics).tokenize_all()
}

struct Lexer<'a> {
    file_id: FileId,
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> Lexer<'a> {
    fn new(file_id: FileId, src: &'a str, diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            file_id,
            src,
            bytes: src.as_bytes(),
            pos: 0,
            diagnostics,
        }
    }

    fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let had_newline_before = self.skip_whitespace_and_comments();
            let start = self.pos;
            let Some(ch) = self.current_char() else {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: self.make_span(start, start),
                    had_newline_before,
                });
                break;
            };

            if let Some(kind) = self.next_token_kind(ch, start) {
                let end = self.pos;
                tokens.push(Token {
                    kind,
                    span: self.make_span(start, end),
                    had_newline_before,
                });
            }
        }
        tokens
    }

    fn skip_whitespace_and_comments(&mut self) -> bool {
        let mut saw_newline = false;
        loop {
            while let Some(b) = self.bytes.get(self.pos).copied() {
                match b {
                    b'\n' => {
                        saw_newline = true;
                        self.pos += 1;
                    }
                    b' ' | b'\t' | b'\r' => {
                        self.pos += 1;
                    }
                    _ => break,
                }
            }

            if self.starts_with("//") {
                self.pos += 2;
                while let Some(b) = self.bytes.get(self.pos).copied() {
                    if b == b'\n' {
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }

            if self.starts_with("/*") {
                let comment_start = self.pos;
                self.pos += 2;
                let mut closed = false;
                while self.pos < self.bytes.len() {
                    if self.starts_with("*/") {
                        self.pos += 2;
                        closed = true;
                        break;
                    }
                    if self.bytes[self.pos] == b'\n' {
                        saw_newline = true;
                    }
                    self.advance_char();
                }
                if !closed {
                    self.diagnostics.emit(
                        Diagnostic::error("E1002", "unterminated block comment `/* ... */`")
                            .with_primary_label(
                                self.make_span(comment_start, self.pos),
                                "block comment starts here and never closes",
                            )
                            .with_help("add `*/` to close the block comment"),
                    );
                }
                continue;
            }

            break;
        }
        saw_newline
    }

    fn next_token_kind(&mut self, ch: char, start: usize) -> Option<TokenKind> {
        if is_ident_start(ch) {
            return Some(self.lex_ident_or_keyword(start));
        }

        if ch.is_ascii_digit() {
            return Some(self.lex_number(start));
        }

        if ch == '"' {
            return Some(self.lex_string(start));
        }

        self.advance_char();
        match ch {
            '(' => Some(TokenKind::LParen),
            ')' => Some(TokenKind::RParen),
            '{' => Some(TokenKind::LBrace),
            '}' => Some(TokenKind::RBrace),
            '[' => Some(TokenKind::LBracket),
            ']' => Some(TokenKind::RBracket),
            ':' => Some(TokenKind::Colon),
            ';' => Some(TokenKind::Semicolon),
            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Dot),
            '+' => Some(TokenKind::Plus),
            '-' => {
                if self.consume_byte(b'>') {
                    Some(TokenKind::ThinArrow)
                } else {
                    Some(TokenKind::Minus)
                }
            }
            '*' => Some(TokenKind::Star),
            '/' => Some(TokenKind::Slash),
            '%' => Some(TokenKind::Percent),
            '=' => {
                if self.consume_byte(b'=') {
                    Some(TokenKind::EqEq)
                } else if self.consume_byte(b'>') {
                    Some(TokenKind::FatArrow)
                } else {
                    Some(TokenKind::Eq)
                }
            }
            '!' => {
                if self.consume_byte(b'=') {
                    Some(TokenKind::NotEq)
                } else {
                    Some(TokenKind::Bang)
                }
            }
            '<' => {
                if self.consume_byte(b'=') {
                    Some(TokenKind::LtEq)
                } else {
                    Some(TokenKind::Lt)
                }
            }
            '>' => {
                if self.consume_byte(b'=') {
                    Some(TokenKind::GtEq)
                } else {
                    Some(TokenKind::Gt)
                }
            }
            '&' => {
                if self.consume_byte(b'&') {
                    Some(TokenKind::AndAnd)
                } else {
                    self.diagnostics.emit(
                        Diagnostic::error("E1001", "unexpected single `&`").with_primary_label(
                            self.make_span(start, self.pos),
                            "use `&&` for logical AND",
                        ),
                    );
                    None
                }
            }
            '|' => {
                if self.consume_byte(b'|') {
                    Some(TokenKind::OrOr)
                } else {
                    self.diagnostics.emit(
                        Diagnostic::error("E1001", "unexpected single `|`").with_primary_label(
                            self.make_span(start, self.pos),
                            "use `||` for logical OR",
                        ),
                    );
                    None
                }
            }
            other => {
                self.diagnostics.emit(
                    Diagnostic::error("E1001", format!("unexpected character `{other}`"))
                        .with_primary_label(
                            self.make_span(start, self.pos),
                            "unrecognized character in Kora source",
                        ),
                );
                None
            }
        }
    }

    fn lex_ident_or_keyword(&mut self, start: usize) -> TokenKind {
        while let Some(c) = self.current_char() {
            if is_ident_continue(c) {
                self.advance_char();
            } else {
                break;
            }
        }
        let text = &self.src[start..self.pos];
        match text {
            "_" => TokenKind::Underscore,
            "struct" => TokenKind::Struct,
            "model" => TokenKind::Model,
            "interface" => TokenKind::Interface,
            "class" => TokenKind::Class,
            "implements" => TokenKind::Implements,
            "init" => TokenKind::Init,
            "enum" => TokenKind::Enum,
            "fn" => TokenKind::Fn,
            "async" => TokenKind::Async,
            "let" => TokenKind::Let,
            "var" => TokenKind::Var,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "match" => TokenKind::Match,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "this" => TokenKind::This,
            _ => TokenKind::Ident(text.to_string()),
        }
    }

    fn lex_number(&mut self, start: usize) -> TokenKind {
        while let Some(b) = self.bytes.get(self.pos).copied() {
            if b.is_ascii_digit() || b == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let is_float = self.bytes.get(self.pos) == Some(&b'.')
            && self
                .bytes
                .get(self.pos + 1)
                .is_some_and(|next| next.is_ascii_digit());

        if is_float {
            self.pos += 1; // consume '.'
            while let Some(b) = self.bytes.get(self.pos).copied() {
                if b.is_ascii_digit() || b == b'_' {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            let cleaned: String = self.src[start..self.pos]
                .chars()
                .filter(|&c| c != '_')
                .collect();
            match cleaned.parse::<f64>() {
                Ok(v) => TokenKind::FloatLit(v),
                Err(_) => {
                    self.diagnostics.emit(
                        Diagnostic::error("E1003", "invalid floating-point literal")
                            .with_primary_label(self.make_span(start, self.pos), "invalid float"),
                    );
                    TokenKind::FloatLit(0.0)
                }
            }
        } else {
            let cleaned: String = self.src[start..self.pos]
                .chars()
                .filter(|&c| c != '_')
                .collect();
            match cleaned.parse::<i64>() {
                Ok(v) => TokenKind::IntLit(v),
                Err(_) => {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E1003",
                            "integer literal out of 64-bit signed integer range",
                        )
                        .with_primary_label(self.make_span(start, self.pos), "overflows `Int`"),
                    );
                    TokenKind::IntLit(0)
                }
            }
        }
    }

    fn lex_string(&mut self, start: usize) -> TokenKind {
        self.pos += 1; // opening '"'
        let mut buf = String::new();
        let mut closed = false;

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.pos += 1;
                closed = true;
                break;
            }
            if ch == '\n' {
                break;
            }
            if ch == '\\' {
                let esc_start = self.pos;
                self.pos += 1;
                let Some(esc) = self.current_char() else {
                    break;
                };
                self.advance_char();
                match esc {
                    '"' => buf.push('"'),
                    '\\' => buf.push('\\'),
                    'n' => buf.push('\n'),
                    'r' => buf.push('\r'),
                    't' => buf.push('\t'),
                    other => {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E1004",
                                format!("unknown escape sequence `\\{other}`"),
                            )
                            .with_primary_label(
                                self.make_span(esc_start, self.pos),
                                "unsupported string escape",
                            )
                            .with_help("supported escapes are `\\\"`, `\\\\`, `\\n`, `\\r`, `\\t`"),
                        );
                        buf.push(other);
                    }
                }
            } else {
                buf.push(ch);
                self.advance_char();
            }
        }

        if !closed {
            self.diagnostics.emit(
                Diagnostic::error("E1005", "unterminated string literal")
                    .with_primary_label(
                        self.make_span(start, self.pos),
                        "missing closing `\"` before end of line",
                    )
                    .with_help("close the string literal with `\"`"),
            );
        }

        TokenKind::StringLit(buf)
    }

    fn current_char(&self) -> Option<char> {
        self.src.get(self.pos..)?.chars().next()
    }

    fn advance_char(&mut self) {
        if let Some(ch) = self.current_char() {
            self.pos += ch.len_utf8();
        }
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.bytes.get(self.pos) == Some(&expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn starts_with(&self, pat: &str) -> bool {
        self.src.get(self.pos..).is_some_and(|s| s.starts_with(pat))
    }

    fn make_span(&self, start: usize, end: usize) -> Span {
        Span::new(self.file_id, start as u32, end as u32)
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_declarations_arrows_and_newlines() {
        let mut bag = DiagnosticBag::new();
        let src = "let name = \"Ahmet\"\nfn add(a: Int, b: Int) -> Int {\n  return a + b\n}\n";
        let tokens = tokenize(FileId(0), src, &mut bag);
        assert!(!bag.has_errors());
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert!(!tokens[0].had_newline_before);
        assert_eq!(tokens[4].kind, TokenKind::Fn);
        assert!(tokens[4].had_newline_before);
        assert!(tokens.iter().any(|t| t.kind == TokenKind::ThinArrow));
        assert_eq!(tokens.last().map(|t| &t.kind), Some(&TokenKind::Eof));
    }

    #[test]
    fn reports_unterminated_string_diagnostic() {
        let mut bag = DiagnosticBag::new();
        let _ = tokenize(FileId(0), "let broken = \"hello\n", &mut bag);
        assert!(bag.has_errors());
        assert_eq!(bag.items()[0].code, "E1005");
    }
}
