use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Metadata;
use crate::asg::{Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "camelCase")]
pub(crate) enum Break {
    Page(BreakBody),
    Thematic(BreakBody),
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BreakBody {
    metadata: Option<Metadata>,
    location: Option<Location>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum BlockBreak {
    Break {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: BlockBreakVariant,
        location: Option<Location>,
    },
}
impl BlockBreak {
    fn new_page() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Page,
            location: None,
        }
    }

    fn new_thematic() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Thematic,
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BlockBreakVariant {
    Page,
    Thematic,
}
