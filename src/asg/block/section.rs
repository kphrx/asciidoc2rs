use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::SectionBody;
use crate::asg::{Headline, Location, NodeType};

use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    title: Headline,
    level: usize,
    blocks: Vec<SectionBody>,
    location: Option<Location>,
}
impl Section {
    pub(crate) fn new(level: usize, heading: &str) -> Self {
        Self {
            name: "section".to_owned(),
            node_type: NodeType::Block,
            title: Headline::new(heading),
            level,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        Err("not implemented".into())
    }
}
