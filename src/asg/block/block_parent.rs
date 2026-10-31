use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Inlines, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum BlockParent {
    Admonition(AdmonitionBlock),
    Example(BlockParentBody),
    Sidebar(BlockParentBody),
    Open(BlockParentBody),
    Quote(BlockParentBody),
}

impl BlockParent {
    fn new_admonition() -> Self {
        Self::Admonition(AdmonitionBlock::new())
    }

    fn new_example() -> Self {
        Self::Example(BlockParentBody::new("====".to_string()))
    }

    fn new_sidebar() -> Self {
        Self::Sidebar(BlockParentBody::new("****".to_string()))
    }

    fn new_open() -> Self {
        Self::Open(BlockParentBody::new("~~~~".to_string()))
    }

    fn new_quote() -> Self {
        Self::Quote(BlockParentBody::new("____".to_string()))
    }

    pub(crate) fn delimiter(&self) -> String {
        let BlockParentBody { delimiter, .. } = match self {
            BlockParent::Admonition(variant) => variant.body(),
            BlockParent::Example(body)
            | BlockParent::Sidebar(body)
            | BlockParent::Open(body)
            | BlockParent::Quote(body) => body.to_owned(),
        };
        delimiter
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "lowercase")]
pub enum AdmonitionBlock {
    Caution(BlockParentBody),
    Important(BlockParentBody),
    Note(BlockParentBody),
    Tip(BlockParentBody),
    Warning(BlockParentBody),
}
impl AdmonitionBlock {
    fn new() -> Self {
        Self::Note(BlockParentBody::new("====".to_string()))
    }

    fn body(&self) -> BlockParentBody {
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
pub struct BlockParentBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: String,
    form: BlockParentForm,
    title: Option<Inlines>,
    blocks: Vec<NonSectionBlockBody>,
    location: Option<Location>,
}
impl BlockParentBody {
    fn new(delimiter: String) -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter,
            form: BlockParentForm::Delimited,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BlockParentForm {
    Delimited,
}

impl Block {
    pub fn new_admonition() -> Self {
        Self::BlockParent(BlockParent::new_admonition())
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
