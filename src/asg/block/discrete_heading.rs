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
