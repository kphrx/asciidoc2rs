use serde::Serialize;

use std::collections::HashMap;
use std::error::Error;

use super::ASG;
use crate::asg::block::Block;
use crate::asg::document::Document as ASGDocument;
use crate::asg::document::Header as ASGHeader;
use crate::asg::inline::Inline;
use crate::asg::section::Body as SectionBody;
use crate::asg::Location;
use crate::Doctype;

#[derive(Serialize, Clone)]
#[serde(into = "ASG")]
pub struct Document {
    doctype: Doctype,
    attributes: Option<HashMap<String, String>>,
    header: Option<Header>,
    blocks: Vec<SectionBody>,
    location: Option<Location>,
}
impl From<Document> for ASG {
    fn from(val: Document) -> Self {
        ASG::Block(Block::Document(ASGDocument::new(
            val.attributes,
            val.header.map(Into::into),
            val.blocks,
            val.location,
        )))
    }
}

#[derive(Clone)]
struct Header {
    title: Vec<Inline>,
    location: Option<Location>,
}
impl From<Header> for ASGHeader {
    fn from(val: Header) -> Self {
        ASGHeader::new(val.title, val.location)
    }
}

impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Document {
            doctype,
            attributes: None,
            header: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }

    pub(crate) fn end(&mut self) {}

    pub(crate) fn push(&mut self, _line: &str) -> Result<(), Box<dyn Error>> {
        Err("Not implemented".into())
    }
}
