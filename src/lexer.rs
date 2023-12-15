#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Heading(usize),
    CommentDelimiter,
    Comment,
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
    let mut comment_delimiter = 0;

    for line in input.lines() {
        lex_line(line, &mut tokens, &mut comment_delimiter);
        tokens.push(Token::NewLine);
    }

    if let Some(Token::NewLine) = tokens.last() {
        tokens.pop();
    }

    tokens
}

fn lex_line(line: &str, tokens: &mut Vec<Token>, comment_delimiter: &mut usize) {
    let line = line.trim_end_matches(' ');
    let mut buffer = String::new();
    let mut col = 0;
    let mut is_constrainable = true;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        col += 1;
        match (c, col, is_constrainable, *comment_delimiter) {
            ('=', 1, _, 0) => {
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
            ('/', 1, _, 0) => {
                while let Some('/') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() && col >= 4 {
                    tokens.push(Token::CommentDelimiter);
                    *comment_delimiter = col;

                    break;
                }

                if col >= 2 {
                    tokens.push(Token::Comment);

                    break;
                }

                buffer.push(c);
            }
            ('/', 1, _, cd) => {
                while let Some('/') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() && col == cd {
                    tokens.push(Token::CommentDelimiter);
                    *comment_delimiter = 0;
                }
            }
            ('*', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::StrongOpen);
            }
            ('*', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::StrongClose);
                }
            },
            ('_', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::EmphasisOpen);
            }
            ('_', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::EmphasisClose);
                }
            },
            ('`', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::CodeOpen);
            }
            ('`', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::CodeClose);
                }
            },
            ('#', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::MarkOpen);
            }
            ('#', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::MarkClose);
                }
            },
            ('~', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::SubscriptOpen);
            }
            ('~', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::SubscriptClose);
                }
            },
            ('^', _, true, 0) => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::SuperscriptOpen);
            }
            ('^', _, _, 0) => match chars.peek() {
                Some(nc) if !is_constrainable_char(nc) => buffer.push(c),
                _ => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    tokens.push(Token::SuperscriptClose);
                }
            },
            (_, _, _, 0) => {
                buffer.push(c);
                is_constrainable = is_constrainable_char(&c);
            }
            (_, _, _, _) => {}
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
        let input = "// comment\n== Heading 2\n\n====\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n\n/////\n====\n////\n/////";
        let expected_output = vec![
            Token::Comment,
            Token::NewLine,
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
            Token::NewLine,
            Token::CommentDelimiter,
            Token::NewLine,
            Token::NewLine,
            Token::NewLine,
            Token::CommentDelimiter,
        ];

        assert_eq!(lex(input), expected_output);
    }
}
