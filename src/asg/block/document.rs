use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::SectionBody;
use crate::asg::{Headline, Location, NodeType};

use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum Document {
    Document {
        #[serde(rename = "type")]
        node_type: NodeType,
        attributes: Option<HashMap<String, String>>,
        header: Option<DocumentHeader>,
        blocks: Vec<SectionBody>,
        location: Option<Location>,
    },
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    title: Headline,
    location: Option<Location>,
}
impl Document {
    pub(crate) fn new() -> Self {
        Self::Document {
            node_type: NodeType::Block,
            attributes: None,
            header: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}
