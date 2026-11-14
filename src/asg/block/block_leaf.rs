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
    fn new_listing(text: &str) -> Self {
        Self::Listing(LeafBody::new_text(text))
    }

    fn new_delimited_listing(delimiter: String, text: &str) -> Self {
        Self::Listing(LeafBody::new_delimited_text(delimiter, text))
    }

    fn new_literal(text: &str) -> Self {
        Self::Literal(LeafBody::new_text(text))
    }

    fn new_delimited_literal(delimiter: String, text: &str) -> Self {
        Self::Literal(LeafBody::new_delimited_text(delimiter, text))
    }

    fn new_paragraph(text: &str) -> Self {
        Self::Paragraph(LeafBody::new_text(text))
    }

    fn new_pass(text: &str) -> Self {
        Self::Pass(LeafBody::new_text(text))
    }

    fn new_delimited_pass(delimiter: String, text: &str) -> Self {
        Self::Pass(LeafBody::new_delimited_text(delimiter, text))
    }

    fn new_stem(text: &str) -> Self {
        Self::Stem(LeafBody::new_text(text))
    }

    fn new_delimited_stem(delimiter: String, text: &str) -> Self {
        Self::Stem(LeafBody::new_delimited_text(delimiter, text))
    }

    fn new_verse(text: &str) -> Self {
        Self::Verse(LeafBody::new_text(text))
    }

    fn new_delimited_verse(delimiter: String, text: &str) -> Self {
        Self::Verse(LeafBody::new_delimited_text(delimiter, text))
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
    fn new_text(text: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            form: None,
            title: None,
            inlines: Inlines::new_with_text(text),
            location: None,
        }
    }

    fn new_delimited_text(delimiter: String, text: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            form: Some(LeafForm::Delimited(delimiter)),
            title: None,
            inlines: Inlines::new_with_text(text),
            location: None,
        }
    }
}

impl Block {
    pub(crate) fn new_listing(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_listing(text))
    }

    pub(crate) fn new_delimited_listing(delimiter: String, text: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_listing(delimiter, text))
    }

    pub(crate) fn new_literal(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_literal(text))
    }

    pub(crate) fn new_delimited_literal(delimiter: String, text: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_literal(delimiter, text))
    }

    pub(crate) fn new_paragraph(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_paragraph(text))
    }

    pub(crate) fn new_pass(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_pass(text))
    }

    pub(crate) fn new_delimited_pass(delimiter: String, text: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_pass(delimiter, text))
    }

    pub(crate) fn new_stem(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_stem(text))
    }

    pub(crate) fn new_delimited_stem(delimiter: String, text: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_stem(delimiter, text))
    }

    pub(crate) fn new_verse(text: &str) -> Self {
        Self::Leaf(LeafBlock::new_verse(text))
    }

    pub(crate) fn new_delimited_verse(delimiter: String, text: &str) -> Self {
        Self::Leaf(LeafBlock::new_delimited_verse(delimiter, text))
    }
}
