#![doc = include_str!("../README.md")]

pub mod asg;

use asg::block::Document;
use asg::Inlines;

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

    pub fn parse_to_inlines(self) -> Result<Inlines, Box<dyn Error>> {
        let mut inlines = Inlines::new();
        for line in self.text.lines() {
            inlines.push(line)?;
        }

        Ok(inlines)
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
