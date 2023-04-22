mod block_break;
mod block_leaf;
mod block_macro;
mod block_parent;
mod document;
mod list;
mod section;

pub(crate) use block_break::*;
pub(crate) use block_leaf::*;
pub(crate) use block_macro::*;
pub(crate) use block_parent::*;
pub(crate) use document::*;
pub(crate) use list::*;
pub(crate) use section::*;

use serde::{Deserialize, Serialize};

use std::error::Error;

enum LineKind {
    Unknown,
    Empty,
    CommentDelimiter(String),
    CommentMarker,
    HeadingMarker {
        level: usize,
        title: String,
    },
    UnorderedListMarker {
        marker: String,
        principal: String,
    },
    OrderedListMarker {
        marker: String,
        principal: String,
    },
    OffsetOrderedListMarker {
        offset: usize,
        principal: String,
    },
    CalloutListMarker {
        marker: String,
        principal: String,
    },
    DescriptionListMarker {
        marker: String,
        term: String,
        principal: Option<String>,
    },
}
impl LineKind {
    fn parse(line: String) -> Self {
        let line = line.trim_end_matches(' ');

        if line == "" {
            return Self::Empty;
        }

        if line.starts_with("//") {
            if !line.starts_with("///") {
                return Self::CommentMarker;
            }

            if line.starts_with("////") && !line.contains(|c: char| c != '/') {
                return Self::CommentDelimiter(line.to_owned());
            }
        }

        if let Some((marker, title)) = line.split_once("= ") {
            if marker == "" || !marker.contains(|c: char| c != '=') {
                return Self::HeadingMarker {
                    level: marker.len(),
                    title: title.to_owned(),
                };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("* ") {
            let mut marker = list_marker.trim_indent().to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            if marker == "" || !marker.contains(|c: char| c != '*') {
                marker.push_str("*");
                return Self::UnorderedListMarker { marker, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("- ") {
            let mut marker = list_marker.trim_indent().to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            // Unknown `-` list marker can be repeated infinitely.
            if marker == "" {
                marker.push_str("-");
                return Self::UnorderedListMarker { marker, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once(". ") {
            let mut marker = list_marker.trim_indent().to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            if marker == "" || !marker.contains(|c: char| c != '.') {
                marker.push_str(".");
                return Self::OrderedListMarker { marker, principal };
            }

            let offset: usize = marker.parse().unwrap_or(0);
            if 0 < offset && offset <= 9 {
                return Self::OffsetOrderedListMarker { offset, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("> ") {
            if let Some(marker) = list_marker.trim_indent().strip_prefix('<').to_owned() {
                let principal = principal.trim_start_matches(' ').to_owned();
                if marker == "." {
                    return Self::CalloutListMarker {
                        marker: "<.>".to_owned(),
                        principal,
                    };
                }

                if let Ok(number) = marker.parse::<usize>() {
                    return Self::CalloutListMarker {
                        marker: format!("<{}>", number),
                        principal,
                    };
                }
            };
        }

        if let Some((dlist_term, primary_text)) = line.split_once(":: ") {
            let dlist_term = dlist_term.trim_indent();
            let term = dlist_term.trim_end_matches(':').to_owned();
            let principal = Some(primary_text.trim_start_matches(' ').to_owned());
            if !term.ends_with(' ') {
                let marker = ":".repeat(dlist_term.len() - term.len() + 2);

                return Self::DescriptionListMarker {
                    marker,
                    term,
                    principal,
                };
            }
        }

        if line.ends_with("::") {
            let dlist_term = line.trim_indent();
            let term = dlist_term.trim_end_matches(':').to_owned();
            let marker = ":".repeat(dlist_term.len() - term.len());

            return Self::DescriptionListMarker {
                marker,
                term,
                principal: None,
            };
        }

        Self::Unknown
    }
}

trait TrimIndent {
    fn trim_indent<'a>(&'a self) -> &'a str;
}
impl TrimIndent for str {
    fn trim_indent<'a>(&'a self) -> &'a str {
        self.trim_start_matches('\t').trim_start_matches(' ')
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SectionBody {
    Block(Block),
    Section(Section),
}
impl SectionBody {
    fn new_block(b: Block) -> Self {
        Self::Block(b)
    }

    fn new_section(level: usize, title: &str) -> Self {
        Self::Section(Section::new(level, title))
    }
}

type NonSectionBlockBody = Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Block {
    BlockParent(BlockParent),
    BlockLeaf(BlockLeaf),
    BlockMacro(BlockMacro),
    BlockBreak(BlockBreak),
    AnyList(AnyList),
}
impl Block {
    fn is_delimited_block(&self) -> bool {
        match self {
            Block::BlockParent(parent) => parent.delimiter().is_some(),
            Block::BlockLeaf(leaf) => leaf.delimiter().is_some(),
            _ => false,
        }
    }

    fn delimiter(&self) -> Option<String> {
        match self {
            Block::BlockParent(parent) => parent.delimiter(),
            Block::BlockLeaf(leaf) => leaf.delimiter(),
            _ => None,
        }
    }

    pub(crate) fn end(&mut self) {}

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::BlockLeaf(leaf) => leaf.push(line),
            Self::AnyList(list) => list.push(line),
            _ => Err("not implemented".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_or_comment_line_kind() {
        assert!(matches!(LineKind::parse("".to_owned()), LineKind::Empty));
        assert!(matches!(
            LineKind::parse("// inline comment".to_owned()),
            LineKind::CommentMarker
        ));
        assert!(
            matches!(LineKind::parse("/////".to_owned()), LineKind::CommentDelimiter(x) if x == "/////")
        );
    }

    #[test]
    fn section_line_kind() {
        assert!(
            matches!(LineKind::parse("= heading level 0".to_owned()), LineKind::HeadingMarker { level, title } if level == 0 && title == "heading level 0")
        );
        assert!(
            matches!(LineKind::parse("== heading level 1".to_owned()), LineKind::HeadingMarker { level, title } if level == 1 && title == "heading level 1")
        );
        assert!(
            matches!(LineKind::parse("=== heading level 2".to_owned()), LineKind::HeadingMarker { level, title } if level == 2 && title == "heading level 2")
        );
        assert!(
            matches!(LineKind::parse("==== heading level 3".to_owned()), LineKind::HeadingMarker { level, title } if level == 3 && title == "heading level 3")
        );
        assert!(
            matches!(LineKind::parse("===== heading level 4".to_owned()), LineKind::HeadingMarker { level, title } if level == 4 && title == "heading level 4")
        );
        assert!(
            matches!(LineKind::parse("====== heading level 5".to_owned()), LineKind::HeadingMarker { level, title } if level == 5 && title == "heading level 5")
        );
        assert!(
            matches!(LineKind::parse("======= heading level 6".to_owned()), LineKind::HeadingMarker { level, title } if level == 6 && title == "heading level 6")
        );
    }

    #[test]
    fn unordered_list_line_kind() {
        assert!(
            matches!(LineKind::parse("** unordered list".to_owned()), LineKind::UnorderedListMarker { marker, principal } if marker == "**" && principal == "unordered list")
        );
        assert!(
            matches!(LineKind::parse("- unordered list".to_owned()), LineKind::UnorderedListMarker { marker, principal } if marker == "-" && principal == "unordered list")
        );
    }

    #[test]
    fn ordered_list_line_kind() {
        assert!(
            matches!(LineKind::parse("\t\t... ordered list".to_owned()), LineKind::OrderedListMarker { marker, principal } if marker == "..." && principal == "ordered list")
        );
        assert!(
            matches!(LineKind::parse("  5. ordered list".to_owned()), LineKind::OffsetOrderedListMarker { offset, principal } if offset == 5 && principal == "ordered list")
        );
    }

    #[test]
    fn dlist_line_kind() {
        assert!(
            matches!(LineKind::parse(" term::".to_owned()), LineKind::DescriptionListMarker { marker, term, principal } if marker == "::" && term == "term" && principal.is_none())
        );
        assert!(
            matches!(LineKind::parse("term:: description list".to_owned()), LineKind::DescriptionListMarker { marker, term, principal } if marker == "::" && term == "term" && principal == Some("description list".to_owned()))
        );
    }

    #[test]
    fn callout_line_kind() {
        assert!(
            matches!(LineKind::parse("<.> callout list".to_owned()), LineKind::CalloutListMarker { marker, principal } if marker == "<.>" && principal == "callout list")
        );
        assert!(
            matches!(LineKind::parse("<3> callout list".to_owned()), LineKind::CalloutListMarker { marker, principal } if marker == "<3>" && principal == "callout list")
        );
    }
}
