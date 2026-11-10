//! Source file registry, byte spans, and 1-based line/column lookup for the Kora compiler.

/// Compact identifier referencing a [`SourceFile`] inside a [`SourceMap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FileId(pub u32);

/// Half-open byte range `[start, end)` within a specific [`SourceFile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub file_id: FileId,
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub const fn new(file_id: FileId, start: u32, end: u32) -> Self {
        Self {
            file_id,
            start,
            end,
        }
    }

    #[inline]
    pub const fn dummy() -> Self {
        Self {
            file_id: FileId(u32::MAX),
            start: 0,
            end: 0,
        }
    }

    #[inline]
    pub const fn is_dummy(self) -> bool {
        self.file_id.0 == u32::MAX
    }

    /// Combines two spans into a single span covering both.
    #[inline]
    pub fn merge(self, other: Self) -> Self {
        if self.is_dummy() {
            return other;
        }
        if other.is_dummy() || self.file_id != other.file_id {
            return self;
        }
        Self {
            file_id: self.file_id,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// 1-based line and UTF-8 character column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
}

/// A loaded source file with precomputed line-start byte offsets.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id: FileId,
    pub name: String,
    pub src: String,
    line_starts: Vec<u32>,
}

impl SourceFile {
    pub fn new(id: FileId, name: impl Into<String>, src: impl Into<String>) -> Self {
        let name = name.into();
        let src = src.into();
        let mut line_starts = vec![0];
        for (idx, byte) in src.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push((idx + 1) as u32);
            }
        }
        Self {
            id,
            name,
            src,
            line_starts,
        }
    }

    /// Resolves a byte offset to a 1-based [`LineCol`].
    pub fn line_col(&self, byte_pos: u32) -> LineCol {
        let clamped = (byte_pos as usize).min(self.src.len());
        let line_idx = match self.line_starts.binary_search(&(clamped as u32)) {
            Ok(exact) => exact,
            Err(insert_pos) => insert_pos.saturating_sub(1),
        };
        let line_start = self.line_starts[line_idx] as usize;
        let prefix = self.src.get(line_start..clamped).unwrap_or("");
        let col = prefix.chars().count() + 1;
        LineCol {
            line: line_idx + 1,
            col,
        }
    }

    /// Returns the text of a 1-based line without trailing `\r` or `\n`.
    pub fn line_text(&self, line_1based: usize) -> Option<&str> {
        if line_1based == 0 || line_1based > self.line_starts.len() {
            return None;
        }
        let idx = line_1based - 1;
        let start = self.line_starts[idx] as usize;
        let end = self
            .line_starts
            .get(idx + 1)
            .map(|&pos| pos as usize)
            .unwrap_or(self.src.len());
        let raw = self.src.get(start..end)?;
        Some(raw.trim_end_matches(['\r', '\n']))
    }

    /// Safely extracts the source slice covered by `span`.
    pub fn slice(&self, span: Span) -> &str {
        let start = (span.start as usize).min(self.src.len());
        let end = (span.end as usize).min(self.src.len()).max(start);
        self.src.get(start..end).unwrap_or("")
    }
}

/// Registry of all `.kora` source files compiled in a session.
#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add_file(&mut self, name: impl Into<String>, src: impl Into<String>) -> FileId {
        let id = FileId(self.files.len() as u32);
        self.files.push(SourceFile::new(id, name, src));
        id
    }

    pub fn get_file(&self, id: FileId) -> Option<&SourceFile> {
        self.files.get(id.0 as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_lines_and_columns_accurately() {
        let mut sm = SourceMap::new();
        let id = sm.add_file("test.kora", "let x = 10\nlet name = \"Ahmet\"\n");
        let file = sm.get_file(id).expect("file exists");

        assert_eq!(file.line_col(0), LineCol { line: 1, col: 1 });
        assert_eq!(file.line_col(4), LineCol { line: 1, col: 5 });
        assert_eq!(file.line_col(11), LineCol { line: 2, col: 1 });
        assert_eq!(file.line_text(1), Some("let x = 10"));
        assert_eq!(file.line_text(2), Some("let name = \"Ahmet\""));
    }

    #[test]
    fn merges_spans_safely() {
        let f = FileId(0);
        let a = Span::new(f, 4, 8);
        let b = Span::new(f, 12, 16);
        assert_eq!(a.merge(b), Span::new(f, 4, 16));
        assert_eq!(Span::dummy().merge(a), a);
    }
}
