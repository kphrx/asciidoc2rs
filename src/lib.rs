#![doc = include_str!("../README.md")]

pub mod asg;

use asg::block::Document;
use asg::Inline;

use std::error::Error;

#[derive(Debug, Default)]
pub enum Doctype {
    #[default]
    Article,
    Book,
    Manpage,
}

pub struct Parser<'input> {
    text: &'input str,
    doctype: Doctype,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        Self::new_with_doctype(text, Default::default())
    }

    pub fn new_with_doctype(text: &'input str, doctype: Doctype) -> Self {
        Self { text, doctype }
    }

    pub fn parse_to_inline(self) -> Vec<Inline> {
        Inline::new(self.text)
    }

    pub fn parse_to_asg(self) -> Result<Document, Box<dyn Error>> {
        let mut doc = Document::new(self.doctype);
        for line in self.text.lines() {
            doc.push(line)?;
        }
        doc.end();

        Ok(doc)
    }

    pub fn parse_from_asg(text: &'input str) -> Result<Document, Box<dyn Error>> {
        let doc = serde_json::from_str(text)?;

        Ok(doc)
    }
}
