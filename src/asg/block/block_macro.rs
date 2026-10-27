use nanoserde::{DeJson, SerJson};

use super::Block;
use crate::asg::{Inline, Location, NodeType};

#[derive(SerJson, DeJson, Debug, Clone)]
#[nserde(tag = "name", rename_all = "camelCase")]
pub enum BlockMacro {
    Audio(BlockMacroBody),
    Video(BlockMacroBody),
    Image(BlockMacroBody),
    Toc(BlockMacroBody),
}
impl BlockMacro {
    fn new_audio() -> Self {
        Self::Audio(BlockMacroBody::new())
    }

    fn new_video() -> Self {
        Self::Video(BlockMacroBody::new())
    }

    fn new_image() -> Self {
        Self::Image(BlockMacroBody::new())
    }

    fn new_toc() -> Self {
        Self::Toc(BlockMacroBody::new())
    }
}

#[derive(SerJson, DeJson, Debug, Clone)]
pub struct BlockMacroBody {
    #[nserde(rename = "type")]
    node_type: NodeType,
    target: Option<String>,
    title: Option<Vec<Inline>>,
    location: Option<Location>,
}
impl BlockMacroBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            target: None,
            title: None,
            location: None,
        }
    }
}

impl Block {
    pub fn new_audio() -> Self {
        Self::BlockMacro(BlockMacro::new_audio())
    }

    pub fn new_video() -> Self {
        Self::BlockMacro(BlockMacro::new_video())
    }

    pub fn new_image() -> Self {
        Self::BlockMacro(BlockMacro::new_image())
    }

    pub fn new_toc() -> Self {
        Self::BlockMacro(BlockMacro::new_toc())
    }
}
