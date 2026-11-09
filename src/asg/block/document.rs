use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::SectionBody;
use crate::asg::{Inlines, Location, NodeType};

use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "name", rename = "document")]
pub struct Document {
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(flatten)]
    pub header: Option<DocumentHeader>,
    #[serde(default)]
    pub blocks: Vec<SectionBody>,
    location: Option<Location>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DocumentHeader {
    pub header: Option<TitleAndAuthors>,
    pub attributes: HashMap<String, Option<String>>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TitleAndAuthors {
    pub title: Option<Inlines>,
    pub authors: Option<Vec<Author>>,
    pub(crate) location: Option<Location>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Author {
    pub fullname: Option<String>,
    pub(crate) initials: Option<String>,
    pub(crate) firstname: Option<String>,
    pub(crate) middlename: Option<String>,
    pub(crate) lastname: Option<String>,
    pub address: Option<String>,
}
