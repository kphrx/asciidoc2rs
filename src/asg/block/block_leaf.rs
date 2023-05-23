use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Inline, Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum BlockLeaf {
    Listing(BlockLeafBody),
    Literal(BlockLeafBody),
    Paragraph(BlockLeafBody),
    Pass(BlockLeafBody),
    Stem(BlockLeafBody),
    Verse(BlockLeafBody),
}

impl BlockLeaf {
    fn new_listing() -> Self {
        Self::Listing(BlockLeafBody::new())
    }

    fn new_literal() -> Self {
        Self::Literal(BlockLeafBody::new())
    }

    fn new_paragraph(line: &str) -> Self {
        Self::Paragraph(BlockLeafBody::new_text(line))
    }

    fn new_pass() -> Self {
        Self::Pass(BlockLeafBody::new())
    }

    fn new_stem() -> Self {
        Self::Stem(BlockLeafBody::new())
    }

    fn new_verse() -> Self {
        Self::Verse(BlockLeafBody::new())
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Paragraph(body) => {
                body.inlines = Inline::append(body.inlines.clone(), line);

                Ok(())
            }
            _ => Err("not implemented".into()),
        }
    }

    pub(crate) fn delimiter(&self) -> Option<String> {
        match self {
            BlockLeaf::Listing(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
            BlockLeaf::Literal(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
            BlockLeaf::Paragraph(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
            BlockLeaf::Pass(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
            BlockLeaf::Stem(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
            BlockLeaf::Verse(BlockLeafBody { delimiter, .. }) => delimiter.to_owned(),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockLeafBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: Option<String>,
    title: Option<Vec<Inline>>,
    inlines: Vec<Inline>,
    location: Option<Location>,
}

impl BlockLeafBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            inlines: Vec::with_capacity(0),
            location: None,
        }
    }

    fn new_text(line: &str) -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            inlines: Inline::new(line),
            location: None,
        }
    }

    pub(crate) fn inlines(&self) -> Vec<Inline> {
        self.inlines.clone()
    }
}

impl Block {
    fn new_listing() -> Self {
        Self::BlockLeaf(BlockLeaf::new_listing())
    }

    fn new_literal() -> Self {
        Self::BlockLeaf(BlockLeaf::new_literal())
    }

    pub(crate) fn new_paragraph(line: &str) -> Self {
        Self::BlockLeaf(BlockLeaf::new_paragraph(line))
    }

    fn new_pass() -> Self {
        Self::BlockLeaf(BlockLeaf::new_pass())
    }

    fn new_stem() -> Self {
        Self::BlockLeaf(BlockLeaf::new_stem())
    }

    fn new_verse() -> Self {
        Self::BlockLeaf(BlockLeaf::new_verse())
    }
}
