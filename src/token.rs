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
    Identifier {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    And {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Class {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Else {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    False {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Fun {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    For {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    If {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Nil {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Or {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Print {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Return {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Super {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    This {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    True {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Var {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    While {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
    Eof {
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    },
}
