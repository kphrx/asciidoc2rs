use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use std::collections::HashMap;

use super::block::Block;
use super::inline::Inline;
use super::section::Body as SectionBody;
use super::Location;
use super::ASG;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "ASG", into = "ASG")]
pub(crate) struct Document {
    attributes: Option<HashMap<String, String>>,
    header: Option<Header>,
    blocks: Vec<SectionBody>,
    location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Header {
    title: Vec<Inline>,
    location: Option<Location>,
}

impl From<ASG> for Document {
    fn from(asg: ASG) -> Document {
        if let ASG::Block(Block::Document(document)) = asg {
            document
        } else {
            panic!()
        }
    }
}

impl From<Document> for ASG {
    fn from(val: Document) -> Self {
        ASG::Block(Block::Document(val))
    }
}
