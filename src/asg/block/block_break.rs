use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum BlockBreak {
    Break {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: BlockBreakVariant,
        location: Option<Location>,
    },
}
impl BlockBreak {
    fn new_page() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Page,
            location: None,
        }
    }

    fn new_thematic() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Thematic,
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BlockBreakVariant {
    Page,
    Thematic,
}

impl Block {
    fn new_page_break() -> Self {
        Self::BlockBreak(BlockBreak::new_page())
    }

    fn new_thematic_break() -> Self {
        Self::BlockBreak(BlockBreak::new_thematic())
    }
}
