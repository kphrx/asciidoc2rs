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

#[derive(Serialize, Deserialize, Debug)]
pub enum SectionBody {
    Block(Block),
    Section(Section),
}
impl SectionBody {
    fn new_block(b: Block) -> Self {
        Self::Block(b)
    }

    fn new_section() -> Self {
        Self::Section(Section::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NonSectionBlockBody {
    Block(Block),
}
impl NonSectionBlockBody {
    fn new(b: Block) -> Self {
        Self::Block(b)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Block {
    BlockParent(BlockParent),
    BlockLeaf(BlockLeaf),
    BlockMacro(BlockMacro),
    BlockBreak(BlockBreak),
    AnyList(AnyList),
}
