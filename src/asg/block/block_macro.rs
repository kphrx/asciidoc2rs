use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Block;
use crate::asg::{Inlines, Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum BlockMacro {
    Audio(MacroBody),
    Video(MacroBody),
    Image(MacroBody),
    Toc(MacroBody),
}
impl BlockMacro {
    fn new_audio() -> Self {
        Self::Audio(MacroBody::new())
    }

    fn new_video() -> Self {
        Self::Video(MacroBody::new())
    }

    fn new_image() -> Self {
        Self::Image(MacroBody::new())
    }

    fn new_toc() -> Self {
        Self::Toc(MacroBody::new())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "form", rename = "macro")]
pub struct MacroBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    target: Option<String>,
    title: Option<Inlines>,
    location: Option<Location>,
}
impl MacroBody {
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
        Self::Macro(BlockMacro::new_audio())
    }

    pub fn new_video() -> Self {
        Self::Macro(BlockMacro::new_video())
    }

    pub fn new_image() -> Self {
        Self::Macro(BlockMacro::new_image())
    }

    pub fn new_toc() -> Self {
        Self::Macro(BlockMacro::new_toc())
    }
}
