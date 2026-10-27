use nanoserde::{DeJson, SerJson};

use super::Block;
use crate::asg::{Location, NodeType};

#[derive(SerJson, DeJson, Debug, Clone)]
#[nserde(tag = "name", rename_all = "camelCase")]
pub enum BlockBreak {
    Break {
        #[nserde(rename = "type")]
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

#[derive(SerJson, DeJson, Debug, Clone)]
#[nserde(rename_all = "camelCase")]
pub enum BlockBreakVariant {
    Page,
    Thematic,
}

impl Block {
    pub fn new_page_break() -> Self {
        Self::BlockBreak(BlockBreak::new_page())
    }

    pub fn new_thematic_break() -> Self {
        Self::BlockBreak(BlockBreak::new_thematic())
    }
}
