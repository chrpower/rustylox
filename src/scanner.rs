use crate::token::Token;

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let line = 1;
    let mut chars = source.char_indices();

    while let Some((idx, c)) = chars.next() {
        match c {
            '(' => tokens.push(Token::LeftParen {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            ')' => tokens.push(Token::RightParen {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '{' => tokens.push(Token::LeftBrace {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '}' => tokens.push(Token::RightBrace {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            ',' => tokens.push(Token::Comma {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '.' => tokens.push(Token::Dot {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '-' => tokens.push(Token::Minus {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '+' => tokens.push(Token::Plus {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            ';' => tokens.push(Token::Semicolon {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            '*' => tokens.push(Token::Star {
                lexeme: &source[idx..idx + c.len_utf8()],
                literal: None,
                line,
            }),
            _ => todo!(),
        }
    }

    tokens.push(Token::Eof {
        lexeme: "",
        literal: None,
        line,
    });
    tokens
}
