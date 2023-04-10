use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use crate::asg::{Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub(crate) struct Headline {
    inlines: Vec<Inline>,
    location: Option<Location>,
}
impl Headline {
    pub(crate) fn new() -> Self {
        Self {
            inlines: Vec::with_capacity(0),
            location: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum Inline {
    Span(InlineParent),
    Text(InlineLiteral),
    Charref(InlineLiteral),
    Raw(InlineLiteral),
}
impl Inline {
    fn new_span() -> Self {
        Self::Span(InlineParent::new())
    }

    fn new_text(value: String) -> Self {
        Self::Text(InlineLiteral::new(value))
    }

    fn new_charref(value: String) -> Self {
        Self::Charref(InlineLiteral::new(value))
    }

    fn new_raw(value: String) -> Self {
        Self::Raw(InlineLiteral::new(value))
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
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
#[derive(Serialize, Debug)]
pub(crate) struct InlineLiteral {
    #[serde(rename = "type")]
    node_type: NodeType,
    value: String,
    location: Option<Location>,
}
impl InlineLiteral {
    fn new(value: String) -> Self {
        Self {
            node_type: NodeType::Inline,
            value,
            location: None,
        }
    }
}
