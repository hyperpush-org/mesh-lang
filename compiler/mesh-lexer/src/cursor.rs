/// Byte-level source iterator for the Mesh lexer.
///
/// The cursor wraps a source string and provides character-by-character
/// iteration with byte-offset position tracking. All positions are byte
/// offsets into the original UTF-8 source text.
pub struct Cursor<'src> {
    source: &'src str,
    pos: u32,
    chars: std::str::Chars<'src>,
}

impl<'src> Cursor<'src> {
    /// Create a new cursor at the start of the source text.
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            pos: 0,
            chars: source.chars(),
        }
    }

    /// Look at the current character without consuming it.
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Look at the character after the current one without consuming anything.
    pub fn peek_next(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    /// Consume the current character and advance the position.
    ///
    /// Returns the consumed character, or `None` if at end of input.
    pub fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += c.len_utf8() as u32;
        Some(c)
    }

    /// Current byte position in the source text.
    pub fn pos(&self) -> u32 {
        self.pos
    }

    /// Whether there are no more characters to consume.
    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }

    /// Advance while the predicate holds for the current character.
    pub fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Extract a slice of the source text by byte offsets.
    ///
    /// # Panics
    ///
    /// Panics if start or end are out of bounds or not on UTF-8 boundaries.
    pub fn slice(&self, start: u32, end: u32) -> &'src str {
        &self.source[start as usize..end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cursor_starts_at_zero() {
        let cursor = Cursor::new("hello");
        assert_eq!(cursor.pos(), 0);
        assert!(!cursor.is_eof());
    }

    #[test]
    fn peek_does_not_advance() {
        let cursor = Cursor::new("ab");
        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.pos(), 0);
    }

    #[test]
    fn peek_next_looks_ahead() {
        let cursor = Cursor::new("ab");
        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.peek_next(), Some('b'));
        assert_eq!(cursor.pos(), 0);
    }

    #[test]
    fn advance_moves_position() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.pos(), 1);
        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.pos(), 2);
        assert_eq!(cursor.advance(), Some('c'));
        assert_eq!(cursor.pos(), 3);
        assert_eq!(cursor.advance(), None);
        assert!(cursor.is_eof());
    }

    #[test]
    fn advance_tracks_multibyte_utf8() {
        // U+00E9 (e with accent) is 2 bytes in UTF-8
        let mut cursor = Cursor::new("\u{00E9}a");
        assert_eq!(cursor.advance(), Some('\u{00E9}'));
        assert_eq!(cursor.pos(), 2);
        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.pos(), 3);
    }

    #[test]
    fn eat_while_consumes_matching() {
        let mut cursor = Cursor::new("aaab");
        cursor.eat_while(|c| c == 'a');
        assert_eq!(cursor.pos(), 3);
        assert_eq!(cursor.peek(), Some('b'));
    }

    #[test]
    fn slice_extracts_text() {
        let cursor = Cursor::new("hello world");
        assert_eq!(cursor.slice(0, 5), "hello");
        assert_eq!(cursor.slice(6, 11), "world");
    }

    #[test]
    fn empty_source() {
        let cursor = Cursor::new("");
        assert!(cursor.is_eof());
        assert_eq!(cursor.peek(), None);
        assert_eq!(cursor.peek_next(), None);
    }
}
