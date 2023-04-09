mod document;
mod section;

use super::Inline;

pub(crate) use document::Document;
pub(crate) use section::Section;

use dyn_clone::{clone_trait_object, DynClone};

#[derive(Debug, Clone, Copy)]
pub enum Doctype {
    Article,
    Book,
    Manpage,
}

pub(crate) enum BlockTree {
    Simple(Vec<Box<dyn Inline>>),
    Compound(Vec<Box<dyn Block>>),
}

pub(crate) trait Block: DynClone {
    fn children(&self) -> BlockTree;
    fn push(&mut self, text: &'static str);
}
clone_trait_object!(Block);
