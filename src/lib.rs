//! # asciidoc2rs
//! ![TCK passed count](https://img.shields.io/endpoint?style=flat-square&url=https://gist.githubusercontent.com/kphrx/e795d8d23466d049a08e03c23301e996/raw/asciidoc2rs-passed.json)
//! ![TCK failed count](https://img.shields.io/endpoint?style=flat-square&url=https://gist.githubusercontent.com/kphrx/e795d8d23466d049a08e03c23301e996/raw/asciidoc2rs-failed.json)
//!
//! A parser for AsciiDoc

mod asg;

use asg::block::Document;
use asg::Inline;

use std::error::Error;

#[derive(Debug)]
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

    fn parse_inline(self) -> Vec<Inline> {
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

    pub fn parse_from_asg(self) -> Result<Document, Box<dyn Error>> {
        let doc = serde_json::from_str(self.text)?;

        Ok(doc)
    }
}
