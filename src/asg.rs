pub mod block;
mod inlines;

use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;

pub use inlines::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "Vec<T>")]
struct NonEmptyVec<T>(Vec<T>);
impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = EmptySequence;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptySequence)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EmptySequence;
impl fmt::Display for EmptySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("sequence must contain at least one element")
    }
}
impl Error for EmptySequence {}
