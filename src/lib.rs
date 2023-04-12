mod asg;
mod blocks;

use std::error::Error;

use asg::block::Document as ASGDocument;
use blocks::{Block, Document};

use dyn_clone::{clone_trait_object, DynClone};

trait Inline: DynClone {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Inline>>>;
}
clone_trait_object!(Inline);

#[derive(Debug, Clone, Copy)]
pub(crate) enum Doctype {
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

    fn new_with_doctype(text: &'input str, doctype: Doctype) -> Self {
        Self { text, doctype }
    }

    fn parse(self) -> Document<'input> {
        let mut document = Document::new(self.doctype);

        let mut is_comment = false;

        for line in self.text.lines() {
            if line == "////" {
                if is_comment {
                    is_comment = false
                } else {
                    is_comment = true
                }

                continue;
            }
            if is_comment {
                continue;
            }
            if line.starts_with("//") && !line.starts_with("///") {
                continue;
            }

            document.push(line);
        }

        document.close();

        document
    }

    fn parse_inline(self) -> Vec<Box<dyn Inline>> {
        Vec::with_capacity(1)
    }

    pub fn parse_to_asg(self) -> Result<ASGDocument, Box<dyn Error>> {
        let mut doc = ASGDocument::new(self.doctype);
        for line in self.text.lines() {
            doc.push(line.to_owned())?;
        }

        Ok(doc)
    }

    pub fn parse_from_asg(self) -> Result<ASGDocument, Box<dyn Error>> {
        let doc = serde_json::from_str(self.text)?;

        Ok(doc)
    }
}
