use crate::token::Token;

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<String>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut line = 1;
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
                if let Some((_, '=')) = chars.peek() {
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
                if let Some((_, '=')) = chars.peek() {
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
                if let Some((_, '=')) = chars.peek() {
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
                if let Some((_, '=')) = chars.peek() {
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
            '/' => {
                if let Some((_, '/')) = chars.peek() {
                    while let Some((_, next)) = chars.peek() {
                        if *next == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    tokens.push(Token::Slash {
                        lexeme: &source[idx..idx + 1],
                        literal: None,
                        line,
                    });
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                line += 1;
            }
            '"' => {
                let mut end_idx = idx;
                let mut last_char = None;

                while let Some((next_idx, next)) = chars.peek() {
                    end_idx = *next_idx;
                    last_char = Some(*next);
                    if *next == '\n' {
                        line += 1;
                    }
                    if *next == '"' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }

                match last_char {
                    Some('"') => {
                        tokens.push(Token::String {
                            lexeme: &source[idx..end_idx + 1],
                            literal: Some(&source[idx + 1..end_idx]),
                            line,
                        });
                    }
                    _ => {
                        errors.push(format!("Unterminated string on line {}", line));
                    }
                }
            }
            '0'..='9' => {
                let mut end_idx = idx;
                let mut contains_dot = false;
                while let Some((next_idx, next_char)) = chars.peek() {
                    match *next_char {
                        '.' => {
                            end_idx = *next_idx;
                            contains_dot = true
                        }
                        c if c.is_ascii_digit() => end_idx = *next_idx,
                        _ => break,
                    }
                    chars.next();
                }

                let lexeme = &source[idx..=end_idx];
                if contains_dot {
                    match lexeme.parse::<f64>() {
                        Ok(num) => tokens.push(Token::Float {
                            lexeme,
                            literal: Some(num),
                            line,
                        }),
                        Err(_) => errors.push(format!("Invalid float on line {}", line)),
                    }
                } else {
                    match lexeme.parse::<i64>() {
                        Ok(num) => tokens.push(Token::Integer {
                            lexeme,
                            literal: Some(num),
                            line,
                        }),
                        Err(_) => errors.push(format!("Invalid integer on line {}", line)),
                    }
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

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_integer() {
        let source = "123";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0],
            Token::Integer {
                lexeme: "123",
                literal: Some(123),
                line: 1,
            }
        );
    }

    #[test]
    fn test_simple_float() {
        let source = "123.456";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0],
            Token::Float {
                lexeme: "123.456",
                literal: Some(123.456),
                line: 1,
            }
        );
    }

    #[test]
    fn test_negative_integer() {
        let source = "-123";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0],
            Token::Minus {
                lexeme: "-",
                literal: None,
                line: 1,
            }
        );
        assert_eq!(
            tokens[1],
            Token::Integer {
                lexeme: "123",
                literal: Some(123),
                line: 1,
            }
        );
    }

    #[test]
    fn test_negative_float() {
        let source = "-123.456";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0],
            Token::Minus {
                lexeme: "-",
                literal: None,
                line: 1,
            }
        );
        assert_eq!(
            tokens[1],
            Token::Float {
                lexeme: "123.456",
                literal: Some(123.456),
                line: 1,
            }
        );
    }

    #[test]
    fn test_leading_zeros_integer() {
        let source = "007";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0],
            Token::Integer {
                lexeme: "007",
                literal: Some(7),
                line: 1,
            }
        );
    }

    #[test]
    fn test_leading_zeros_float() {
        let source = "0.007";
        let tokens = scan_tokens(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0],
            Token::Float {
                lexeme: "0.007",
                literal: Some(0.007),
                line: 1,
            }
        );
    }
}
