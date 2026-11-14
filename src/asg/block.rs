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
