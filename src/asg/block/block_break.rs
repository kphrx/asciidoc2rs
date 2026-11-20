use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "lowercase")]
pub enum Break {
    Page(BreakInfo),
    Thematic(BreakInfo),
}
impl Break {
    fn new_page() -> Self {
        Self::Page(BreakInfo {
            node_type: NodeType::Block,
            location: None,
        })
    }

    fn new_thematic() -> Self {
        Self::Thematic(BreakInfo {
            node_type: NodeType::Block,
            location: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename = "break")]
pub struct BreakInfo {
    #[serde(rename = "type")]
    node_type: NodeType,
    location: Option<Location>,
}

impl Block {
    pub fn new_page_break() -> Self {
        Self::Break(Break::new_page())
    }

    pub fn new_thematic_break() -> Self {
        Self::Break(Break::new_thematic())
    }
}
