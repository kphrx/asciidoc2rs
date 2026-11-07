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
    fn new_admonition() -> Self {
        Self::Admonition(AdmonitionBlock::new())
    }

    fn new_example() -> Self {
        Self::Example(ParentBody::new("====".to_string()))
    }

    fn new_sidebar() -> Self {
        Self::Sidebar(ParentBody::new("****".to_string()))
    }

    fn new_open() -> Self {
        Self::Open(ParentBody::new("~~~~".to_string()))
    }

    fn new_quote() -> Self {
        Self::Quote(ParentBody::new("____".to_string()))
    }

    pub(crate) fn delimiter(&self) -> String {
        let ParentBody { delimiter, .. } = match self {
            ParentBlock::Admonition(variant) => variant.body(),
            ParentBlock::Example(body)
            | ParentBlock::Sidebar(body)
            | ParentBlock::Open(body)
            | ParentBlock::Quote(body) => body.to_owned(),
        };
        delimiter
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
    fn new() -> Self {
        Self::Note(ParentBody::new("====".to_string()))
    }

    fn body(&self) -> ParentBody {
        let (Self::Caution(body)
        | Self::Important(body)
        | Self::Note(body)
        | Self::Tip(body)
        | Self::Warning(body)) = self;
        body.to_owned()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn new(delimiter: String) -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}

impl Block {
    pub fn new_admonition() -> Self {
        Self::Parent(ParentBlock::new_admonition())
    }

    pub fn new_example() -> Self {
        Self::Parent(ParentBlock::new_example())
    }

    pub fn new_sidebar() -> Self {
        Self::Parent(ParentBlock::new_sidebar())
    }

    pub fn new_open() -> Self {
        Self::Parent(ParentBlock::new_open())
    }

    pub fn new_quote() -> Self {
        Self::Parent(ParentBlock::new_quote())
    }
}
