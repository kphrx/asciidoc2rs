use serde::{Deserialize, Serialize};

use crate::asg::{Inlines, Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename = "heading")]
pub struct DiscreteHeading {
    #[serde(rename = "type")]
    node_type: NodeType,
    level: usize,
    title: Inlines,
    location: Option<Location>,
}
impl DiscreteHeading {
    pub(crate) fn new(level: usize, heading: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            title: Inlines::new_with_line(heading),
            level,
            location: None,
        }
    }
}
