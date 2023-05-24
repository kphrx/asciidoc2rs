use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;

use super::ASG;
use crate::Doctype;
use crate::asg::block::Block;
use crate::asg::document::Document as ASGDocument;
use crate::asg::document::Header as ASGHeader;
use crate::asg::inline::Inline;
use crate::asg::section::Body as SectionBody;
use crate::asg::Location;

#[derive(Serialize, Clone)]
#[serde(into = "ASG")]
pub struct Document {
    doctype: Doctype,
    attributes: Option<HashMap<String, String>>,
    header: Option<Header>,
    blocks: Vec<SectionBody>,
    location: Option<Location>,
}
impl Into<ASG> for Document {
    fn into(self) -> ASG {
        ASG::Block(Block::Document(ASGDocument::new(
            self.attributes,
            self.header.map(Into::into),
            self.blocks,
            self.location,
        )))
    }
}

#[derive(Clone)]
struct Header {
    title: Vec<Inline>,
    location: Option<Location>,
}
impl Into<ASGHeader> for Header {
    fn into(self) -> ASGHeader {
        ASGHeader::new(self.title, self.location)
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

    pub(crate) fn end(&mut self) {
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        Err("Not implemented".into())
    }
}
