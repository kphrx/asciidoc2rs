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

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        tokens.append(&mut lex_line(line));
        tokens.push(Token::NewLine);
    }

    if let Some(Token::NewLine) = tokens.last() {
        tokens.pop();
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
                    tokens.push(Token::Delimiter("=".repeat(level)));

                    break;
                }

                if chars.peek() != Some(&' ') {
                    buffer += &"=".repeat(level);

                    break;
                }

                while let Some(' ') = chars.peek() {
                    chars.next();
                }

                tokens.push(Token::Heading(level));
            },
            _ => buffer.push(c),
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer.clone()));
        buffer.clear();
    }

    tokens
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
