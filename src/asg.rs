mod blocks;
mod inlines;
mod section;

use blocks::*;
use inlines::*;
pub(crate) use section::*;

use serde::{Serialize, Deserialize};
use serde_with_macros::skip_serializing_none;

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
pub struct Location {
    start: LocationBoundary,
    end: LocationBoundary,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct LocationBoundary {
    line: usize,
    column: usize,
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    Block,
    Inline,
    String,
}
