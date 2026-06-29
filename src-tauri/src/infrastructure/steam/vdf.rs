//! Minimal parser for Valve's KeyValues (VDF/ACF) text format.
//!
//! We do not need a full VDF object model — only the ability to extract the
//! values we care about (library paths, `installdir`). The parser is tolerant
//! of formatting variations and nested blocks.

/// A flat list of `(key, value)` string pairs in document order. Nested keys
/// (blocks) are represented by their key with no value entry; scalar keys carry
/// their value.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct KeyValues {
    pub pairs: Vec<(String, String)>,
}

impl KeyValues {
    /// Collect every scalar value whose key equals `key` (case-insensitive).
    pub fn values_for(&self, key: &str) -> Vec<&str> {
        self.pairs
            .iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
            .collect()
    }

    /// First scalar value for `key`, if any.
    pub fn first(&self, key: &str) -> Option<&str> {
        self.values_for(key).into_iter().next()
    }
}

/// Parse VDF/ACF text into a flat list of quoted `(key, value)` scalar pairs.
///
/// The format uses double-quoted tokens. A key followed by another quoted token
/// on the same logical position is a scalar; a key followed by `{` opens a
/// block. We only record scalar key/value pairs, which is sufficient for
/// `libraryfolders.vdf` (`"path"` entries) and `appmanifest_*.acf`
/// (`"installdir"`, `"appid"`).
pub fn parse(input: &str) -> KeyValues {
    let mut tokens = Tokenizer::new(input);
    let mut pairs = Vec::new();

    // Pending key awaiting its value or block.
    let mut pending_key: Option<String> = None;

    while let Some(tok) = tokens.next_token() {
        match tok {
            Token::Quoted(text) => {
                if let Some(key) = pending_key.take() {
                    // key followed by a quoted value => scalar pair
                    pairs.push((key, text));
                } else {
                    pending_key = Some(text);
                }
            }
            Token::OpenBrace => {
                // The pending key was a block header; drop it (we keep it flat).
                pending_key = None;
            }
            Token::CloseBrace => {
                pending_key = None;
            }
        }
    }

    KeyValues { pairs }
}

enum Token {
    Quoted(String),
    OpenBrace,
    CloseBrace,
}

struct Tokenizer<'a> {
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            peeked: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.peeked.take() {
            Some(c)
        } else {
            self.chars.next()
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            let c = self.next_char()?;
            match c {
                c if c.is_whitespace() => continue,
                '{' => return Some(Token::OpenBrace),
                '}' => return Some(Token::CloseBrace),
                '/' if self.peek_char() == Some('/') => {
                    // Line comment.
                    for nc in self.by_ref() {
                        if nc == '\n' {
                            break;
                        }
                    }
                    continue;
                }
                '"' => return Some(Token::Quoted(self.read_quoted())),
                _ => continue, // ignore stray characters
            }
        }
    }

    fn read_quoted(&mut self) -> String {
        let mut out = String::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => break,
                '\\' => {
                    // VDF escapes: \\ \" \n \t
                    if let Some(esc) = self.next_char() {
                        match esc {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            other => out.push(other),
                        }
                    }
                }
                other => out.push(other),
            }
        }
        out
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.next_char()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_library_folders() {
        let input = r#"
        "libraryfolders"
        {
            "0"
            {
                "path"  "C:\\Program Files (x86)\\Steam"
                "apps" { "550" "12345" }
            }
            "1"
            {
                "path"  "D:\\SteamLibrary"
            }
        }
        "#;
        let kv = parse(input);
        let paths = kv.values_for("path");
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], r"C:\Program Files (x86)\Steam");
        assert_eq!(paths[1], r"D:\SteamLibrary");
    }

    #[test]
    fn parses_appmanifest() {
        let input = r#"
        "AppState"
        {
            "appid"      "550"
            "installdir" "Left 4 Dead 2"
        }
        "#;
        let kv = parse(input);
        assert_eq!(kv.first("appid"), Some("550"));
        assert_eq!(kv.first("installdir"), Some("Left 4 Dead 2"));
    }
}
