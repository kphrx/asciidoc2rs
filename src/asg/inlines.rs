use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use crate::asg::{Location, NodeType};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum Inline {
    Span(InlineParent),
    Text(InlineLiteral),
    Charref(InlineLiteral),
    Raw(InlineLiteral),
}
impl Inline {
    fn new_text(value: &str) -> Self {
        Self::Text(InlineLiteral::new(value.to_owned()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Inlines(Vec<Inline>);
impl Inlines {
    pub fn new() -> Self {
        Self(Vec::with_capacity(0))
    }

    pub fn new_with_line(line: &str) -> Self {
        Self(vec![Inline::new_text(line)])
    }

    pub fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if let Some(Inline::Text(last)) = self.0.last_mut() {
            last.value.push_str(line);
            Ok(())
        } else {
            self.0.push(Inline::new_text(line));
            Ok(())
        }
    }
}

impl Default for Inlines {
    fn default() -> Self {
        Self::new()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineParent {
    #[serde(rename = "type")]
    node_type: NodeType,
    inlines: Vec<Inline>,
    location: Option<Location>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineLiteral {
    #[serde(rename = "type")]
    node_type: NodeType,
    value: String,
    location: Option<Location>,
}
impl InlineLiteral {
    fn new(value: String) -> Self {
        Self {
            node_type: NodeType::String,
            value,
            location: None,
        }
    }
}
