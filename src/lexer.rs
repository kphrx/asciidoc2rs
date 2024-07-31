use crate::token::Token;

use lexgen::lexer;

struct LexerState {
    prev_space_like: bool,
    opened_comment_block: usize,
}

impl Default for LexerState {
    fn default() -> Self {
        Self {
            prev_space_like: true,
            opened_comment_block: 0,
        }
    }
}

lexer! {
    Lexer(LexerState) -> Token;

    let eol = ['\r' '\n'];
    let unconstrained_marks = ['~' '^'];
    let constrained_marks = ['`' '_' '*' '#'];
    let inline_marks = $constrained_marks | $unconstrained_marks;
    let word_character = ['a'-'z' 'A'-'Z' '0'-'9' '_'];

    rule Init {
        ("\r\n" | $eol)+ $,
        "\r\n" | $eol => |lexer| lexer.return_(Token::NewLine),

        "//" (_ # '/') (_ # $eol | $)* > ($eol | $) => |lexer| lexer.return_(Token::Comment),

        "////" '/'* > ($eol | $) => |lexer| {
            let count = lexer.match_().chars().count();
            lexer.state().opened_comment_block = count;
            lexer.switch_and_return(LexerRule::Comment, Token::CommentDelimiter(count))
        },

        '='+ ' '+ => |lexer| {
            let count = lexer.match_().trim_end_matches(' ').chars().count();
            lexer.switch_and_return(LexerRule::Inline, Token::Heading(count))
        },

        "====" '='* > ($eol | $) => |lexer| {
            let count = lexer.match_().chars().count();
            lexer.return_(Token::ExampleDelimiter(count))
        },

        "****" '*'* > ($eol | $) => |lexer| {
            let count = lexer.match_().chars().count();
            lexer.return_(Token::SidebarDelimiter(count))
        },

        '*'+ ' '+ => |lexer| {
            let count = lexer.match_().trim_end_matches(' ').chars().count();
            lexer.switch_and_return(LexerRule::Inline, Token::UnorderedList(count))
        },

        ':' => |lexer| lexer.switch(LexerRule::Attributes),

        '`' > ([' ' '(' ')'] | $inline_marks) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Code(prev_is_space_like, true))
        },

        '`' > (_ # $eol) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Code(prev_is_space_like, false))
        },

        '_' > ([' ' '(' ')'] | $inline_marks) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Emphasis(prev_is_space_like, true))
        },

        '_' > (_ # $eol) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Emphasis(prev_is_space_like, false))
        },

        '*' > ([' ' '(' ')'] | $inline_marks) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Strong(prev_is_space_like, true))
        },

        '*' > (_ # $eol) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Strong(prev_is_space_like, false))
        },

        '~' > (_ # $eol) => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Subscript)
        },

        '^' > (_ # $eol) => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Superscript)
        },

        '#' > ([' ' '(' ')'] | $inline_marks) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Mark(prev_is_space_like, true))
        },

        '#' > (_ # $eol) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.switch_and_return(LexerRule::Inline, Token::Mark(prev_is_space_like, false))
        },

        (_ # ($inline_marks | $eol))* [' ' '(' ')'] $constrained_marks+ > [' ' '(' ')'] => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.switch(LexerRule::Inline)
        },

        (_ # ($inline_marks | $eol))* (_ # ([' ' '(' ')'] | $constrained_marks | $eol)) $constrained_marks+ > (_ # ([' ' '(' ')'] | $constrained_marks | $eol)) => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.switch(LexerRule::Inline)
        },

        _ > ($eol | $) => |lexer| {
            let text = lexer.match_().to_owned();
            lexer.return_(Token::Text(text))
        },

        _ => |lexer| lexer.switch(LexerRule::Inline),
    }

    rule Comment {
        ("\r\n" | $eol)+ $,
        "\r\n" | $eol => |lexer| lexer.return_(Token::NewLine),

        "////" '/'* > ($eol | $) => |lexer| {
            let count = lexer.match_().chars().count();
            if lexer.state().opened_comment_block == count {
                lexer.switch_and_return(LexerRule::Init, Token::CommentDelimiter(count))
            } else {
                lexer.reset_match();
                lexer.continue_()
            }
        },

        (_ # $eol)* > ($eol | $) => |lexer| {
            lexer.reset_match();
            lexer.continue_()
        },
    }

    rule Attributes {
        '!' $word_character ($word_character | '-')* ':' > ($eol | $) => |lexer| {
            let text = lexer.match_().trim_matches(&['!', ':']).to_owned();
            lexer.switch_and_return(LexerRule::Init, Token::AttributeEntry(text, true))
        },

        $word_character ($word_character | '-')* ':' > ($eol | $) => |lexer| {
            let text = lexer.match_().trim_matches(':').to_owned();
            lexer.switch_and_return(LexerRule::Init, Token::AttributeEntry(text, false))
        },

        $word_character ($word_character | '-')* ':' ' '+ => |lexer| {
            let text = lexer.match_().trim_end_matches(' ').trim_matches(':').to_owned();
            lexer.switch_and_return(LexerRule::Inline, Token::AttributeEntry(text, false))
        },

        _ => |lexer| lexer.switch(LexerRule::Inline),
    }

    rule Inline {
        ("\r\n" | $eol)+ $,
        "\r\n" | $eol => |lexer| lexer.switch_and_return(LexerRule::Init, Token::NewLine),

        '`' > ([' ' '(' ')'] | $inline_marks | $eol | $) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Code(prev_is_space_like, true))
        },

        '`' => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Code(prev_is_space_like, false))
        },

        '_' > ([' ' '(' ')'] | $inline_marks | $eol | $) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Emphasis(prev_is_space_like, true))
        },

        '_' => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Emphasis(prev_is_space_like, false))
        },

        '*' > ([' ' '(' ')'] | $inline_marks | $eol | $) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Strong(prev_is_space_like, true))
        },

        '*' => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Strong(prev_is_space_like, false))
        },

        '~' => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Subscript)
        },

        '^' => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Superscript)
        },

        '#' > ([' ' '(' ')'] | $inline_marks | $eol | $) => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Mark(prev_is_space_like, true))
        },

        '#' => |lexer| {
            let prev_is_space_like = lexer.state().prev_space_like;
            lexer.state().prev_space_like = true;
            lexer.reset_match();
            lexer.return_(Token::Mark(prev_is_space_like, false))
        },

        (_ # ($inline_marks | $eol))* [' ' '(' ')'] $constrained_marks+ > [' ' '(' ')'] => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.continue_()
        },

        (_ # ($inline_marks | $eol))* (_ # ([' ' '(' ')'] | $constrained_marks | $eol)) $constrained_marks+ > (_ # ([' ' '(' ')'] | $constrained_marks | $eol)) => |lexer| {
            lexer.state().prev_space_like = true;
            lexer.continue_()
        },

        (_ # ($inline_marks | $eol))+ > $inline_marks => |lexer| {
            let text = lexer.match_().to_owned();
            lexer.state().prev_space_like = matches!(text.clone().pop(), Some(' ' | '(' | ')' | '`' | '_' | '*' | '~' | '^' | '#'));
            lexer.return_(Token::Text(text))
        },

        (_ # ($inline_marks | $eol))+ > ($eol | $) => |lexer| {
            let text = lexer.match_().to_owned();
            lexer.state().prev_space_like = true;
            lexer.switch_and_return(LexerRule::Init, Token::Text(text))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_attribute_entry() {
        let input = "= Document Header\n:notitle:\n:description: test document.\n:!showtitle:\n\nprincipal text.\n";
        let expected_output = vec![
            Token::Heading(1),
            Token::Text("Document Header".to_string()),
            Token::NewLine,
            Token::AttributeEntry("notitle".to_string(), false),
            Token::NewLine,
            Token::AttributeEntry("description".to_string(), false),
            Token::Text("test document.".to_string()),
            Token::NewLine,
            Token::AttributeEntry("showtitle".to_string(), true),
            Token::NewLine,
            Token::NewLine,
            Token::Text("principal text.".to_string()),
        ];

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_constrained_mark() {
        let input = "== Heading 2\n\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n";
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
            Token::Subscript,
            Token::Text("subscript".to_string()),
            Token::Subscript,
            Token::Text(" and ".to_string()),
            Token::Superscript,
            Token::Text("superscript".to_string()),
            Token::Superscript,
            Token::Text(" text.".to_string()),
        ];

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_constrained_mark_edge() {
        let input = "== Heading 2\n\nMore **constrain*ed * bold* and _constrain_ed _ italic__ and ` `constrain`ed(`monospace`)` and #constrain#ed highlight# # and ~constrain~ed ~ subscript~ and ^constrain^ed ^ superscript^ text.\n";
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
            Token::Text(")` and ".to_string()),
            Token::Mark(true, false),
            Token::Text("constrain#ed highlight".to_string()),
            Token::Mark(false, true),
            Token::Text(" # and ".to_string()),
            Token::Subscript,
            Token::Text("constrain".to_string()),
            Token::Subscript,
            Token::Text("ed ".to_string()),
            Token::Subscript,
            Token::Text(" subscript".to_string()),
            Token::Subscript,
            Token::Text(" and ".to_string()),
            Token::Superscript,
            Token::Text("constrain".to_string()),
            Token::Superscript,
            Token::Text("ed ".to_string()),
            Token::Superscript,
            Token::Text(" superscript".to_string()),
            Token::Superscript,
            Token::Text(" text.".to_string()),
        ];

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_unordered_lists() {
        let input =
            "== Heading 2\n\n*** Unordered level 3 list item\n* Unordered level 1 list item\n** Unordered level 2 list item\n**** Unordered level 4 list item\n";
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

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_asterisk_edge() {
        let input =
            "== Heading 2\n\n* Unordered level 1 list item\n*\n*****\n***\n\n****strong mark\n";
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
            Token::NewLine,
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::Strong(true, true),
            Token::Strong(true, false),
            Token::Text("strong mark".to_string()),
        ];

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_examples_block_delimiter() {
        let input =
            "== Heading 2\n\n====\n= Block heading 1\n\nMore *bold* and _italic_ text.\n====\n";
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

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_sidebars_block_delimiter() {
        let input =
            "== Heading 2\n\n****\n= Block heading 1\n\nMore *bold* and _italic_ text.\n****\n";
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

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn lexer_comment_out() {
        let input = "// comment\n== Heading 2\n\n/////\nMore *bold* and _italic_ and `monospace` and #highlight# and ~subscript~ and ^superscript^ text.\n////\n/////\n";
        let expected_output = vec![
            Token::Comment,
            Token::NewLine,
            Token::Heading(2),
            Token::Text("Heading 2".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::CommentDelimiter(5),
            Token::NewLine,
            Token::NewLine,
            Token::NewLine,
            Token::CommentDelimiter(5),
        ];

        let lexer = Lexer::new(input);
        let mut tokens = vec![];
        for res in lexer {
            match res {
                Ok((_, token, _)) => tokens.push(token),
                Err(err) => panic!("{err:?}"),
            }
        }

        assert_eq!(tokens, expected_output);
    }
}
