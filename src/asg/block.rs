mod block_break;
mod block_leaf;
mod block_macro;
mod block_parent;
mod document;
mod list;
mod section;

pub(crate) use block_break::*;
pub(crate) use block_leaf::*;
pub(crate) use block_macro::*;
pub(crate) use block_parent::*;
pub(crate) use document::*;
pub(crate) use list::*;
pub(crate) use section::*;

use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SectionBody {
    Block(Block),
    Section(Section),
}
impl SectionBody {
    fn new_block(b: Block) -> Self {
        Self::Block(b)
    }

    fn new_section(level: usize, title: &str) -> Self {
        Self::Section(Section::new(level, title))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NonSectionBlockBody {
    Block(Block),
}
impl NonSectionBlockBody {
    fn new(b: Block) -> Self {
        Self::Block(b)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Block {
    BlockParent(BlockParent),
    BlockLeaf(BlockLeaf),
    BlockMacro(BlockMacro),
    BlockBreak(BlockBreak),
    AnyList(AnyList),
}
impl Block {
    fn is_delimited_block(&self) -> bool {
        match self {
            Block::BlockParent(parent) => parent.delimiter().is_some(),
            Block::BlockLeaf(leaf) => leaf.delimiter().is_some(),
            _ => false,
        }
    }

    fn delimiter(&self) -> Option<String> {
        match self {
            Block::BlockParent(parent) => parent.delimiter(),
            Block::BlockLeaf(leaf) => leaf.delimiter(),
            _ => None,
        }
    }

    pub(crate) fn end(&mut self) {}

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::BlockLeaf(leaf) => leaf.push(line),
            _ => Err("not implemented".into()),
        }
    }
}
