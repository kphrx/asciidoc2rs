mod document;
mod section;

use dyn_clone::{clone_trait_object, DynClone};

use super::Inline;

pub(crate) use document::Document;
use section::Section;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Doctype {
    Article,
    Book,
    Manpage,
}

pub(crate) enum BlockTree<'line> {
    Simple(Vec<Box<dyn Inline + 'line>>),
    Compound(Vec<Box<dyn Block<'line> + 'line>>),
}

pub(crate) trait Block<'line>: DynClone {
    fn children(&self) -> BlockTree<'line>;
    fn push(&mut self, line: &'line str);
    fn close(&mut self);
}
clone_trait_object!(Block<'_>);
