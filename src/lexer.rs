#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Heading(usize),
    Delimiter(String),
    NewLine,
    Text(String),
    StrongOpen,
    StrongClose,
    EmphasisOpen,
    EmphasisClose,
    CodeOpen,
    CodeClose,
    MarkOpen,
    MarkClose,
    SubscriptOpen,
    SubscriptClose,
    SuperscriptOpen,
    SuperscriptClose,
}

fn is_constrainable_char(c: &char) -> bool {
    matches!(Some(c), Some(' ' | ',' | ';' | '"' | '.' | '?' | '!'))
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    for line in input.lines() {
        lex_line(line, &mut tokens);
        tokens.push(Token::NewLine);
    }

    if let Some(Token::NewLine) = tokens.last() {
        tokens.pop();
    }

    tokens
}

fn lex_line(line: &str, tokens: &mut Vec<Token>) {
    let line = line.trim_end_matches(' ');
    let mut buffer = String::new();
    let mut col = 0;
    let mut is_constrainable = true;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        col += 1;
        match (c, col, is_constrainable) {
            ('=', 1, _) => {
                while let Some('=') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() {
                    if col >= 4 {
                        tokens.push(Token::Delimiter("=".repeat(col)));
                    } else {
                        tokens.push(Token::Text("=".repeat(col)));
                    }

                    break;
                }

                if chars.peek() != Some(&' ') {
                    buffer += &"=".repeat(col);
                    is_constrainable = false;

                    continue;
                }

                tokens.push(Token::Heading(col));

                while let Some(' ') = chars.peek() {
                    chars.next();
                    col += 1;
                }
            }
            ('*', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::StrongOpen);
            }
            ('*', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::StrongClose);
                }
            },
            ('_', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::EmphasisOpen);
            }
            ('_', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::EmphasisClose);
                }
            },
            ('`', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::CodeOpen);
            }
            ('`', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::CodeClose);
                }
            },
            ('#', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::MarkOpen);
            }
            ('#', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::MarkClose);
                }
            },
            ('~', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::SubscriptOpen);
            }
            ('~', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::SubscriptClose);
                }
            },
            ('^', _, true) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::SuperscriptOpen);
            }
            ('^', _, _) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::SuperscriptClose);
                }
            },
            (_, _, _) => {
                buffer.push(c);
                is_constrainable = is_constrainable_char(&c);
            }
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer.clone()));
        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "== Heading 2\n\n====\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n====";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Delimiter("====".to_string()),
            Token::NewLine,
            Token::Text("More ".to_string()),
            Token::StrongOpen,
            Token::Text("bold".to_string()),
            Token::StrongClose,
            Token::Text(" and ".to_string()),
            Token::EmphasisOpen,
            Token::Text("italic".to_string()),
            Token::EmphasisClose,
            Token::Text(" and ".to_string()),
            Token::CodeOpen,
            Token::Text("monospace".to_string()),
            Token::CodeClose,
            Token::Text(" and ".to_string()),
            Token::MarkOpen,
            Token::Text("highlight".to_string()),
            Token::MarkClose,
            Token::Text(" and ".to_string()),
            Token::SubscriptOpen,
            Token::Text("subscript".to_string()),
            Token::SubscriptClose,
            Token::Text(" and ".to_string()),
            Token::SuperscriptOpen,
            Token::Text("superscript".to_string()),
            Token::SuperscriptClose,
            Token::Text(" text.".to_string()),
            Token::NewLine,
            Token::Delimiter("====".to_string()),
        ];

        assert_eq!(lex(input), expected_output);
    }
}
