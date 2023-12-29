use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::Location;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Inline {
    Inline(Parent),
    String(String),
}
impl Inline {
    pub(crate) fn new(line: &str) -> Vec<Self> {
        let mut inlines = Vec::with_capacity(0);
        inlines.push(Inline::new_text(line));

        inlines
    }

    fn new_text(value: &str) -> Self {
        Self::String(String::new_text(value))
    }

    pub(crate) fn append(mut inlines: Vec<Self>, line: &str) -> Vec<Self> {
        if let Some(Self::String(String::Text(last))) = inlines.last_mut() {
            last.value.push_str(line);
            inlines
        } else {
            inlines.push(Self::new_text(line));
            inlines
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum Parent {
    Span(Span),
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "form", rename_all = "camelCase")]
pub enum Span {
    Constrained(SpanVariant),
    Unonstrained(SpanVariant),
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "variant", rename_all = "camelCase")]
pub enum SpanVariant {
    Strong(ParentBody),
    Emphasis(ParentBody),
    Code(ParentBody),
    Mark(ParentBody),
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParentBody {
    inlines: Vec<Inline>,
    location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum String {
    Text(Literal),
    Charref(Literal),
    Raw(Literal),
}
impl String {
    fn new_text(value: &str) -> Self {
        Self::Text(Literal::new(value))
    }
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Literal {
    value: std::string::String,
    location: Option<Location>,
}
impl Literal {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            location: None,
        }
    }
}
