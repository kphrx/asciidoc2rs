use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Metadata;
use crate::asg::{inline::Inline, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "form", rename_all = "camelCase")]
pub(crate) enum Macro {
    Macro {
        target: Option<String>,
        title: Option<Vec<Inline>>,
        metadata: Option<Metadata>,
        location: Option<Location>,
    },
}

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
