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

            '!' | '=' | '<' | '>' | '/' => {
                let next_char = chars.peek().map(|&(_, nc)| nc);
                match (c, next_char) {
                    ('!', Some('=')) | ('=', Some('=')) | ('<', Some('=')) | ('>', Some('=')) => {
                        chars.next();
                        tokens.push(match c {
                            '!' => Token::BangEqual {
                                lexeme: &source[idx..idx + 2],
                                literal: None,
                                line,
                            },
                            '=' => Token::EqualEqual {
                                lexeme: &source[idx..idx + 2],
                                literal: None,
                                line,
                            },
                            '<' => Token::LessEqual {
                                lexeme: &source[idx..idx + 2],
                                literal: None,
                                line,
                            },
                            '>' => Token::GreaterEqual {
                                lexeme: &source[idx..idx + 2],
                                literal: None,
                                line,
                            },
                            _ => unreachable!(),
                        });
                    }
                    ('/', Some('/')) => {
                        while let Some((_, next)) = chars.peek() {
                            if *next == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    }
                    _ => {
                        tokens.push(match c {
                            '!' => Token::Bang {
                                lexeme: &source[idx..idx + 1],
                                literal: None,
                                line,
                            },
                            '=' => Token::Equal {
                                lexeme: &source[idx..idx + 1],
                                literal: None,
                                line,
                            },
                            '<' => Token::Less {
                                lexeme: &source[idx..idx + 1],
                                literal: None,
                                line,
                            },
                            '>' => Token::Greater {
                                lexeme: &source[idx..idx + 1],
                                literal: None,
                                line,
                            },
                            '/' => Token::Slash {
                                lexeme: &source[idx..idx + 1],
                                literal: None,
                                line,
                            },
                            _ => unreachable!(),
                        });
                    }
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                line += 1;
            }

            '"' => {
                let mut end_idx = idx;
                let mut terminated = false;

                while let Some((next_idx, next_char)) = chars.peek() {
                    end_idx = *next_idx;
                    match *next_char {
                        '\n' => {
                            line += 1;
                            chars.next();
                        }
                        '"' => {
                            terminated = true;
                            chars.next();
                            break;
                        }
                        _ => {
                            chars.next();
                        }
                    }
                }

                if terminated {
                    tokens.push(Token::String {
                        lexeme: &source[idx..=end_idx],
                        literal: Some(&source[idx + 1..end_idx]),
                        line,
                    });
                } else {
                    errors.push(format!("Unterminated string on line {}", line));
                }
            }

            '0'..='9' => {
                let mut end_idx = idx;
                let mut contains_dot = false;

                while let Some((next_idx, next_char)) = chars.peek() {
                    match *next_char {
                        '.' => {
                            end_idx = *next_idx;
                            contains_dot = true;
                            chars.next();
                        }
                        c if c.is_ascii_digit() => {
                            end_idx = *next_idx;
                            chars.next();
                        }
                        _ => break,
                    }
                }

                let lexeme = &source[idx..=end_idx];
                if contains_dot {
                    match lexeme.chars().last() {
                        Some(last_char) if last_char.is_ascii_digit() => {
                            match lexeme.parse::<f64>() {
                                Ok(num) => tokens.push(Token::Float {
                                    lexeme,
                                    literal: Some(num),
                                    line,
                                }),
                                Err(_) => errors.push(format!("Invalid float on line {}", line)),
                            }
                        }
                        _ => {
                            errors.push(format!(
                                "Invalid float on line {}  - last char is a .",
                                line
                            ));
                        }
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

            c if c.is_ascii_alphabetic() => {
                let mut end_idx = idx;
                while let Some((next_idx, next_char)) = chars.peek() {
                    match *next_char {
                        c if c.is_ascii_alphanumeric() => {
                            end_idx = *next_idx;
                            chars.next();
                        }
                        _ => break,
                    }
                }

                let lexeme = &source[idx..=end_idx];
                tokens.push(match lexeme {
                    "and" => Token::And {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "class" => Token::Class {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "else" => Token::Else {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "false" => Token::False {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "for" => Token::For {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "fun" => Token::Fun {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "if" => Token::If {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "nil" => Token::Nil {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "or" => Token::Or {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "print" => Token::Print {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "return" => Token::Return {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "super" => Token::Super {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "this" => Token::This {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "true" => Token::True {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "var" => Token::Var {
                        lexeme,
                        literal: None,
                        line,
                    },
                    "while" => Token::While {
                        lexeme,
                        literal: None,
                        line,
                    },
                    _ => Token::Identifier {
                        lexeme,
                        literal: None,
                        line,
                    },
                })
            }
            _ => errors.push(format!("Unexpected character: {} on line {}", c, line)),
        };
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

    mod int_and_float {
        use crate::{scanner::scan_tokens, token::Token};

        #[test]
        fn simple_integer() {
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
        fn leading_zeros_integer() {
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
        fn handles_invalid_float_gracefully() {
            let source = "123.45.67";
            let result = scan_tokens(source);
            assert!(result.is_err());
            let errors = result.err().unwrap();
            assert!(!errors.is_empty());
            assert!(errors[0].contains("Invalid float on line 1"));
        }

        #[test]
        fn fails_to_parse_float_with_dot_at_end() {
            let source = "123.";
            let result = scan_tokens(source);
            assert!(result.is_err());
            let errors = result.err().unwrap();
            assert!(!errors.is_empty());
            assert!(errors[0].contains("Invalid float on line 1  - last char is a ."));
        }
    }

    mod strings {
        use crate::{scanner::scan_tokens, token::Token};

        #[test]
        fn parses_terminated_strings_correctly() {
            let source = "\"This is a string.\"";
            let tokens = scan_tokens(source).unwrap();
            assert_eq!(tokens.len(), 2);

            assert_eq!(
                tokens[0],
                Token::String {
                    lexeme: "\"This is a string.\"",
                    literal: Some("This is a string."),
                    line: 1,
                }
            );
        }

        #[test]
        fn reports_unterminated_strings() {
            let source = "\"This string has no end";
            let result = scan_tokens(source);
            assert!(result.is_err());
            let errors = result.err().unwrap();
            assert!(!errors.is_empty());
            assert!(errors[0].contains("Unterminated string"));
        }

        #[test]
        fn handles_strings_with_newlines() {
            let source = "\"This is a string\nwith a newline.\"";
            let tokens = scan_tokens(source).unwrap();
            assert_eq!(tokens.len(), 2);
            assert_eq!(
                tokens[0],
                Token::String {
                    lexeme: "\"This is a string\nwith a newline.\"",
                    literal: Some("This is a string\nwith a newline."),
                    line: 2,
                }
            );
        }
    }
}
