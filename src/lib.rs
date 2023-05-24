#![doc = include_str!("../README.md")]

mod asg;
mod document;

use asg::ASG;
use document::Document;

use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Doctype {
    Article,
    Book,
    Manpage,
}
impl Default for Doctype {
    fn default() -> Self {
        Self::Article
    }
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

    pub fn parse(self) -> Result<Document, Box<dyn Error>> {
        let mut doc = Document::new(self.doctype);
        for line in self.text.lines() {
            doc.push(line)?;
        }
        doc.end();

        Ok(doc)
    }
}
