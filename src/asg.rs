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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    #[default]
    Block,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn serialize_location() {
        assert_eq!(
            serde_json::to_string(&Location {
                start: LocationBoundary {
                    line: NonZeroUsize::try_from(2).unwrap(),
                    column: 0,
                    file: None,
                },
                end: LocationBoundary {
                    line: NonZeroUsize::try_from(3).unwrap(),
                    column: 5,
                    file: None,
                },
            })
            .unwrap(),
            r#"[{"line":2,"col":0},{"line":3,"col":5}]"#
        );
    }

    #[test]
    fn deserialize_location() {
        assert_matches!(
            serde_json::from_str(r#"[{"line":2,"col":0},{"line":3,"col":5}]"#).unwrap(),
            Location {
                start: LocationBoundary { line: start_line, column: 0, file: None },
                end: LocationBoundary { line: end_line, column: 5, file: None },
            } if start_line.get() == 2 && end_line.get() == 3
        );
    }
}
