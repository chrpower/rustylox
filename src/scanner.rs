use crate::token::Token;


pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<String>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let line = 1;
    let mut chars = source.char_indices().peekable();

    while let Some((idx, c)) = chars.next() {
        match c {
            '(' => tokens.push(Token::LeftParen {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            ')' => tokens.push(Token::RightParen {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '{' => tokens.push(Token::LeftBrace {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '}' => tokens.push(Token::RightBrace {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            ',' => tokens.push(Token::Comma {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '.' => tokens.push(Token::Dot {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '-' => tokens.push(Token::Minus {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '+' => tokens.push(Token::Plus {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            ';' => tokens.push(Token::Semicolon {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '*' => tokens.push(Token::Star {
                lexeme: &source[idx..idx + 1],
                literal: None,
                line,
            }),
            '!' => {
                let next = chars.peek();
                if let Some((_, '=')) = next {
                    chars.next();
                    tokens.push(Token::BangEqual {
                        lexeme: &source[idx..idx + 2],
                        literal: None,
                        line,
                    });
                } else {
                    tokens.push(Token::Bang {
                        lexeme: &source[idx..idx + 1],
                        literal: None,
                        line,
                    });
                }
            }
            '=' => {
                let next = chars.peek();
                if let Some((_, '=')) = next {
                    chars.next();
                    tokens.push(Token::EqualEqual {
                        lexeme: &source[idx..idx + 2],
                        literal: None,
                        line,
                    });
                } else {
                    tokens.push(Token::Equal {
                        lexeme: &source[idx..idx + 1],
                        literal: None,
                        line,
                    });
                }
            }
            '<' => {
                let next = chars.peek();
                if let Some((_, '=')) = next {
                    chars.next();
                    tokens.push(Token::LessEqual {
                        lexeme: &source[idx..idx + 2],
                        literal: None,
                        line,
                    });
                } else {
                    tokens.push(Token::Less {
                        lexeme: &source[idx..idx + 1],
                        literal: None,
                        line,
                    });
                }
            }
            '>' => {
                let next = chars.peek();
                if let Some((_, '=')) = next {
                    chars.next();
                    tokens.push(Token::GreaterEqual {
                        lexeme: &source[idx..idx + 2],
                        literal: None,
                        line,
                    });
                } else {
                    tokens.push(Token::Greater {
                        lexeme: &source[idx..idx + 1],
                        literal: None,
                        line,
                    });
                }
            }

            _ => errors.push(format!("Unexpected character: {} on line {}", c, line)),
        }
    }

    tokens.push(Token::Eof {
        lexeme: "",
        literal: None,
        line,
    });

    if errors.len() > 0 {
        return Err(errors);
    }
    Ok(tokens)
}
