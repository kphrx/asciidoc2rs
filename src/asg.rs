pub mod block;
mod inlines;
mod non_empty_vec;

use std::num::NonZeroUsize;

pub use inlines::*;
pub use non_empty_vec::NonEmptyVec;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(
    from = "(LocationBoundary, LocationBoundary)",
    into = "(LocationBoundary, LocationBoundary)"
)]
pub struct Location {
    start: LocationBoundary,
    end: LocationBoundary,
}
impl From<(LocationBoundary, LocationBoundary)> for Location {
    fn from(value: (LocationBoundary, LocationBoundary)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}
impl From<Location> for (LocationBoundary, LocationBoundary) {
    fn from(value: Location) -> Self {
        (value.start, value.end)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct LocationBoundary {
    line: NonZeroUsize,
    #[serde(rename = "col")]
    column: usize,
    file: Option<NonEmptyVec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Block,
}
