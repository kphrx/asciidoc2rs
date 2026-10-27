use nanoserde::{DeJson, SerJson};

use crate::asg::{Location, NodeType};

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
#[nserde(tag = "name", rename_all = "camelCase")]
pub enum Inline {
    Span(InlineParent),
    Text(InlineLiteral),
    Charref(InlineLiteral),
    Raw(InlineLiteral),
}
impl Inline {
    pub(crate) fn new(line: &str) -> Vec<Self> {
        vec![Inline::new_text(line)]
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

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
pub struct InlineParent {
    #[nserde(rename = "type")]
    node_type: NodeType,
    inlines: Vec<Inline>,
    location: Option<Location>,
}

#[derive(SerJson, DeJson, Debug, Clone, PartialEq)]
pub struct InlineLiteral {
    #[nserde(rename = "type")]
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
