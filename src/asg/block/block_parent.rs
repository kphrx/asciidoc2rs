use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Inlines, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum ParentBlock {
    Admonition(AdmonitionBlock),
    Example(ParentBody),
    Sidebar(ParentBody),
    Open(ParentBody),
    Quote(ParentBody),
}

impl ParentBlock {
    fn new_caution(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Admonition(AdmonitionBlock::new_caution(delimiter, blocks))
    }

    fn new_important(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Admonition(AdmonitionBlock::new_important(delimiter, blocks))
    }

    fn new_note(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Admonition(AdmonitionBlock::new_note(delimiter, blocks))
    }

    fn new_tip(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Admonition(AdmonitionBlock::new_tip(delimiter, blocks))
    }

    fn new_warning(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Admonition(AdmonitionBlock::new_warning(delimiter, blocks))
    }

    fn new_example(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Example(ParentBody::new(delimiter, blocks))
    }

    fn new_sidebar(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Sidebar(ParentBody::new(delimiter, blocks))
    }

    fn new_open(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Open(ParentBody::new(delimiter, blocks))
    }

    fn new_quote(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Quote(ParentBody::new(delimiter, blocks))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "lowercase")]
pub enum AdmonitionBlock {
    Caution(ParentBody),
    Important(ParentBody),
    Note(ParentBody),
    Tip(ParentBody),
    Warning(ParentBody),
}
impl AdmonitionBlock {
    fn new_caution(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Caution(ParentBody::new(delimiter, blocks))
    }

    fn new_important(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Important(ParentBody::new(delimiter, blocks))
    }

    fn new_note(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Note(ParentBody::new(delimiter, blocks))
    }

    fn new_tip(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Tip(ParentBody::new(delimiter, blocks))
    }

    fn new_warning(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Warning(ParentBody::new(delimiter, blocks))
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "form", rename = "delimited")]
pub struct ParentBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: String,
    title: Option<Inlines>,
    blocks: Vec<NonSectionBlockBody>,
    location: Option<Location>,
}
impl ParentBody {
    fn new(delimiter: String, blocks: Vec<Block>) -> Self {
        Self {
            delimiter,
            blocks,
            ..Default::default()
        }
    }
}

impl Block {
    pub(crate) fn new_caution(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_caution(delimiter, blocks))
    }

    pub(crate) fn new_important(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_important(delimiter, blocks))
    }

    pub(crate) fn new_note(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_note(delimiter, blocks))
    }

    pub(crate) fn new_tip(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_tip(delimiter, blocks))
    }

    pub(crate) fn new_warning(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_warning(delimiter, blocks))
    }

    pub(crate) fn new_example(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_example(delimiter, blocks))
    }

    pub(crate) fn new_sidebar(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_sidebar(delimiter, blocks))
    }

    pub(crate) fn new_open(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_open(delimiter, blocks))
    }

    pub(crate) fn new_quote(delimiter: String, blocks: Vec<Block>) -> Self {
        Self::Parent(ParentBlock::new_quote(delimiter, blocks))
    }
}
