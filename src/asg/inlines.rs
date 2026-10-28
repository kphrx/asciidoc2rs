use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use crate::asg::Location;

use std::error::Error;

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
        if let Some(Inline::String(InlineLiteral::Text(last))) = self.0.last_mut() {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Inline {
    Inline(ParentInline),
    String(InlineLiteral),
}
impl Inline {
    fn new_text(value: &str) -> Self {
        Self::String(InlineLiteral::Text(LiteralValue::new(value.to_owned())))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum ParentInline {
    Span(InlineSpan),
    Ref(InlineRef),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "variant", rename_all = "camelCase")]
pub enum InlineSpan {
    Strong(SpanBody),
    Emphasis(SpanBody),
    Code(SpanBody),
    Mark(SpanBody),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpanBody {
    inlines: Vec<Inline>,
    location: Option<Location>,
    form: SpanForm,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SpanForm {
    Constrained,
    Unconstrained,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "variant", rename_all = "camelCase")]
pub enum InlineRef {
    Link(RefBody),
    Xref(RefBody),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RefBody {
    inlines: Vec<Inline>,
    location: Option<Location>,
    target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum InlineLiteral {
    Text(LiteralValue),
    Charref(LiteralValue),
    Raw(LiteralValue),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiteralValue {
    value: String,
    location: Option<Location>,
}
impl LiteralValue {
    fn new(value: String) -> Self {
        Self {
            value,
            location: None,
        }
    }
}
