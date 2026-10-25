use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use crate::asg::{Location, NodeType};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum Inline {
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

    pub(crate) fn append(mut inlines: Vec<Self>, line: &str) -> Vec<Self> {
        if let Some(Self::Text(last)) = inlines.last_mut() {
            last.value.push_str(line);
            inlines
        } else {
            inlines.push(Self::new_text(line));
            inlines
        }
    }

    fn new_text(value: &str) -> Self {
        Self::Text(InlineLiteral::new(value.to_owned()))
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
