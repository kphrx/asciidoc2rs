use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use crate::asg::{Headline, Location, NodeType, SectionBody};

use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum Document {
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
#[derive(Serialize, Debug)]
pub(crate) struct DocumentHeader {
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

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum Section {
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
    pub(crate) fn new() -> Self {
        Self::Section {
            node_type: NodeType::Block,
            title: Headline::new(),
            level: 1,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}
