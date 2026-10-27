use nanoserde::{DeJson, SerJson};

use super::{Block, NonSectionBlockBody};
use crate::asg::{Inline, Location, NodeType};

#[derive(SerJson, DeJson, Debug, Clone)]
#[nserde(tag = "name", rename_all = "camelCase")]
pub enum BlockParent {
    Admonition {
        #[nserde(rename = "type")]
        node_type: NodeType,
        delimiter: Option<String>,
        title: Option<Vec<Inline>>,
        blocks: Vec<NonSectionBlockBody>,
        location: Option<Location>,
        variant: AdmonitionVariant,
    },
    Example(BlockParentBody),
    Sidebar(BlockParentBody),
    Open(BlockParentBody),
    Quote(BlockParentBody),
}
#[derive(SerJson, DeJson, Debug, Clone)]
pub enum AdmonitionVariant {
    Caution,
    Important,
    Note,
    Tip,
    Warning,
}
impl BlockParent {
    fn new_admonition(variant: AdmonitionVariant) -> Self {
        Self::Admonition {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
            variant,
        }
    }

    fn new_example() -> Self {
        Self::Example(BlockParentBody::new())
    }

    fn new_sidebar() -> Self {
        Self::Sidebar(BlockParentBody::new())
    }

    fn new_open() -> Self {
        Self::Open(BlockParentBody::new())
    }

    fn new_quote() -> Self {
        Self::Quote(BlockParentBody::new())
    }

    pub(crate) fn delimiter(&self) -> Option<String> {
        match self {
            BlockParent::Admonition { delimiter, .. } => delimiter.to_owned(),
            BlockParent::Example(BlockParentBody { delimiter, .. }) => delimiter.to_owned(),
            BlockParent::Sidebar(BlockParentBody { delimiter, .. }) => delimiter.to_owned(),
            BlockParent::Open(BlockParentBody { delimiter, .. }) => delimiter.to_owned(),
            BlockParent::Quote(BlockParentBody { delimiter, .. }) => delimiter.to_owned(),
        }
    }
}

#[derive(SerJson, DeJson, Debug, Clone)]
pub struct BlockParentBody {
    #[nserde(rename = "type")]
    node_type: NodeType,
    delimiter: Option<String>,
    title: Option<Vec<Inline>>,
    blocks: Vec<NonSectionBlockBody>,
    location: Option<Location>,
}
impl BlockParentBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}

impl Block {
    pub fn new_admonition(variant: AdmonitionVariant) -> Self {
        Self::BlockParent(BlockParent::new_admonition(variant))
    }

    pub fn new_example() -> Self {
        Self::BlockParent(BlockParent::new_example())
    }

    pub fn new_sidebar() -> Self {
        Self::BlockParent(BlockParent::new_sidebar())
    }

    pub fn new_open() -> Self {
        Self::BlockParent(BlockParent::new_open())
    }

    pub fn new_quote() -> Self {
        Self::BlockParent(BlockParent::new_quote())
    }
}
