use crate::token::Token;

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
    let mut prev = None;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        col += 1;

        match (c, prev, chars.peek(), *comment_delimiter) {
            ('/', None, Some('/'), 0) => {
                while let Some('/') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() && col >= 4 {
                    tokens.push(Token::CommentDelimiter);
                    *comment_delimiter = col;

                    break;
                }

                tokens.push(Token::Comment);

                break;
            }
            ('/', None, Some('/'), cd) => {
                while let Some('/') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() && col == cd {
                    tokens.push(Token::CommentDelimiter);
                    *comment_delimiter = 0;
                }
            }
            ('=', None, _, 0) => {
                while let Some('=') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                if chars.peek().is_none() && col >= 4 {
                    tokens.push(Token::ExampleDelimiter(col));

                    break;
                }

                if chars.peek() != Some(&' ') {
                    buffer += &"=".repeat(col);
                    prev = Some('=');

                    continue;
                }

                tokens.push(Token::Heading(col));

                while let Some(' ') = chars.peek() {
                    chars.next();
                    col += 1;
                }
            }
            ('*', None, _, 0) => {
                while let Some('*') = chars.peek() {
                    chars.next();
                    col += 1;
                }

                let next = chars.peek();

                if next.is_none() && col >= 4 {
                    tokens.push(Token::SidebarDelimiter(col));

                    break;
                }

                if next.is_none() && col == 1 {
                    buffer.push(c);
                    prev = Some(c);

                    continue;
                }

                if let Some(' ') = next {
                    tokens.push(Token::UnorderedList(col));

                    while let Some(' ') = chars.peek() {
                        chars.next();
                        col += 1;
                    }

                    prev = Some(' ');

                    continue;
                }

                for _ in 1..col {
                    tokens.push(Token::Strong(true, true));
                }
                tokens.push(Token::Strong(
                    true,
                    next.map_or(true, |c| !c.is_alphanumeric()),
                ));
            }
            ('*', Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('*', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Strong(!is_wordy_prev, !is_wordy_next));
                }
            },
            ('_', None | Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('_', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Emphasis(!is_wordy_prev, !is_wordy_next));
                }
            },
            ('`', None | Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('`', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Code(!is_wordy_prev, !is_wordy_next));
                }
            },
            ('#', None | Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('#', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Mark(!is_wordy_prev, !is_wordy_next));
                }
            },
            ('~', None | Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('~', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Subscript(!is_wordy_prev, !is_wordy_next));
                }
            },
            ('^', None | Some(' '), None | Some(' '), 0) => {
                buffer.push(c);
            }
            ('^', pr, nx, 0) => match (
                pr.map_or(false, |c| c.is_alphanumeric()),
                nx.map_or(false, |c| c.is_alphanumeric()),
            ) {
                (true, true) => {
                    buffer.push(c);
                }
                (is_wordy_prev, is_wordy_next) => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    tokens.push(Token::Superscript(!is_wordy_prev, !is_wordy_next));
                }
            },
            (_, _, _, 0) => {
                buffer.push(c);
            }
            (_, _, _, _) => {
                break;
            }
        }

        prev = Some(c);
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
    fn test_lex_constrained_mark() {
        let input = "== Heading 2\n\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Text("More ".to_string()),
            Token::Strong(true, false),
            Token::Text("bold".to_string()),
            Token::Strong(false, true),
            Token::Text(" and ".to_string()),
            Token::Emphasis(true, false),
            Token::Text("italic".to_string()),
            Token::Emphasis(false, true),
            Token::Text(" and ".to_string()),
            Token::Code(true, false),
            Token::Text("monospace".to_string()),
            Token::Code(false, true),
            Token::Text(" and ".to_string()),
            Token::Mark(true, false),
            Token::Text("highlight".to_string()),
            Token::Mark(false, true),
            Token::Text(" and ".to_string()),
            Token::Subscript(true, false),
            Token::Text("subscript".to_string()),
            Token::Subscript(false, true),
            Token::Text(" and ".to_string()),
            Token::Superscript(true, false),
            Token::Text("superscript".to_string()),
            Token::Superscript(false, true),
            Token::Text(" text.".to_string()),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_constrained_mark_edge() {
        let input = "== Heading 2\n\nMore **constrain*ed * bold* and _constrain_ed _ italic__ and ` `constrain`ed(`monospace`)` and #constrain#ed highlight# # and ~constrain~ed ~ subscript~ and ^constrain^ed ^ superscript^ text.";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Text("More ".to_string()),
            Token::Strong(true, true),
            Token::Strong(true, false),
            Token::Text("constrain*ed * bold".to_string()),
            Token::Strong(false, true),
            Token::Text(" and ".to_string()),
            Token::Emphasis(true, false),
            Token::Text("constrain_ed _ italic".to_string()),
            Token::Emphasis(false, true),
            Token::Emphasis(true, true),
            Token::Text(" and ` ".to_string()),
            Token::Code(true, false),
            Token::Text("constrain`ed(".to_string()),
            Token::Code(true, false),
            Token::Text("monospace".to_string()),
            Token::Code(false, true),
            Token::Text(")".to_string()),
            Token::Code(true, true),
            Token::Text(" and ".to_string()),
            Token::Mark(true, false),
            Token::Text("constrain#ed highlight".to_string()),
            Token::Mark(false, true),
            Token::Text(" # and ".to_string()),
            Token::Subscript(true, false),
            Token::Text("constrain~ed ~ subscript".to_string()),
            Token::Subscript(false, true),
            Token::Text(" and ".to_string()),
            Token::Superscript(true, false),
            Token::Text("constrain^ed ^ superscript".to_string()),
            Token::Superscript(false, true),
            Token::Text(" text.".to_string()),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_unordered_lists() {
        let input =
            "== Heading 2\n\n*** Unordered level 3 list item\n* Unordered level 1 list item\n** Unordered level 2 list item\n**** Unordered level 4 list item";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::UnorderedList(3),
            Token::Text("Unordered level 3 list item".to_string()),
            Token::NewLine,
            Token::UnorderedList(1),
            Token::Text("Unordered level 1 list item".to_string()),
            Token::NewLine,
            Token::UnorderedList(2),
            Token::Text("Unordered level 2 list item".to_string()),
            Token::NewLine,
            Token::UnorderedList(4),
            Token::Text("Unordered level 4 list item".to_string()),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_asterisk_edge() {
        let input = "== Heading 2\n\n* Unordered level 1 list item\n*\n*****\n***\n****strong mark";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::UnorderedList(1),
            Token::Text("Unordered level 1 list item".to_string()),
            Token::NewLine,
            Token::Text("*".to_string()),
            Token::NewLine,
            Token::SidebarDelimiter(5),
            Token::NewLine,
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::NewLine,
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::Strong(true, false),
            Token::Text("strong mark".to_string()),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_examples_block_delimiter() {
        let input =
            "== Heading 2\n\n====\n= Block heading 1\n\nMore *bold* and _italic_ text.\n====";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::ExampleDelimiter(4),
            Token::NewLine,
            Token::Heading(1),
            Token::Text("Block heading 1".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Text("More ".to_string()),
            Token::Strong(true, false),
            Token::Text("bold".to_string()),
            Token::Strong(false, true),
            Token::Text(" and ".to_string()),
            Token::Emphasis(true, false),
            Token::Text("italic".to_string()),
            Token::Emphasis(false, true),
            Token::Text(" text.".to_string()),
            Token::NewLine,
            Token::ExampleDelimiter(4),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_sidebars_block_delimiter() {
        let input =
            "== Heading 2\n\n****\n= Block heading 1\n\nMore *bold* and _italic_ text.\n****";
        let expected_output = vec![
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::SidebarDelimiter(4),
            Token::NewLine,
            Token::Heading(1),
            Token::Text("Block heading 1".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Text("More ".to_string()),
            Token::Strong(true, false),
            Token::Text("bold".to_string()),
            Token::Strong(false, true),
            Token::Text(" and ".to_string()),
            Token::Emphasis(true, false),
            Token::Text("italic".to_string()),
            Token::Emphasis(false, true),
            Token::Text(" text.".to_string()),
            Token::NewLine,
            Token::SidebarDelimiter(4),
        ];

        assert_eq!(lex(input), expected_output);
    }

    #[test]
    fn test_lex_comment_out() {
        let input = "// comment\n== Heading 2\n\n/////\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n////\n/////";
        let expected_output = vec![
            Token::Comment,
            Token::NewLine,
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
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
