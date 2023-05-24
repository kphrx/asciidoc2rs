use serde::Serialize;

use std::collections::HashMap;
use std::error::Error;

use super::ASG;
use crate::asg::block::Block;
use crate::asg::document::Document as ASGDocument;
use crate::asg::document::Header;
use crate::asg::inline::Inline;
use crate::asg::section::Body as SectionBody;

use crate::Doctype;

#[derive(Serialize, Clone)]
#[serde(into = "ASG")]
pub struct Document {
    doctype: Doctype,
    header_parser: HeaderParser,
    body_started: bool,
    blocks: Vec<SectionBody>,
}
impl From<Document> for ASG {
    fn from(val: Document) -> Self {
        let header = val
            .header_parser
            .title
            .map(|title| Header::new(title, None));
        let attributes = val.header_parser.attributes.or(if header.is_some() {
            Some(HashMap::new())
        } else {
            None
        });
        ASG::Block(Block::Document(ASGDocument::new(
            attributes, header, val.blocks, None,
        )))
    }
}

impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Document {
            doctype,
            header_parser: HeaderParser::new(doctype),
            body_started: false,
            blocks: Vec::with_capacity(0),
        }
    }

    pub(crate) fn end(&mut self) {}

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if !self.body_started {
            match self.header_parser.parse_line(line)? {
                HeaderParseResult::Skip => return Ok(()),
                HeaderParseResult::Ok => return Ok(()),
                HeaderParseResult::End => {
                    self.body_started = true;
                    return Ok(());
                }
                HeaderParseResult::NotHeader => {
                    self.body_started = true;
                }
            }
        }

        Ok(())
    }
}

enum HeaderParseResult {
    Skip,
    Ok,
    End,
    NotHeader,
}

#[derive(Clone)]
struct HeaderParser {
    doctype: Doctype,
    title: Option<Vec<Inline>>,
    attributes: Option<HashMap<String, String>>,
    is_authors_line: bool,
    is_revision_line: bool,
    wrapped_attr: Option<(String, String)>,
}
impl HeaderParser {
    fn new(doctype: Doctype) -> Self {
        Self {
            doctype,
            title: None,
            attributes: None,
            is_authors_line: false,
            is_revision_line: false,
            wrapped_attr: None,
        }
    }

    fn has_header(&self) -> bool {
        self.title.is_some() || self.attributes.is_some()
    }

    fn parse_line(&self, line: &str) -> Result<HeaderParseResult, Box<dyn Error>> {
        if line.is_empty() {
            return if self.has_header() {
                Ok(HeaderParseResult::End)
            } else {
                Ok(HeaderParseResult::Skip)
            };
        }

        if line.starts_with("//") && !line.starts_with("///") {
            return Ok(HeaderParseResult::Skip);
        }

        Ok(HeaderParseResult::NotHeader)
    }
}
