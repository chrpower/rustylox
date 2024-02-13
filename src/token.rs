#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Single-character tokens.
    LeftParen {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    RightParen {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    LeftBrace {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    RightBrace {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Comma {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Dot {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Minus {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Plus {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Semicolon {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Slash {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Star {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },

    // One or two character tokens.
    Bang {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    BangEqual {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Equal {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    EqualEqual {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Greater {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    GreaterEqual {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Less {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    LessEqual {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    String {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Integer {
        lexeme: &'a str,
        literal: Option<i64>,
        line: usize,
    },
    Float {
        lexeme: &'a str,
        literal: Option<f64>,
        line: usize,
    },
    Eof {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
}
