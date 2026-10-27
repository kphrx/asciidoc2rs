pub mod block;
mod inlines;

pub use inlines::*;

use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
pub struct Location {
    start: LocationBoundary,
    end: LocationBoundary,
}

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
struct LocationBoundary {
    line: usize,
    column: usize,
    source: Option<String>,
}

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
#[nserde(rename_all = "camelCase")]
pub enum NodeType {
    Block,
    Inline,
    String,
}
