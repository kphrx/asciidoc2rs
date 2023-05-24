pub mod block;
pub(crate) mod document;
pub(crate) mod inline;
pub(crate) mod section;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ASG {
    Block(block::Block),
    Inline(inline::Parent),
    String(inline::String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
    start: LocationBoundary,
    end: LocationBoundary,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct LocationBoundary {
    line: usize,
    column: usize,
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    Block,
    Inline,
    String,
}
