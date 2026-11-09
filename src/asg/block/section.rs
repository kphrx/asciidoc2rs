use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::SectionBody;
use crate::asg::{Inlines, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename = "section")]
pub struct Section {
    #[serde(rename = "type")]
    node_type: NodeType,
    pub title: Inlines,
    pub(crate) level: usize,
    pub blocks: Vec<SectionBody>,
    location: Option<Location>,
}
impl Section {
    pub(crate) fn new_with_blocks(level: usize, heading: &str, blocks: Vec<SectionBody>) -> Self {
        Self {
            node_type: NodeType::Block,
            title: Inlines::new_with_line(heading),
            level,
            blocks,
            location: None,
        }
    }
}
