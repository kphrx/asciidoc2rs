use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Headline, Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockMacroBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    target: Option<String>,
    title: Option<Headline>,
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
    fn new_audio() -> Self {
        Self::BlockMacro(BlockMacro::new_audio())
    }

    fn new_video() -> Self {
        Self::BlockMacro(BlockMacro::new_video())
    }

    fn new_image() -> Self {
        Self::BlockMacro(BlockMacro::new_image())
    }

    fn new_toc() -> Self {
        Self::BlockMacro(BlockMacro::new_toc())
    }
}
