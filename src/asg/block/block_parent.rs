use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Headline, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum BlockParent {
    Admonition {
        #[serde(rename = "type")]
        node_type: NodeType,
        delimiter: Option<String>,
        title: Option<Headline>,
        blocks: Vec<NonSectionBlockBody>,
        location: Option<Location>,
        variant: AdmonitionVariant,
    },
    Example(BlockParentBody),
    Sidebar(BlockParentBody),
    Open(BlockParentBody),
    Quote(BlockParentBody),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockParentBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: Option<String>,
    title: Option<Headline>,
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
    fn new_admonition(variant: AdmonitionVariant) -> Self {
        Self::BlockParent(BlockParent::new_admonition(variant))
    }

    fn new_example() -> Self {
        Self::BlockParent(BlockParent::new_example())
    }

    fn new_sidebar() -> Self {
        Self::BlockParent(BlockParent::new_sidebar())
    }

    fn new_open() -> Self {
        Self::BlockParent(BlockParent::new_open())
    }

    fn new_quote() -> Self {
        Self::BlockParent(BlockParent::new_quote())
    }
}
