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

pub(crate) enum BlockTree<'line> {
    Simple(Vec<Box<dyn Inline + 'line>>),
    Compound(Vec<Box<dyn Block<'line> + 'line>>),
}

pub(crate) trait Block<'line>: DynClone {
    fn children(&self) -> BlockTree<'line>;
    fn push(&mut self, line: &'line str);
}
clone_trait_object!(Block<'_>);
