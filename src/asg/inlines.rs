use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use crate::asg::{Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Headline {
    inlines: Vec<Inline>,
    location: Option<Location>,
}
impl Headline {
    pub(crate) fn new(text: &str) -> Self {
        Self {
            inlines: Inline::new(text),
            location: None,
        }
    }

    pub(crate) fn heading(&self) -> Vec<Inline> {
        self.inlines.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum Inline {
    Span(InlineParent),
    Text(InlineLiteral),
    Charref(InlineLiteral),
    Raw(InlineLiteral),
}
impl Inline {
    pub(crate) fn new(line: &str) -> Vec<Self> {
        let mut inlines = Vec::with_capacity(0);
        inlines.push(Inline::new_text(line));

        inlines
    }

    fn new_span() -> Self {
        Self::Span(InlineParent::new())
    }

    fn new_text(value: &str) -> Self {
        Self::Text(InlineLiteral::new(value.to_owned()))
    }

    fn new_charref(value: &str) -> Self {
        Self::Charref(InlineLiteral::new(value.to_owned()))
    }

    fn new_raw(value: &str) -> Self {
        Self::Raw(InlineLiteral::new(value.to_owned()))
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct InlineParent {
    #[serde(rename = "type")]
    node_type: NodeType,
    inlines: Vec<Inline>,
    location: Option<Location>,
}
impl InlineParent {
    fn new() -> Self {
        Self {
            node_type: NodeType::Inline,
            inlines: Vec::with_capacity(0),
            location: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct InlineLiteral {
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
