use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Inlines, Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum LeafBlock {
    Listing(LeafBody),
    Literal(LeafBody),
    Paragraph(LeafBody),
    Pass(LeafBody),
    Stem(LeafBody),
    Verse(LeafBody),
}

impl LeafBlock {
    fn new_listing() -> Self {
        Self::Listing(LeafBody::new())
    }

    fn new_delimited_listing(delimiter: String, line: &str) -> Self {
        Self::Listing(LeafBody::new_delimited_text(delimiter, line))
    }

    fn new_literal() -> Self {
        Self::Literal(LeafBody::new())
    }

    fn new_delimited_literal(delimiter: String, line: &str) -> Self {
        Self::Literal(LeafBody::new_delimited_text(delimiter, line))
    }

    fn new_paragraph(line: &str) -> Self {
        Self::Paragraph(LeafBody::new_text(line))
    }

    fn new_pass() -> Self {
        Self::Pass(LeafBody::new())
    }

    fn new_delimited_pass(delimiter: String, line: &str) -> Self {
        Self::Pass(LeafBody::new_delimited_text(delimiter, line))
    }

    fn new_stem() -> Self {
        Self::Stem(LeafBody::new())
    }

    fn new_delimited_stem(delimiter: String, line: &str) -> Self {
        Self::Stem(LeafBody::new_delimited_text(delimiter, line))
    }

    fn new_verse() -> Self {
        Self::Verse(LeafBody::new())
    }

    fn new_delimited_verse(delimiter: String, line: &str) -> Self {
        Self::Verse(LeafBody::new_delimited_text(delimiter, line))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "form", content = "delimiter", rename_all = "lowercase")]
pub(crate) enum LeafForm {
    Delimited(String),
    Indented,
    Paragraph,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeafBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(flatten)]
    pub(crate) form: Option<LeafForm>,
    title: Option<Inlines>,
    pub inlines: Inlines,
    location: Option<Location>,
}

impl LeafBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            form: None,
            title: None,
            inlines: Inlines::new(),
            location: None,
        }
    }

    fn new_text(line: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            form: None,
            title: None,
            inlines: Inlines::new_with_line(line),
            location: None,
        }
    }

    fn new_delimited_text(delimiter: String, line: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            form: Some(LeafForm::Delimited(delimiter)),
            title: None,
            inlines: Inlines::new_with_line(line),
            location: None,
        }
    }
}

impl Block {
    pub fn new_listing() -> Self {
        Self::Leaf(LeafBlock::new_listing())
    }

    pub(crate) fn new_delimited_listing(delimiter: String, line: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_listing(delimiter, line))
    }

    pub fn new_literal() -> Self {
        Self::Leaf(LeafBlock::new_literal())
    }

    pub(crate) fn new_delimited_literal(delimiter: String, line: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_literal(delimiter, line))
    }

    pub fn new_paragraph(line: &str) -> Self {
        Self::Leaf(LeafBlock::new_paragraph(line))
    }

    pub fn new_pass() -> Self {
        Self::Leaf(LeafBlock::new_pass())
    }

    pub(crate) fn new_delimited_pass(delimiter: String, line: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_pass(delimiter, line))
    }

    pub fn new_stem() -> Self {
        Self::Leaf(LeafBlock::new_stem())
    }

    pub(crate) fn new_delimited_stem(delimiter: String, line: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_stem(delimiter, line))
    }

    pub fn new_verse() -> Self {
        Self::Leaf(LeafBlock::new_verse())
    }

    pub(crate) fn new_delimited_verse(delimiter: String, line: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_verse(delimiter, line))
    }
}
