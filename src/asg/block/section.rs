use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::SectionBody;
use crate::asg::{Headline, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum Section {
    Section {
        #[serde(rename = "type")]
        node_type: NodeType,
        title: Headline,
        level: usize,
        blocks: Vec<SectionBody>,
        location: Option<Location>,
    },
}
impl Section {
    pub(crate) fn new(level: usize, heading: String) -> Self {
        let title = Headline::new(heading);
        Self::Section {
            node_type: NodeType::Block,
            title,
            level,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}
