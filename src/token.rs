#[derive(Debug)]
pub enum Token<'a> {
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
    Star {
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
