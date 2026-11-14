use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use crate::asg::Location;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Inlines(pub Vec<Inline>);
impl Inlines {
    pub(crate) fn new_with_text(text: &str) -> Self {
        Self(vec![Inline::new_text(text)])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Inline {
    #[serde(rename = "inline")]
    Parent(ParentInline),
    #[serde(rename = "string")]
    Literal(InlineLiteral),
}
impl Inline {
    fn new_text(value: &str) -> Self {
        Self::Literal(InlineLiteral::Text(LiteralValue::new(value.to_owned())))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum ParentInline {
    Span(InlineSpan),
    Ref(InlineRef),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "lowercase")]
pub enum InlineSpan {
    Strong(SpanBody),
    Emphasis(SpanBody),
    Code(SpanBody),
    Mark(SpanBody),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpanBody {
    inlines: Vec<Inline>,
    location: Option<Location>,
    form: SpanForm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SpanForm {
    Constrained,
    Unconstrained,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", rename_all = "lowercase")]
pub enum InlineRef {
    Link(RefBody),
    Xref(RefBody),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefBody {
    inlines: Vec<Inline>,
    location: Option<Location>,
    target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum InlineLiteral {
    Text(LiteralValue),
    Charref(LiteralValue),
    Raw(LiteralValue),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteralValue {
    pub value: String,
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
