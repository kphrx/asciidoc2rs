pub(crate) mod block;
mod inlines;

use inlines::*;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

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
