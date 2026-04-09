use serde::Serialize;

/// Byte-offset span into source text. Start is inclusive, end is exclusive.
///
/// All positions in the Mesh compiler are tracked as byte offsets into the
/// original source string. Line/column information is computed on demand
/// via [`LineIndex`] when needed for error reporting or diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    /// Create a new span from byte offsets.
    pub fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end, "span start ({start}) must be <= end ({end})");
        Self { start, end }
    }

    /// Length of the span in bytes.
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Whether the span is empty (zero-length).
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Merge two spans into one that covers both.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// Pre-computed index of line start positions for on-demand line/column lookup.
///
/// Constructed once per source file, then used to convert byte offsets to
/// human-readable (line, column) pairs via binary search.
#[derive(Debug)]
pub struct LineIndex {
    /// Byte offset of the start of each line. The first entry is always 0.
    line_starts: Vec<u32>,
}

impl LineIndex {
    /// Build a line index by scanning the source text for newline characters.
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0u32];
        for (i, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }
        Self { line_starts }
    }

    /// Convert a byte offset to a 1-based (line, column) pair.
    ///
    /// Uses binary search on the pre-computed line start positions.
    /// Column is measured in bytes from the start of the line (1-based).
    pub fn line_col(&self, offset: u32) -> (u32, u32) {
        // Binary search for the line containing this offset.
        // partition_point returns the index of the first line_start > offset,
        // so the line index is one less than that.
        let line_idx = self.line_starts.partition_point(|&start| start <= offset);
        let line_idx = line_idx.saturating_sub(1);
        let line = (line_idx as u32) + 1; // 1-based
        let col = offset - self.line_starts[line_idx] + 1; // 1-based
        (line, col)
    }

    /// Return the number of lines in the source.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_new_and_len() {
        let span = Span::new(5, 10);
        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
        assert_eq!(span.len(), 5);
        assert!(!span.is_empty());
    }

    #[test]
    fn span_empty() {
        let span = Span::new(3, 3);
        assert_eq!(span.len(), 0);
        assert!(span.is_empty());
    }

    #[test]
    fn span_merge() {
        let a = Span::new(5, 10);
        let b = Span::new(8, 15);
        let merged = a.merge(b);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 15);
    }

    #[test]
    fn line_index_single_line() {
        let idx = LineIndex::new("hello");
        assert_eq!(idx.line_col(0), (1, 1));
        assert_eq!(idx.line_col(4), (1, 5));
    }

    #[test]
    fn line_index_multiple_lines() {
        let src = "hello\nworld\nfoo";
        let idx = LineIndex::new(src);
        // 'h' is at offset 0 -> line 1, col 1
        assert_eq!(idx.line_col(0), (1, 1));
        // 'w' is at offset 6 -> line 2, col 1
        assert_eq!(idx.line_col(6), (2, 1));
        // 'f' is at offset 12 -> line 3, col 1
        assert_eq!(idx.line_col(12), (3, 1));
        // 'o' (second char of "foo") is at offset 13 -> line 3, col 2
        assert_eq!(idx.line_col(13), (3, 2));
    }

    #[test]
    fn line_index_newline_at_offset() {
        let src = "ab\ncd";
        let idx = LineIndex::new(src);
        // '\n' is at offset 2 -> still line 1, col 3
        assert_eq!(idx.line_col(2), (1, 3));
        // 'c' is at offset 3 -> line 2, col 1
        assert_eq!(idx.line_col(3), (2, 1));
    }

    #[test]
    fn line_index_line_count() {
        let idx = LineIndex::new("a\nb\nc");
        assert_eq!(idx.line_count(), 3);
    }
}
