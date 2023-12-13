#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Block(BToken),
    Inline(IToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BToken {
    Heading(usize, Vec<IToken>),
    Delimiter(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IToken {
    Text(String),
    Strong(String),
    Emphasis(String),
    Code(String),
    Mark(String),
    Subscript(String),
    Superscript(String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        tokens.append(&mut lex_line(line));
        tokens.push(Token::Inline(IToken::Text("\n".to_string())));
    }

    tokens
}

fn lex_line(line: &str) -> Vec<Token> {
    let line = line.trim_end_matches(' ');
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '=' => {
                let mut level = 1;
                while let Some('=') = chars.peek() {
                    chars.next();
                    level += 1;
                }

                if chars.peek().is_none() && level >= 4 {
                    tokens.push(Token::Block(BToken::Delimiter(line.to_string())));

                    break;
                }

                if chars.peek() != Some(&' ') {
                    tokens.push(Token::Inline(IToken::Text(line.to_string())));

                    break;
                }

                while let Some(' ') = chars.peek() {
                    chars.next();
                }

                let text = chars.collect::<String>();
                tokens.push(Token::Block(BToken::Heading(
                    level,
                    lex_inline(text.as_str()),
                )));

                break;
            }
            _ => buffer.push(c),
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::Inline(IToken::Text(buffer.clone())));
        buffer.clear();
    }

    tokens
}

fn lex_inline(_text: &str) -> Vec<IToken> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "== Heading 2\n\n====\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n====";
        let expected_output = vec![
            Token::Block(BToken::Heading(
                2,
                vec![IToken::Text("Heading 2".to_string())],
            )),
            Token::Inline(IToken::Text("\n".to_string())),
            Token::Inline(IToken::Text("\n".to_string())),
            Token::Block(BToken::Delimiter("====".to_string())),
            Token::Inline(IToken::Text("\n".to_string())),
            Token::Inline(IToken::Text("More ".to_string())),
            Token::Inline(IToken::Strong("bold".to_string())),
            Token::Inline(IToken::Text(" and ".to_string())),
            Token::Inline(IToken::Emphasis("italic".to_string())),
            Token::Inline(IToken::Text(" and ".to_string())),
            Token::Inline(IToken::Code("monospace".to_string())),
            Token::Inline(IToken::Text(" and ".to_string())),
            Token::Inline(IToken::Mark("highlight".to_string())),
            Token::Inline(IToken::Text(" and ".to_string())),
            Token::Inline(IToken::Subscript("subscript".to_string())),
            Token::Inline(IToken::Text(" and ".to_string())),
            Token::Inline(IToken::Superscript("superscript".to_string())),
            Token::Inline(IToken::Text(" text.".to_string())),
            Token::Inline(IToken::Text("\n".to_string())),
            Token::Block(BToken::Delimiter("====".to_string())),
        ];

        assert_eq!(lex(input), expected_output);
    }
}
