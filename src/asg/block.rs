mod block_break;
mod block_leaf;
mod block_macro;
mod block_parent;
mod discrete_heading;
mod document;
mod list;
mod section;

pub use block_break::*;
pub use block_leaf::*;
pub use block_macro::*;
pub use block_parent::*;
pub use discrete_heading::*;
pub use document::*;
pub use list::*;
pub use section::*;

use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SectionBody {
    Block(Block),
    Section(Box<Section>),
}

type NonSectionBlockBody = Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Block {
    Parent(ParentBlock),
    Leaf(LeafBlock),
    Macro(BlockMacro),
    Break(Break),
    Heading(DiscreteHeading),
    AnyList(AnyList),
}
impl Block {
    fn is_delimited_block(&self) -> bool {
        match self {
            Block::Parent(_) => true,
            Block::Leaf(
                LeafBlock::Listing(leaf)
                | LeafBlock::Literal(leaf)
                | LeafBlock::Paragraph(leaf)
                | LeafBlock::Pass(leaf)
                | LeafBlock::Stem(leaf)
                | LeafBlock::Verse(leaf),
            ) if let Some(LeafForm::Delimited(_)) = leaf.form => true,
            _ => false,
        }
    }

    fn delimiter(&self) -> Option<String> {
        match self {
            Block::Parent(parent) => Some(parent.delimiter()),
            Block::Leaf(
                LeafBlock::Listing(leaf)
                | LeafBlock::Literal(leaf)
                | LeafBlock::Paragraph(leaf)
                | LeafBlock::Pass(leaf)
                | LeafBlock::Stem(leaf)
                | LeafBlock::Verse(leaf),
            ) => match leaf.form {
                Some(LeafForm::Delimited(ref delimiter)) => Some(delimiter.to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn end(&mut self) {}

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Leaf(leaf) => leaf.push(line),
            Self::AnyList(list) => list.push(line),
            _ => Err("not implemented".into()),
        }
    }
}
