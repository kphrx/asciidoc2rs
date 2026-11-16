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
    fn new_audio(target: String) -> Self {
        Self::Audio(MacroBody::new_with_target(target))
    }

    fn new_video(target: String) -> Self {
        Self::Video(MacroBody::new_with_target(target))
    }

    fn new_image(target: String) -> Self {
        Self::Image(MacroBody::new_with_target(target))
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
    pub(crate) target: Option<String>,
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
    fn new_with_target(target: String) -> Self {
        Self {
            node_type: NodeType::Block,
            target: Some(target),
            title: None,
            location: None,
        }
    }
}

impl Block {
    pub(crate) fn new_audio(target: String) -> Self {
        Self::Macro(BlockMacro::new_audio(target))
    }

    pub(crate) fn new_video(target: String) -> Self {
        Self::Macro(BlockMacro::new_video(target))
    }

    pub(crate) fn new_image(target: String) -> Self {
        Self::Macro(BlockMacro::new_image(target))
    }

    pub(crate) fn new_toc() -> Self {
        Self::Macro(BlockMacro::new_toc())
    }
}
