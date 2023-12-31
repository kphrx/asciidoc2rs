#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = "AbstractBlock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"type\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AbstractBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: AbstractBlockType,
}
impl From<&AbstractBlock> for AbstractBlock {
    fn from(value: &AbstractBlock) -> Self {
        value.clone()
    }
}
impl AbstractBlock {
    pub fn builder() -> builder::AbstractBlock {
        Default::default()
    }
}
#[doc = "AbstractBlockType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AbstractBlockType {
    #[serde(rename = "block")]
    Block,
}
impl From<&AbstractBlockType> for AbstractBlockType {
    fn from(value: &AbstractBlockType) -> Self {
        value.clone()
    }
}
impl ToString for AbstractBlockType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for AbstractBlockType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AbstractBlockType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AbstractBlockType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AbstractBlockType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AbstractHeading"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"level\",\n    \"title\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"level\": {\n      \"type\": \"integer\",\n      \"minimum\": 0.0\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AbstractHeading {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub level: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    pub title: Inlines,
    #[serde(rename = "type")]
    pub type_: AbstractHeadingType,
}
impl From<&AbstractHeading> for AbstractHeading {
    fn from(value: &AbstractHeading) -> Self {
        value.clone()
    }
}
impl AbstractHeading {
    pub fn builder() -> builder::AbstractHeading {
        Default::default()
    }
}
#[doc = "AbstractHeadingType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AbstractHeadingType {
    #[serde(rename = "block")]
    Block,
}
impl From<&AbstractHeadingType> for AbstractHeadingType {
    fn from(value: &AbstractHeadingType) -> Self {
        value.clone()
    }
}
impl ToString for AbstractHeadingType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for AbstractHeadingType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AbstractHeadingType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AbstractHeadingType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AbstractHeadingType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AbstractListItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"marker\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"blocks\": {\n      \"$ref\": \"#/$defs/nonSectionBlockBody\"\n    },\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"marker\": {\n      \"type\": \"string\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"principal\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AbstractListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<NonSectionBlockBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub marker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: AbstractListItemType,
}
impl From<&AbstractListItem> for AbstractListItem {
    fn from(value: &AbstractListItem) -> Self {
        value.clone()
    }
}
impl AbstractListItem {
    pub fn builder() -> builder::AbstractListItem {
        Default::default()
    }
}
#[doc = "AbstractListItemType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AbstractListItemType {
    #[serde(rename = "block")]
    Block,
}
impl From<&AbstractListItemType> for AbstractListItemType {
    fn from(value: &AbstractListItemType) -> Self {
        value.clone()
    }
}
impl ToString for AbstractListItemType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for AbstractListItemType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AbstractListItemType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AbstractListItemType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AbstractListItemType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AbstractParentInline"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"inlines\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"inlines\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"inline\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AbstractParentInline {
    pub inlines: Inlines,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "type")]
    pub type_: AbstractParentInlineType,
}
impl From<&AbstractParentInline> for AbstractParentInline {
    fn from(value: &AbstractParentInline) -> Self {
        value.clone()
    }
}
impl AbstractParentInline {
    pub fn builder() -> builder::AbstractParentInline {
        Default::default()
    }
}
#[doc = "AbstractParentInlineType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"inline\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AbstractParentInlineType {
    #[serde(rename = "inline")]
    Inline,
}
impl From<&AbstractParentInlineType> for AbstractParentInlineType {
    fn from(value: &AbstractParentInlineType) -> Self {
        value.clone()
    }
}
impl ToString for AbstractParentInlineType {
    fn to_string(&self) -> String {
        match *self {
            Self::Inline => "inline".to_string(),
        }
    }
}
impl std::str::FromStr for AbstractParentInlineType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AbstractParentInlineType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AbstractParentInlineType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AbstractParentInlineType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "A structured representation of the semantics in an AsciiDoc document, primarily used for validating the compliance of an AsciiDoc processor."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"$id\": \"https://schemas.asciidoc.org/asg/1-0-0/draft-01\",\n  \"title\": \"AsciiDoc Abstract Semantic Graph (ASG)\",\n  \"description\": \"A structured representation of the semantics in an AsciiDoc document, primarily used for validating the compliance of an AsciiDoc processor.\",\n  \"oneOf\": [\n    {\n      \"$id\": \"https://schemas.asciidoc.org/asg/1-0-0/draft-01\",\n      \"title\": \"AsciiDoc Abstract Semantic Graph (ASG)\",\n      \"description\": \"A structured representation of the semantics in an AsciiDoc document, primarily used for validating the compliance of an AsciiDoc processor.\",\n      \"type\": \"object\",\n      \"required\": [\n        \"name\",\n        \"type\"\n      ],\n      \"properties\": {\n        \"attributes\": {\n          \"type\": \"object\",\n          \"additionalProperties\": {\n            \"oneOf\": [\n              {\n                \"type\": \"string\"\n              },\n              {\n                \"type\": \"null\"\n              }\n            ]\n          }\n        },\n        \"blocks\": {\n          \"$ref\": \"#/$defs/sectionBody\"\n        },\n        \"header\": {\n          \"type\": \"object\",\n          \"properties\": {\n            \"authors\": {\n              \"type\": \"array\",\n              \"items\": {\n                \"$ref\": \"#/$defs/author\"\n              },\n              \"minItems\": 1\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"title\": {\n              \"$ref\": \"#/$defs/inlines\"\n            }\n          },\n          \"additionalProperties\": false\n        },\n        \"location\": {\n          \"$ref\": \"#/$defs/location\"\n        },\n        \"name\": {\n          \"type\": \"string\",\n          \"enum\": [\n            \"document\"\n          ]\n        },\n        \"type\": {\n          \"type\": \"string\",\n          \"enum\": [\n            \"block\"\n          ]\n        }\n      },\n      \"additionalProperties\": false,\n      \"defaults\": {\n        \"blocks\": []\n      }\n    },\n    {\n      \"allOf\": [\n        {\n          \"$id\": \"https://schemas.asciidoc.org/asg/1-0-0/draft-01\",\n          \"title\": \"AsciiDoc Abstract Semantic Graph (ASG)\",\n          \"description\": \"A structured representation of the semantics in an AsciiDoc document, primarily used for validating the compliance of an AsciiDoc processor.\",\n          \"type\": \"object\",\n          \"required\": [\n            \"name\",\n            \"type\"\n          ],\n          \"properties\": {\n            \"attributes\": {\n              \"type\": \"object\",\n              \"additionalProperties\": {\n                \"oneOf\": [\n                  {\n                    \"type\": \"string\"\n                  },\n                  {\n                    \"type\": \"null\"\n                  }\n                ]\n              }\n            },\n            \"blocks\": {\n              \"$ref\": \"#/$defs/sectionBody\"\n            },\n            \"header\": {\n              \"type\": \"object\",\n              \"properties\": {\n                \"authors\": {\n                  \"type\": \"array\",\n                  \"items\": {\n                    \"$ref\": \"#/$defs/author\"\n                  },\n                  \"minItems\": 1\n                },\n                \"location\": {\n                  \"$ref\": \"#/$defs/location\"\n                },\n                \"title\": {\n                  \"$ref\": \"#/$defs/inlines\"\n                }\n              },\n              \"additionalProperties\": false\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"name\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"document\"\n              ]\n            },\n            \"type\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"block\"\n              ]\n            }\n          },\n          \"additionalProperties\": false,\n          \"defaults\": {\n            \"blocks\": []\n          }\n        },\n        {\n          \"required\": [\n            \"attributes\",\n            \"header\",\n            \"name\",\n            \"type\"\n          ]\n        }\n      ]\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum AsciiDocAbstractSemanticGraphAsg {
    Variant0 {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        attributes: std::collections::HashMap<String, Option<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<SectionBody>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        header: Option<AsciiDocAbstractSemanticGraphAsgHeader>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        name: AsciiDocAbstractSemanticGraphAsgName,
        #[serde(rename = "type")]
        type_: AsciiDocAbstractSemanticGraphAsgType,
    },
    Variant1 {
        attributes: std::collections::HashMap<String, Option<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<SectionBody>,
        header: AsciiDocAbstractSemanticGraphAsgVariant1Header,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        name: AsciiDocAbstractSemanticGraphAsgVariant1Name,
        #[serde(rename = "type")]
        type_: AsciiDocAbstractSemanticGraphAsgVariant1Type,
    },
}
impl From<&AsciiDocAbstractSemanticGraphAsg> for AsciiDocAbstractSemanticGraphAsg {
    fn from(value: &AsciiDocAbstractSemanticGraphAsg) -> Self {
        value.clone()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgHeader"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"authors\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/author\"\n      },\n      \"minItems\": 1\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    }\n  },\n  \"additionalProperties\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AsciiDocAbstractSemanticGraphAsgHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
}
impl From<&AsciiDocAbstractSemanticGraphAsgHeader> for AsciiDocAbstractSemanticGraphAsgHeader {
    fn from(value: &AsciiDocAbstractSemanticGraphAsgHeader) -> Self {
        value.clone()
    }
}
impl AsciiDocAbstractSemanticGraphAsgHeader {
    pub fn builder() -> builder::AsciiDocAbstractSemanticGraphAsgHeader {
        Default::default()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"document\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsciiDocAbstractSemanticGraphAsgName {
    #[serde(rename = "document")]
    Document,
}
impl From<&AsciiDocAbstractSemanticGraphAsgName> for AsciiDocAbstractSemanticGraphAsgName {
    fn from(value: &AsciiDocAbstractSemanticGraphAsgName) -> Self {
        value.clone()
    }
}
impl ToString for AsciiDocAbstractSemanticGraphAsgName {
    fn to_string(&self) -> String {
        match *self {
            Self::Document => "document".to_string(),
        }
    }
}
impl std::str::FromStr for AsciiDocAbstractSemanticGraphAsgName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "document" => Ok(Self::Document),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AsciiDocAbstractSemanticGraphAsgName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AsciiDocAbstractSemanticGraphAsgName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AsciiDocAbstractSemanticGraphAsgName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsciiDocAbstractSemanticGraphAsgType {
    #[serde(rename = "block")]
    Block,
}
impl From<&AsciiDocAbstractSemanticGraphAsgType> for AsciiDocAbstractSemanticGraphAsgType {
    fn from(value: &AsciiDocAbstractSemanticGraphAsgType) -> Self {
        value.clone()
    }
}
impl ToString for AsciiDocAbstractSemanticGraphAsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for AsciiDocAbstractSemanticGraphAsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AsciiDocAbstractSemanticGraphAsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AsciiDocAbstractSemanticGraphAsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AsciiDocAbstractSemanticGraphAsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgVariant1Header"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"authors\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/author\"\n      },\n      \"minItems\": 1\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    }\n  },\n  \"additionalProperties\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AsciiDocAbstractSemanticGraphAsgVariant1Header {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
}
impl From<&AsciiDocAbstractSemanticGraphAsgVariant1Header>
    for AsciiDocAbstractSemanticGraphAsgVariant1Header
{
    fn from(value: &AsciiDocAbstractSemanticGraphAsgVariant1Header) -> Self {
        value.clone()
    }
}
impl AsciiDocAbstractSemanticGraphAsgVariant1Header {
    pub fn builder() -> builder::AsciiDocAbstractSemanticGraphAsgVariant1Header {
        Default::default()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgVariant1Name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"document\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsciiDocAbstractSemanticGraphAsgVariant1Name {
    #[serde(rename = "document")]
    Document,
}
impl From<&AsciiDocAbstractSemanticGraphAsgVariant1Name>
    for AsciiDocAbstractSemanticGraphAsgVariant1Name
{
    fn from(value: &AsciiDocAbstractSemanticGraphAsgVariant1Name) -> Self {
        value.clone()
    }
}
impl ToString for AsciiDocAbstractSemanticGraphAsgVariant1Name {
    fn to_string(&self) -> String {
        match *self {
            Self::Document => "document".to_string(),
        }
    }
}
impl std::str::FromStr for AsciiDocAbstractSemanticGraphAsgVariant1Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "document" => Ok(Self::Document),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AsciiDocAbstractSemanticGraphAsgVariant1Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AsciiDocAbstractSemanticGraphAsgVariant1Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AsciiDocAbstractSemanticGraphAsgVariant1Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "AsciiDocAbstractSemanticGraphAsgVariant1Type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsciiDocAbstractSemanticGraphAsgVariant1Type {
    #[serde(rename = "block")]
    Block,
}
impl From<&AsciiDocAbstractSemanticGraphAsgVariant1Type>
    for AsciiDocAbstractSemanticGraphAsgVariant1Type
{
    fn from(value: &AsciiDocAbstractSemanticGraphAsgVariant1Type) -> Self {
        value.clone()
    }
}
impl ToString for AsciiDocAbstractSemanticGraphAsgVariant1Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for AsciiDocAbstractSemanticGraphAsgVariant1Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AsciiDocAbstractSemanticGraphAsgVariant1Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AsciiDocAbstractSemanticGraphAsgVariant1Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AsciiDocAbstractSemanticGraphAsgVariant1Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Author"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"address\": {\n      \"type\": \"string\"\n    },\n    \"firstname\": {\n      \"type\": \"string\"\n    },\n    \"fullname\": {\n      \"type\": \"string\"\n    },\n    \"initials\": {\n      \"type\": \"string\"\n    },\n    \"lastname\": {\n      \"type\": \"string\"\n    },\n    \"middlename\": {\n      \"type\": \"string\"\n    }\n  },\n  \"additionalProperties\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fullname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middlename: Option<String>,
}
impl From<&Author> for Author {
    fn from(value: &Author) -> Self {
        value.clone()
    }
}
impl Author {
    pub fn builder() -> builder::Author {
        Default::default()
    }
}
#[doc = "Block"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"oneOf\": [\n    {\n      \"$ref\": \"#/$defs/list\"\n    },\n    {\n      \"$ref\": \"#/$defs/dlist\"\n    },\n    {\n      \"$ref\": \"#/$defs/discreteHeading\"\n    },\n    {\n      \"$ref\": \"#/$defs/break\"\n    },\n    {\n      \"$ref\": \"#/$defs/blockMacro\"\n    },\n    {\n      \"$ref\": \"#/$defs/leafBlock\"\n    },\n    {\n      \"$ref\": \"#/$defs/parentBlock\"\n    }\n  ],\n  \"discriminator\": {\n    \"propertyName\": \"name\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Block {
    List(List),
    Dlist(Dlist),
    DiscreteHeading(DiscreteHeading),
    Break(Break),
    BlockMacro(BlockMacro),
    LeafBlock(LeafBlock),
    ParentBlock(ParentBlock),
}
impl From<&Block> for Block {
    fn from(value: &Block) -> Self {
        value.clone()
    }
}
impl From<List> for Block {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}
impl From<Dlist> for Block {
    fn from(value: Dlist) -> Self {
        Self::Dlist(value)
    }
}
impl From<DiscreteHeading> for Block {
    fn from(value: DiscreteHeading) -> Self {
        Self::DiscreteHeading(value)
    }
}
impl From<Break> for Block {
    fn from(value: Break) -> Self {
        Self::Break(value)
    }
}
impl From<BlockMacro> for Block {
    fn from(value: BlockMacro) -> Self {
        Self::BlockMacro(value)
    }
}
impl From<LeafBlock> for Block {
    fn from(value: LeafBlock) -> Self {
        Self::LeafBlock(value)
    }
}
impl From<ParentBlock> for Block {
    fn from(value: ParentBlock) -> Self {
        Self::ParentBlock(value)
    }
}
#[doc = "BlockMacro"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"form\",\n    \"name\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"form\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"macro\"\n      ]\n    },\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"audio\",\n        \"video\",\n        \"image\",\n        \"toc\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"target\": {\n      \"type\": \"string\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMacro {
    pub form: BlockMacroForm,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: BlockMacroName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: BlockMacroType,
}
impl From<&BlockMacro> for BlockMacro {
    fn from(value: &BlockMacro) -> Self {
        value.clone()
    }
}
impl BlockMacro {
    pub fn builder() -> builder::BlockMacro {
        Default::default()
    }
}
#[doc = "BlockMacroForm"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"macro\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BlockMacroForm {
    #[serde(rename = "macro")]
    Macro,
}
impl From<&BlockMacroForm> for BlockMacroForm {
    fn from(value: &BlockMacroForm) -> Self {
        value.clone()
    }
}
impl ToString for BlockMacroForm {
    fn to_string(&self) -> String {
        match *self {
            Self::Macro => "macro".to_string(),
        }
    }
}
impl std::str::FromStr for BlockMacroForm {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "macro" => Ok(Self::Macro),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BlockMacroForm {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BlockMacroForm {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BlockMacroForm {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "BlockMacroName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"audio\",\n    \"video\",\n    \"image\",\n    \"toc\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BlockMacroName {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "toc")]
    Toc,
}
impl From<&BlockMacroName> for BlockMacroName {
    fn from(value: &BlockMacroName) -> Self {
        value.clone()
    }
}
impl ToString for BlockMacroName {
    fn to_string(&self) -> String {
        match *self {
            Self::Audio => "audio".to_string(),
            Self::Video => "video".to_string(),
            Self::Image => "image".to_string(),
            Self::Toc => "toc".to_string(),
        }
    }
}
impl std::str::FromStr for BlockMacroName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            "image" => Ok(Self::Image),
            "toc" => Ok(Self::Toc),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BlockMacroName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BlockMacroName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BlockMacroName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "BlockMacroType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BlockMacroType {
    #[serde(rename = "block")]
    Block,
}
impl From<&BlockMacroType> for BlockMacroType {
    fn from(value: &BlockMacroType) -> Self {
        value.clone()
    }
}
impl ToString for BlockMacroType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for BlockMacroType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BlockMacroType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BlockMacroType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BlockMacroType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "BlockMetadata"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"attributes\": {\n      \"type\": \"object\",\n      \"additionalProperties\": {\n        \"oneOf\": [\n          {\n            \"type\": \"string\"\n          }\n        ]\n      }\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"options\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\"\n      }\n    },\n    \"roles\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\"\n      }\n    }\n  },\n  \"additionalProperties\": false,\n  \"defaults\": {\n    \"attributes\": {},\n    \"options\": [],\n    \"roles\": []\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockMetadata {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
impl From<&BlockMetadata> for BlockMetadata {
    fn from(value: &BlockMetadata) -> Self {
        value.clone()
    }
}
impl BlockMetadata {
    pub fn builder() -> builder::BlockMetadata {
        Default::default()
    }
}
#[doc = "Break"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"name\",\n    \"type\",\n    \"variant\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"break\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    },\n    \"variant\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"page\",\n        \"thematic\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Break {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: BreakName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: BreakType,
    pub variant: BreakVariant,
}
impl From<&Break> for Break {
    fn from(value: &Break) -> Self {
        value.clone()
    }
}
impl Break {
    pub fn builder() -> builder::Break {
        Default::default()
    }
}
#[doc = "BreakName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"break\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BreakName {
    #[serde(rename = "break")]
    Break,
}
impl From<&BreakName> for BreakName {
    fn from(value: &BreakName) -> Self {
        value.clone()
    }
}
impl ToString for BreakName {
    fn to_string(&self) -> String {
        match *self {
            Self::Break => "break".to_string(),
        }
    }
}
impl std::str::FromStr for BreakName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "break" => Ok(Self::Break),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BreakName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BreakName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BreakName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "BreakType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BreakType {
    #[serde(rename = "block")]
    Block,
}
impl From<&BreakType> for BreakType {
    fn from(value: &BreakType) -> Self {
        value.clone()
    }
}
impl ToString for BreakType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for BreakType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BreakType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BreakType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BreakType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "BreakVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"page\",\n    \"thematic\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BreakVariant {
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "thematic")]
    Thematic,
}
impl From<&BreakVariant> for BreakVariant {
    fn from(value: &BreakVariant) -> Self {
        value.clone()
    }
}
impl ToString for BreakVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Page => "page".to_string(),
            Self::Thematic => "thematic".to_string(),
        }
    }
}
impl std::str::FromStr for BreakVariant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "page" => Ok(Self::Page),
            "thematic" => Ok(Self::Thematic),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BreakVariant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BreakVariant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BreakVariant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "DiscreteHeading"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"level\",\n    \"name\",\n    \"title\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"level\": {\n      \"type\": \"integer\",\n      \"minimum\": 0.0\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"heading\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscreteHeading {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub level: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: DiscreteHeadingName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    pub title: Inlines,
    #[serde(rename = "type")]
    pub type_: DiscreteHeadingType,
}
impl From<&DiscreteHeading> for DiscreteHeading {
    fn from(value: &DiscreteHeading) -> Self {
        value.clone()
    }
}
impl DiscreteHeading {
    pub fn builder() -> builder::DiscreteHeading {
        Default::default()
    }
}
#[doc = "DiscreteHeadingName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"heading\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DiscreteHeadingName {
    #[serde(rename = "heading")]
    Heading,
}
impl From<&DiscreteHeadingName> for DiscreteHeadingName {
    fn from(value: &DiscreteHeadingName) -> Self {
        value.clone()
    }
}
impl ToString for DiscreteHeadingName {
    fn to_string(&self) -> String {
        match *self {
            Self::Heading => "heading".to_string(),
        }
    }
}
impl std::str::FromStr for DiscreteHeadingName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "heading" => Ok(Self::Heading),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DiscreteHeadingName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiscreteHeadingName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiscreteHeadingName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "DiscreteHeadingType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DiscreteHeadingType {
    #[serde(rename = "block")]
    Block,
}
impl From<&DiscreteHeadingType> for DiscreteHeadingType {
    fn from(value: &DiscreteHeadingType) -> Self {
        value.clone()
    }
}
impl ToString for DiscreteHeadingType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for DiscreteHeadingType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DiscreteHeadingType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiscreteHeadingType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiscreteHeadingType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Dlist"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"items\",\n    \"marker\",\n    \"name\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"items\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/dlistItem\"\n      },\n      \"minItems\": 1\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"marker\": {\n      \"type\": \"string\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"dlist\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dlist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub items: Vec<DlistItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub marker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: DlistName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: DlistType,
}
impl From<&Dlist> for Dlist {
    fn from(value: &Dlist) -> Self {
        value.clone()
    }
}
impl Dlist {
    pub fn builder() -> builder::Dlist {
        Default::default()
    }
}
#[doc = "DlistItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"marker\",\n    \"name\",\n    \"terms\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"blocks\": {\n      \"$ref\": \"#/$defs/nonSectionBlockBody\"\n    },\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"marker\": {\n      \"type\": \"string\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"dlistItem\"\n      ]\n    },\n    \"principal\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"terms\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/inlines\"\n      },\n      \"minItems\": 1\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DlistItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<NonSectionBlockBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub marker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: DlistItemName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    pub terms: Vec<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: DlistItemType,
}
impl From<&DlistItem> for DlistItem {
    fn from(value: &DlistItem) -> Self {
        value.clone()
    }
}
impl DlistItem {
    pub fn builder() -> builder::DlistItem {
        Default::default()
    }
}
#[doc = "DlistItemName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"dlistItem\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DlistItemName {
    #[serde(rename = "dlistItem")]
    DlistItem,
}
impl From<&DlistItemName> for DlistItemName {
    fn from(value: &DlistItemName) -> Self {
        value.clone()
    }
}
impl ToString for DlistItemName {
    fn to_string(&self) -> String {
        match *self {
            Self::DlistItem => "dlistItem".to_string(),
        }
    }
}
impl std::str::FromStr for DlistItemName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dlistItem" => Ok(Self::DlistItem),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DlistItemName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DlistItemName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DlistItemName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "DlistItemType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DlistItemType {
    #[serde(rename = "block")]
    Block,
}
impl From<&DlistItemType> for DlistItemType {
    fn from(value: &DlistItemType) -> Self {
        value.clone()
    }
}
impl ToString for DlistItemType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for DlistItemType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DlistItemType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DlistItemType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DlistItemType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "DlistName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"dlist\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DlistName {
    #[serde(rename = "dlist")]
    Dlist,
}
impl From<&DlistName> for DlistName {
    fn from(value: &DlistName) -> Self {
        value.clone()
    }
}
impl ToString for DlistName {
    fn to_string(&self) -> String {
        match *self {
            Self::Dlist => "dlist".to_string(),
        }
    }
}
impl std::str::FromStr for DlistName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dlist" => Ok(Self::Dlist),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DlistName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DlistName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DlistName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "DlistType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DlistType {
    #[serde(rename = "block")]
    Block,
}
impl From<&DlistType> for DlistType {
    fn from(value: &DlistType) -> Self {
        value.clone()
    }
}
impl ToString for DlistType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for DlistType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DlistType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DlistType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DlistType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Inline"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"oneOf\": [\n    {\n      \"$ref\": \"#/$defs/inlineSpan\"\n    },\n    {\n      \"$ref\": \"#/$defs/inlineRef\"\n    },\n    {\n      \"$ref\": \"#/$defs/inlineLiteral\"\n    }\n  ],\n  \"discriminator\": {\n    \"propertyName\": \"name\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Inline {
    Span(InlineSpan),
    Ref(InlineRef),
    Literal(InlineLiteral),
}
impl From<&Inline> for Inline {
    fn from(value: &Inline) -> Self {
        value.clone()
    }
}
impl From<InlineSpan> for Inline {
    fn from(value: InlineSpan) -> Self {
        Self::Span(value)
    }
}
impl From<InlineRef> for Inline {
    fn from(value: InlineRef) -> Self {
        Self::Ref(value)
    }
}
impl From<InlineLiteral> for Inline {
    fn from(value: InlineLiteral) -> Self {
        Self::Literal(value)
    }
}
#[doc = "InlineLiteral"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"name\",\n    \"type\",\n    \"value\"\n  ],\n  \"properties\": {\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"text\",\n        \"charref\",\n        \"raw\"\n      ]\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"string\"\n      ]\n    },\n    \"value\": {\n      \"type\": \"string\"\n    }\n  },\n  \"additionalProperties\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InlineLiteral {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub name: InlineLiteralName,
    #[serde(rename = "type")]
    pub type_: InlineLiteralType,
    pub value: String,
}
impl From<&InlineLiteral> for InlineLiteral {
    fn from(value: &InlineLiteral) -> Self {
        value.clone()
    }
}
impl InlineLiteral {
    pub fn builder() -> builder::InlineLiteral {
        Default::default()
    }
}
#[doc = "InlineLiteralName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"text\",\n    \"charref\",\n    \"raw\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineLiteralName {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "charref")]
    Charref,
    #[serde(rename = "raw")]
    Raw,
}
impl From<&InlineLiteralName> for InlineLiteralName {
    fn from(value: &InlineLiteralName) -> Self {
        value.clone()
    }
}
impl ToString for InlineLiteralName {
    fn to_string(&self) -> String {
        match *self {
            Self::Text => "text".to_string(),
            Self::Charref => "charref".to_string(),
            Self::Raw => "raw".to_string(),
        }
    }
}
impl std::str::FromStr for InlineLiteralName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "text" => Ok(Self::Text),
            "charref" => Ok(Self::Charref),
            "raw" => Ok(Self::Raw),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineLiteralName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineLiteralName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineLiteralName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineLiteralType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"string\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineLiteralType {
    #[serde(rename = "string")]
    String,
}
impl From<&InlineLiteralType> for InlineLiteralType {
    fn from(value: &InlineLiteralType) -> Self {
        value.clone()
    }
}
impl ToString for InlineLiteralType {
    fn to_string(&self) -> String {
        match *self {
            Self::String => "string".to_string(),
        }
    }
}
impl std::str::FromStr for InlineLiteralType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineLiteralType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineLiteralType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineLiteralType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineRef"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"inlines\",\n    \"name\",\n    \"target\",\n    \"type\",\n    \"variant\"\n  ],\n  \"properties\": {\n    \"inlines\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"ref\"\n      ]\n    },\n    \"target\": {\n      \"type\": \"string\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"inline\"\n      ]\n    },\n    \"variant\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"link\",\n        \"xref\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineRef {
    pub inlines: Inlines,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub name: InlineRefName,
    pub target: String,
    #[serde(rename = "type")]
    pub type_: InlineRefType,
    pub variant: InlineRefVariant,
}
impl From<&InlineRef> for InlineRef {
    fn from(value: &InlineRef) -> Self {
        value.clone()
    }
}
impl InlineRef {
    pub fn builder() -> builder::InlineRef {
        Default::default()
    }
}
#[doc = "InlineRefName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"ref\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineRefName {
    #[serde(rename = "ref")]
    Ref,
}
impl From<&InlineRefName> for InlineRefName {
    fn from(value: &InlineRefName) -> Self {
        value.clone()
    }
}
impl ToString for InlineRefName {
    fn to_string(&self) -> String {
        match *self {
            Self::Ref => "ref".to_string(),
        }
    }
}
impl std::str::FromStr for InlineRefName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ref" => Ok(Self::Ref),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineRefName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineRefName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineRefName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineRefType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"inline\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineRefType {
    #[serde(rename = "inline")]
    Inline,
}
impl From<&InlineRefType> for InlineRefType {
    fn from(value: &InlineRefType) -> Self {
        value.clone()
    }
}
impl ToString for InlineRefType {
    fn to_string(&self) -> String {
        match *self {
            Self::Inline => "inline".to_string(),
        }
    }
}
impl std::str::FromStr for InlineRefType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineRefType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineRefType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineRefType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineRefVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"link\",\n    \"xref\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineRefVariant {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "xref")]
    Xref,
}
impl From<&InlineRefVariant> for InlineRefVariant {
    fn from(value: &InlineRefVariant) -> Self {
        value.clone()
    }
}
impl ToString for InlineRefVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Link => "link".to_string(),
            Self::Xref => "xref".to_string(),
        }
    }
}
impl std::str::FromStr for InlineRefVariant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "link" => Ok(Self::Link),
            "xref" => Ok(Self::Xref),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineRefVariant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineRefVariant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineRefVariant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineSpan"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"form\",\n    \"inlines\",\n    \"name\",\n    \"type\",\n    \"variant\"\n  ],\n  \"properties\": {\n    \"form\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"constrained\",\n        \"unconstrained\"\n      ]\n    },\n    \"inlines\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"span\"\n      ]\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"inline\"\n      ]\n    },\n    \"variant\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"strong\",\n        \"emphasis\",\n        \"code\",\n        \"mark\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineSpan {
    pub form: InlineSpanForm,
    pub inlines: Inlines,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub name: InlineSpanName,
    #[serde(rename = "type")]
    pub type_: InlineSpanType,
    pub variant: InlineSpanVariant,
}
impl From<&InlineSpan> for InlineSpan {
    fn from(value: &InlineSpan) -> Self {
        value.clone()
    }
}
impl InlineSpan {
    pub fn builder() -> builder::InlineSpan {
        Default::default()
    }
}
#[doc = "InlineSpanForm"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"constrained\",\n    \"unconstrained\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineSpanForm {
    #[serde(rename = "constrained")]
    Constrained,
    #[serde(rename = "unconstrained")]
    Unconstrained,
}
impl From<&InlineSpanForm> for InlineSpanForm {
    fn from(value: &InlineSpanForm) -> Self {
        value.clone()
    }
}
impl ToString for InlineSpanForm {
    fn to_string(&self) -> String {
        match *self {
            Self::Constrained => "constrained".to_string(),
            Self::Unconstrained => "unconstrained".to_string(),
        }
    }
}
impl std::str::FromStr for InlineSpanForm {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "constrained" => Ok(Self::Constrained),
            "unconstrained" => Ok(Self::Unconstrained),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineSpanForm {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineSpanForm {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineSpanForm {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineSpanName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"span\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineSpanName {
    #[serde(rename = "span")]
    Span,
}
impl From<&InlineSpanName> for InlineSpanName {
    fn from(value: &InlineSpanName) -> Self {
        value.clone()
    }
}
impl ToString for InlineSpanName {
    fn to_string(&self) -> String {
        match *self {
            Self::Span => "span".to_string(),
        }
    }
}
impl std::str::FromStr for InlineSpanName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "span" => Ok(Self::Span),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineSpanName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineSpanName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineSpanName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineSpanType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"inline\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineSpanType {
    #[serde(rename = "inline")]
    Inline,
}
impl From<&InlineSpanType> for InlineSpanType {
    fn from(value: &InlineSpanType) -> Self {
        value.clone()
    }
}
impl ToString for InlineSpanType {
    fn to_string(&self) -> String {
        match *self {
            Self::Inline => "inline".to_string(),
        }
    }
}
impl std::str::FromStr for InlineSpanType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineSpanType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineSpanType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineSpanType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "InlineSpanVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"strong\",\n    \"emphasis\",\n    \"code\",\n    \"mark\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InlineSpanVariant {
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "emphasis")]
    Emphasis,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "mark")]
    Mark,
}
impl From<&InlineSpanVariant> for InlineSpanVariant {
    fn from(value: &InlineSpanVariant) -> Self {
        value.clone()
    }
}
impl ToString for InlineSpanVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Strong => "strong".to_string(),
            Self::Emphasis => "emphasis".to_string(),
            Self::Code => "code".to_string(),
            Self::Mark => "mark".to_string(),
        }
    }
}
impl std::str::FromStr for InlineSpanVariant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "strong" => Ok(Self::Strong),
            "emphasis" => Ok(Self::Emphasis),
            "code" => Ok(Self::Code),
            "mark" => Ok(Self::Mark),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InlineSpanVariant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InlineSpanVariant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InlineSpanVariant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Inlines"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"array\",\n  \"items\": {\n    \"$ref\": \"#/$defs/inline\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inlines(pub Vec<Inline>);
impl std::ops::Deref for Inlines {
    type Target = Vec<Inline>;
    fn deref(&self) -> &Vec<Inline> {
        &self.0
    }
}
impl From<Inlines> for Vec<Inline> {
    fn from(value: Inlines) -> Self {
        value.0
    }
}
impl From<&Inlines> for Inlines {
    fn from(value: &Inlines) -> Self {
        value.clone()
    }
}
impl From<Vec<Inline>> for Inlines {
    fn from(value: Vec<Inline>) -> Self {
        Self(value)
    }
}
#[doc = "LeafBlock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"oneOf\": [\n    {\n      \"allOf\": [\n        {\n          \"type\": \"object\",\n          \"required\": [\n            \"name\",\n            \"type\"\n          ],\n          \"properties\": {\n            \"id\": {\n              \"type\": \"string\"\n            },\n            \"inlines\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"metadata\": {\n              \"$ref\": \"#/$defs/blockMetadata\"\n            },\n            \"name\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"listing\",\n                \"literal\",\n                \"paragraph\",\n                \"pass\",\n                \"stem\",\n                \"verse\"\n              ]\n            },\n            \"reftext\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"title\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"type\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"block\"\n              ]\n            }\n          }\n        },\n        {\n          \"required\": [\n            \"delimiter\",\n            \"form\"\n          ],\n          \"properties\": {\n            \"delimiter\": {\n              \"type\": \"string\"\n            },\n            \"form\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"delimited\"\n              ]\n            }\n          }\n        }\n      ]\n    },\n    {\n      \"allOf\": [\n        {\n          \"type\": \"object\",\n          \"required\": [\n            \"name\",\n            \"type\"\n          ],\n          \"properties\": {\n            \"id\": {\n              \"type\": \"string\"\n            },\n            \"inlines\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"metadata\": {\n              \"$ref\": \"#/$defs/blockMetadata\"\n            },\n            \"name\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"listing\",\n                \"literal\",\n                \"paragraph\",\n                \"pass\",\n                \"stem\",\n                \"verse\"\n              ]\n            },\n            \"reftext\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"title\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"type\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"block\"\n              ]\n            }\n          }\n        },\n        {\n          \"properties\": {\n            \"form\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"indented\",\n                \"paragraph\"\n              ]\n            }\n          }\n        }\n      ]\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LeafBlock {
    Variant0 {
        delimiter: String,
        form: LeafBlockVariant0Form,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<BlockMetadata>,
        name: LeafBlockVariant0Name,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reftext: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<Inlines>,
        #[serde(rename = "type")]
        type_: LeafBlockVariant0Type,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        form: Option<LeafBlockVariant1Form>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<BlockMetadata>,
        name: LeafBlockVariant1Name,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reftext: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<Inlines>,
        #[serde(rename = "type")]
        type_: LeafBlockVariant1Type,
    },
}
impl From<&LeafBlock> for LeafBlock {
    fn from(value: &LeafBlock) -> Self {
        value.clone()
    }
}
#[doc = "LeafBlockVariant0Form"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"delimited\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant0Form {
    #[serde(rename = "delimited")]
    Delimited,
}
impl From<&LeafBlockVariant0Form> for LeafBlockVariant0Form {
    fn from(value: &LeafBlockVariant0Form) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant0Form {
    fn to_string(&self) -> String {
        match *self {
            Self::Delimited => "delimited".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant0Form {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "delimited" => Ok(Self::Delimited),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "LeafBlockVariant0Name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"listing\",\n    \"literal\",\n    \"paragraph\",\n    \"pass\",\n    \"stem\",\n    \"verse\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant0Name {
    #[serde(rename = "listing")]
    Listing,
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "paragraph")]
    Paragraph,
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "stem")]
    Stem,
    #[serde(rename = "verse")]
    Verse,
}
impl From<&LeafBlockVariant0Name> for LeafBlockVariant0Name {
    fn from(value: &LeafBlockVariant0Name) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant0Name {
    fn to_string(&self) -> String {
        match *self {
            Self::Listing => "listing".to_string(),
            Self::Literal => "literal".to_string(),
            Self::Paragraph => "paragraph".to_string(),
            Self::Pass => "pass".to_string(),
            Self::Stem => "stem".to_string(),
            Self::Verse => "verse".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant0Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "listing" => Ok(Self::Listing),
            "literal" => Ok(Self::Literal),
            "paragraph" => Ok(Self::Paragraph),
            "pass" => Ok(Self::Pass),
            "stem" => Ok(Self::Stem),
            "verse" => Ok(Self::Verse),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "LeafBlockVariant0Type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant0Type {
    #[serde(rename = "block")]
    Block,
}
impl From<&LeafBlockVariant0Type> for LeafBlockVariant0Type {
    fn from(value: &LeafBlockVariant0Type) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant0Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant0Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "LeafBlockVariant1Form"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"indented\",\n    \"paragraph\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant1Form {
    #[serde(rename = "indented")]
    Indented,
    #[serde(rename = "paragraph")]
    Paragraph,
}
impl From<&LeafBlockVariant1Form> for LeafBlockVariant1Form {
    fn from(value: &LeafBlockVariant1Form) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant1Form {
    fn to_string(&self) -> String {
        match *self {
            Self::Indented => "indented".to_string(),
            Self::Paragraph => "paragraph".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant1Form {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "indented" => Ok(Self::Indented),
            "paragraph" => Ok(Self::Paragraph),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "LeafBlockVariant1Name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"listing\",\n    \"literal\",\n    \"paragraph\",\n    \"pass\",\n    \"stem\",\n    \"verse\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant1Name {
    #[serde(rename = "listing")]
    Listing,
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "paragraph")]
    Paragraph,
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "stem")]
    Stem,
    #[serde(rename = "verse")]
    Verse,
}
impl From<&LeafBlockVariant1Name> for LeafBlockVariant1Name {
    fn from(value: &LeafBlockVariant1Name) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant1Name {
    fn to_string(&self) -> String {
        match *self {
            Self::Listing => "listing".to_string(),
            Self::Literal => "literal".to_string(),
            Self::Paragraph => "paragraph".to_string(),
            Self::Pass => "pass".to_string(),
            Self::Stem => "stem".to_string(),
            Self::Verse => "verse".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant1Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "listing" => Ok(Self::Listing),
            "literal" => Ok(Self::Literal),
            "paragraph" => Ok(Self::Paragraph),
            "pass" => Ok(Self::Pass),
            "stem" => Ok(Self::Stem),
            "verse" => Ok(Self::Verse),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "LeafBlockVariant1Type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LeafBlockVariant1Type {
    #[serde(rename = "block")]
    Block,
}
impl From<&LeafBlockVariant1Type> for LeafBlockVariant1Type {
    fn from(value: &LeafBlockVariant1Type) -> Self {
        value.clone()
    }
}
impl ToString for LeafBlockVariant1Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for LeafBlockVariant1Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LeafBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LeafBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LeafBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "List"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"items\",\n    \"marker\",\n    \"name\",\n    \"type\",\n    \"variant\"\n  ],\n  \"properties\": {\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"items\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/listItem\"\n      },\n      \"minItems\": 1\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"marker\": {\n      \"type\": \"string\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"list\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    },\n    \"variant\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"callout\",\n        \"ordered\",\n        \"unordered\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct List {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub items: Vec<ListItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub marker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: ListName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: ListType,
    pub variant: ListVariant,
}
impl From<&List> for List {
    fn from(value: &List) -> Self {
        value.clone()
    }
}
impl List {
    pub fn builder() -> builder::List {
        Default::default()
    }
}
#[doc = "ListItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"marker\",\n    \"name\",\n    \"principal\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"blocks\": {\n      \"$ref\": \"#/$defs/nonSectionBlockBody\"\n    },\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"marker\": {\n      \"type\": \"string\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"listItem\"\n      ]\n    },\n    \"principal\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<NonSectionBlockBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    pub marker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: ListItemName,
    pub principal: Inlines,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Inlines>,
    #[serde(rename = "type")]
    pub type_: ListItemType,
}
impl From<&ListItem> for ListItem {
    fn from(value: &ListItem) -> Self {
        value.clone()
    }
}
impl ListItem {
    pub fn builder() -> builder::ListItem {
        Default::default()
    }
}
#[doc = "ListItemName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"listItem\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ListItemName {
    #[serde(rename = "listItem")]
    ListItem,
}
impl From<&ListItemName> for ListItemName {
    fn from(value: &ListItemName) -> Self {
        value.clone()
    }
}
impl ToString for ListItemName {
    fn to_string(&self) -> String {
        match *self {
            Self::ListItem => "listItem".to_string(),
        }
    }
}
impl std::str::FromStr for ListItemName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "listItem" => Ok(Self::ListItem),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ListItemName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListItemName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListItemName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ListItemType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ListItemType {
    #[serde(rename = "block")]
    Block,
}
impl From<&ListItemType> for ListItemType {
    fn from(value: &ListItemType) -> Self {
        value.clone()
    }
}
impl ToString for ListItemType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for ListItemType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ListItemType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListItemType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListItemType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ListName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"list\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ListName {
    #[serde(rename = "list")]
    List,
}
impl From<&ListName> for ListName {
    fn from(value: &ListName) -> Self {
        value.clone()
    }
}
impl ToString for ListName {
    fn to_string(&self) -> String {
        match *self {
            Self::List => "list".to_string(),
        }
    }
}
impl std::str::FromStr for ListName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ListName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ListType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ListType {
    #[serde(rename = "block")]
    Block,
}
impl From<&ListType> for ListType {
    fn from(value: &ListType) -> Self {
        value.clone()
    }
}
impl ToString for ListType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for ListType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ListType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ListVariant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"callout\",\n    \"ordered\",\n    \"unordered\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ListVariant {
    #[serde(rename = "callout")]
    Callout,
    #[serde(rename = "ordered")]
    Ordered,
    #[serde(rename = "unordered")]
    Unordered,
}
impl From<&ListVariant> for ListVariant {
    fn from(value: &ListVariant) -> Self {
        value.clone()
    }
}
impl ToString for ListVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Callout => "callout".to_string(),
            Self::Ordered => "ordered".to_string(),
            Self::Unordered => "unordered".to_string(),
        }
    }
}
impl std::str::FromStr for ListVariant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "callout" => Ok(Self::Callout),
            "ordered" => Ok(Self::Ordered),
            "unordered" => Ok(Self::Unordered),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ListVariant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ListVariant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ListVariant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Location"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"array\",\n  \"maxItems\": 2,\n  \"minItems\": 2,\n  \"prefixItems\": [\n    {\n      \"$ref\": \"#/$defs/locationBoundary\"\n    },\n    {\n      \"$ref\": \"#/$defs/locationBoundary\"\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location(pub [serde_json::Value; 2usize]);
impl std::ops::Deref for Location {
    type Target = [serde_json::Value; 2usize];
    fn deref(&self) -> &[serde_json::Value; 2usize] {
        &self.0
    }
}
impl From<Location> for [serde_json::Value; 2usize] {
    fn from(value: Location) -> Self {
        value.0
    }
}
impl From<&Location> for Location {
    fn from(value: &Location) -> Self {
        value.clone()
    }
}
impl From<[serde_json::Value; 2usize]> for Location {
    fn from(value: [serde_json::Value; 2usize]) -> Self {
        Self(value)
    }
}
#[doc = "LocationBoundary"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"col\",\n    \"line\"\n  ],\n  \"properties\": {\n    \"col\": {\n      \"type\": \"integer\",\n      \"minimum\": 0.0\n    },\n    \"file\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\"\n      },\n      \"minItems\": 1\n    },\n    \"line\": {\n      \"type\": \"integer\",\n      \"minimum\": 1.0\n    }\n  },\n  \"additionalProperties\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocationBoundary {
    pub col: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file: Vec<String>,
    pub line: std::num::NonZeroU64,
}
impl From<&LocationBoundary> for LocationBoundary {
    fn from(value: &LocationBoundary) -> Self {
        value.clone()
    }
}
impl LocationBoundary {
    pub fn builder() -> builder::LocationBoundary {
        Default::default()
    }
}
#[doc = "NonSectionBlockBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"array\",\n  \"items\": {\n    \"$ref\": \"#/$defs/block\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonSectionBlockBody(pub Vec<Block>);
impl std::ops::Deref for NonSectionBlockBody {
    type Target = Vec<Block>;
    fn deref(&self) -> &Vec<Block> {
        &self.0
    }
}
impl From<NonSectionBlockBody> for Vec<Block> {
    fn from(value: NonSectionBlockBody) -> Self {
        value.0
    }
}
impl From<&NonSectionBlockBody> for NonSectionBlockBody {
    fn from(value: &NonSectionBlockBody) -> Self {
        value.clone()
    }
}
impl From<Vec<Block>> for NonSectionBlockBody {
    fn from(value: Vec<Block>) -> Self {
        Self(value)
    }
}
#[doc = "ParentBlock"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"oneOf\": [\n    {\n      \"allOf\": [\n        {\n          \"type\": \"object\",\n          \"required\": [\n            \"delimiter\",\n            \"form\",\n            \"type\"\n          ],\n          \"properties\": {\n            \"blocks\": {\n              \"$ref\": \"#/$defs/nonSectionBlockBody\"\n            },\n            \"delimiter\": {\n              \"type\": \"string\"\n            },\n            \"form\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"delimited\"\n              ]\n            },\n            \"id\": {\n              \"type\": \"string\"\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"metadata\": {\n              \"$ref\": \"#/$defs/blockMetadata\"\n            },\n            \"reftext\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"title\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"type\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"block\"\n              ]\n            }\n          }\n        },\n        {\n          \"required\": [\n            \"name\"\n          ],\n          \"properties\": {\n            \"name\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"example\",\n                \"sidebar\",\n                \"open\",\n                \"quote\"\n              ]\n            }\n          }\n        }\n      ]\n    },\n    {\n      \"allOf\": [\n        {\n          \"type\": \"object\",\n          \"required\": [\n            \"delimiter\",\n            \"form\",\n            \"type\"\n          ],\n          \"properties\": {\n            \"blocks\": {\n              \"$ref\": \"#/$defs/nonSectionBlockBody\"\n            },\n            \"delimiter\": {\n              \"type\": \"string\"\n            },\n            \"form\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"delimited\"\n              ]\n            },\n            \"id\": {\n              \"type\": \"string\"\n            },\n            \"location\": {\n              \"$ref\": \"#/$defs/location\"\n            },\n            \"metadata\": {\n              \"$ref\": \"#/$defs/blockMetadata\"\n            },\n            \"reftext\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"title\": {\n              \"$ref\": \"#/$defs/inlines\"\n            },\n            \"type\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"block\"\n              ]\n            }\n          }\n        },\n        {\n          \"required\": [\n            \"name\",\n            \"variant\"\n          ],\n          \"properties\": {\n            \"name\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"admonition\"\n              ]\n            },\n            \"variant\": {\n              \"type\": \"string\",\n              \"enum\": [\n                \"caution\",\n                \"important\",\n                \"note\",\n                \"tip\",\n                \"warning\"\n              ]\n            }\n          }\n        }\n      ]\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ParentBlock {
    Variant0 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<NonSectionBlockBody>,
        delimiter: String,
        form: ParentBlockVariant0Form,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<BlockMetadata>,
        name: ParentBlockVariant0Name,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reftext: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<Inlines>,
        #[serde(rename = "type")]
        type_: ParentBlockVariant0Type,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<NonSectionBlockBody>,
        delimiter: String,
        form: ParentBlockVariant1Form,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<BlockMetadata>,
        name: ParentBlockVariant1Name,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reftext: Option<Inlines>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<Inlines>,
        #[serde(rename = "type")]
        type_: ParentBlockVariant1Type,
        variant: ParentBlockVariant1Variant,
    },
}
impl From<&ParentBlock> for ParentBlock {
    fn from(value: &ParentBlock) -> Self {
        value.clone()
    }
}
#[doc = "ParentBlockVariant0Form"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"delimited\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant0Form {
    #[serde(rename = "delimited")]
    Delimited,
}
impl From<&ParentBlockVariant0Form> for ParentBlockVariant0Form {
    fn from(value: &ParentBlockVariant0Form) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant0Form {
    fn to_string(&self) -> String {
        match *self {
            Self::Delimited => "delimited".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant0Form {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "delimited" => Ok(Self::Delimited),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant0Form {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant0Name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"example\",\n    \"sidebar\",\n    \"open\",\n    \"quote\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant0Name {
    #[serde(rename = "example")]
    Example,
    #[serde(rename = "sidebar")]
    Sidebar,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "quote")]
    Quote,
}
impl From<&ParentBlockVariant0Name> for ParentBlockVariant0Name {
    fn from(value: &ParentBlockVariant0Name) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant0Name {
    fn to_string(&self) -> String {
        match *self {
            Self::Example => "example".to_string(),
            Self::Sidebar => "sidebar".to_string(),
            Self::Open => "open".to_string(),
            Self::Quote => "quote".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant0Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "example" => Ok(Self::Example),
            "sidebar" => Ok(Self::Sidebar),
            "open" => Ok(Self::Open),
            "quote" => Ok(Self::Quote),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant0Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant0Type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant0Type {
    #[serde(rename = "block")]
    Block,
}
impl From<&ParentBlockVariant0Type> for ParentBlockVariant0Type {
    fn from(value: &ParentBlockVariant0Type) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant0Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant0Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant0Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant1Form"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"delimited\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant1Form {
    #[serde(rename = "delimited")]
    Delimited,
}
impl From<&ParentBlockVariant1Form> for ParentBlockVariant1Form {
    fn from(value: &ParentBlockVariant1Form) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant1Form {
    fn to_string(&self) -> String {
        match *self {
            Self::Delimited => "delimited".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant1Form {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "delimited" => Ok(Self::Delimited),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant1Form {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant1Name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"admonition\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant1Name {
    #[serde(rename = "admonition")]
    Admonition,
}
impl From<&ParentBlockVariant1Name> for ParentBlockVariant1Name {
    fn from(value: &ParentBlockVariant1Name) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant1Name {
    fn to_string(&self) -> String {
        match *self {
            Self::Admonition => "admonition".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant1Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "admonition" => Ok(Self::Admonition),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant1Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant1Type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant1Type {
    #[serde(rename = "block")]
    Block,
}
impl From<&ParentBlockVariant1Type> for ParentBlockVariant1Type {
    fn from(value: &ParentBlockVariant1Type) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant1Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant1Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant1Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ParentBlockVariant1Variant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"caution\",\n    \"important\",\n    \"note\",\n    \"tip\",\n    \"warning\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParentBlockVariant1Variant {
    #[serde(rename = "caution")]
    Caution,
    #[serde(rename = "important")]
    Important,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "tip")]
    Tip,
    #[serde(rename = "warning")]
    Warning,
}
impl From<&ParentBlockVariant1Variant> for ParentBlockVariant1Variant {
    fn from(value: &ParentBlockVariant1Variant) -> Self {
        value.clone()
    }
}
impl ToString for ParentBlockVariant1Variant {
    fn to_string(&self) -> String {
        match *self {
            Self::Caution => "caution".to_string(),
            Self::Important => "important".to_string(),
            Self::Note => "note".to_string(),
            Self::Tip => "tip".to_string(),
            Self::Warning => "warning".to_string(),
        }
    }
}
impl std::str::FromStr for ParentBlockVariant1Variant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "caution" => Ok(Self::Caution),
            "important" => Ok(Self::Important),
            "note" => Ok(Self::Note),
            "tip" => Ok(Self::Tip),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ParentBlockVariant1Variant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ParentBlockVariant1Variant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ParentBlockVariant1Variant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Section"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"level\",\n    \"name\",\n    \"title\",\n    \"type\"\n  ],\n  \"properties\": {\n    \"blocks\": {\n      \"$ref\": \"#/$defs/sectionBody\"\n    },\n    \"id\": {\n      \"type\": \"string\"\n    },\n    \"level\": {\n      \"type\": \"integer\",\n      \"minimum\": 0.0\n    },\n    \"location\": {\n      \"$ref\": \"#/$defs/location\"\n    },\n    \"metadata\": {\n      \"$ref\": \"#/$defs/blockMetadata\"\n    },\n    \"name\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"section\"\n      ]\n    },\n    \"reftext\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"title\": {\n      \"$ref\": \"#/$defs/inlines\"\n    },\n    \"type\": {\n      \"type\": \"string\",\n      \"enum\": [\n        \"block\"\n      ]\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Section {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<SectionBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub level: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
    pub name: SectionName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reftext: Option<Inlines>,
    pub title: Inlines,
    #[serde(rename = "type")]
    pub type_: SectionType,
}
impl From<&Section> for Section {
    fn from(value: &Section) -> Self {
        value.clone()
    }
}
impl Section {
    pub fn builder() -> builder::Section {
        Default::default()
    }
}
#[doc = "SectionBody"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"array\",\n  \"items\": {\n    \"type\": \"object\",\n    \"oneOf\": [\n      {\n        \"$ref\": \"#/$defs/block\"\n      },\n      {\n        \"$ref\": \"#/$defs/section\"\n      }\n    ]\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectionBody(pub Vec<SectionBodyItem>);
impl std::ops::Deref for SectionBody {
    type Target = Vec<SectionBodyItem>;
    fn deref(&self) -> &Vec<SectionBodyItem> {
        &self.0
    }
}
impl From<SectionBody> for Vec<SectionBodyItem> {
    fn from(value: SectionBody) -> Self {
        value.0
    }
}
impl From<&SectionBody> for SectionBody {
    fn from(value: &SectionBody) -> Self {
        value.clone()
    }
}
impl From<Vec<SectionBodyItem>> for SectionBody {
    fn from(value: Vec<SectionBodyItem>) -> Self {
        Self(value)
    }
}
#[doc = "SectionBodyItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"oneOf\": [\n    {\n      \"$ref\": \"#/$defs/block\"\n    },\n    {\n      \"$ref\": \"#/$defs/section\"\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SectionBodyItem {
    Block(Block),
    Section(Section),
}
impl From<&SectionBodyItem> for SectionBodyItem {
    fn from(value: &SectionBodyItem) -> Self {
        value.clone()
    }
}
impl From<Block> for SectionBodyItem {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
impl From<Section> for SectionBodyItem {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}
#[doc = "SectionName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"section\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SectionName {
    #[serde(rename = "section")]
    Section,
}
impl From<&SectionName> for SectionName {
    fn from(value: &SectionName) -> Self {
        value.clone()
    }
}
impl ToString for SectionName {
    fn to_string(&self) -> String {
        match *self {
            Self::Section => "section".to_string(),
        }
    }
}
impl std::str::FromStr for SectionName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "section" => Ok(Self::Section),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SectionName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SectionName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SectionName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "SectionType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"enum\": [\n    \"block\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SectionType {
    #[serde(rename = "block")]
    Block,
}
impl From<&SectionType> for SectionType {
    fn from(value: &SectionType) -> Self {
        value.clone()
    }
}
impl ToString for SectionType {
    fn to_string(&self) -> String {
        match *self {
            Self::Block => "block".to_string(),
        }
    }
}
impl std::str::FromStr for SectionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SectionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SectionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SectionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AbstractBlock {
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::AbstractBlockType, String>,
    }
    impl Default for AbstractBlock {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AbstractBlock {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AbstractBlockType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AbstractBlock> for super::AbstractBlock {
        type Error = String;
        fn try_from(value: AbstractBlock) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                location: value.location?,
                metadata: value.metadata?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AbstractBlock> for AbstractBlock {
        fn from(value: super::AbstractBlock) -> Self {
            Self {
                id: Ok(value.id),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AbstractHeading {
        id: Result<Option<String>, String>,
        level: Result<u64, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<super::Inlines, String>,
        type_: Result<super::AbstractHeadingType, String>,
    }
    impl Default for AbstractHeading {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                level: Err("no value supplied for level".to_string()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                reftext: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AbstractHeading {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<u64>,
            T::Error: std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AbstractHeadingType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AbstractHeading> for super::AbstractHeading {
        type Error = String;
        fn try_from(value: AbstractHeading) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                level: value.level?,
                location: value.location?,
                metadata: value.metadata?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AbstractHeading> for AbstractHeading {
        fn from(value: super::AbstractHeading) -> Self {
            Self {
                id: Ok(value.id),
                level: Ok(value.level),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AbstractListItem {
        blocks: Result<Option<super::NonSectionBlockBody>, String>,
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        marker: Result<String, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        principal: Result<Option<super::Inlines>, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::AbstractListItemType, String>,
    }
    impl Default for AbstractListItem {
        fn default() -> Self {
            Self {
                blocks: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                marker: Err("no value supplied for marker".to_string()),
                metadata: Ok(Default::default()),
                principal: Ok(Default::default()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AbstractListItem {
        pub fn blocks<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonSectionBlockBody>>,
            T::Error: std::fmt::Display,
        {
            self.blocks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocks: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn marker<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marker: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn principal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.principal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AbstractListItemType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AbstractListItem> for super::AbstractListItem {
        type Error = String;
        fn try_from(value: AbstractListItem) -> Result<Self, String> {
            Ok(Self {
                blocks: value.blocks?,
                id: value.id?,
                location: value.location?,
                marker: value.marker?,
                metadata: value.metadata?,
                principal: value.principal?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AbstractListItem> for AbstractListItem {
        fn from(value: super::AbstractListItem) -> Self {
            Self {
                blocks: Ok(value.blocks),
                id: Ok(value.id),
                location: Ok(value.location),
                marker: Ok(value.marker),
                metadata: Ok(value.metadata),
                principal: Ok(value.principal),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AbstractParentInline {
        inlines: Result<super::Inlines, String>,
        location: Result<Option<super::Location>, String>,
        type_: Result<super::AbstractParentInlineType, String>,
    }
    impl Default for AbstractParentInline {
        fn default() -> Self {
            Self {
                inlines: Err("no value supplied for inlines".to_string()),
                location: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AbstractParentInline {
        pub fn inlines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.inlines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inlines: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AbstractParentInlineType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AbstractParentInline> for super::AbstractParentInline {
        type Error = String;
        fn try_from(value: AbstractParentInline) -> Result<Self, String> {
            Ok(Self {
                inlines: value.inlines?,
                location: value.location?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AbstractParentInline> for AbstractParentInline {
        fn from(value: super::AbstractParentInline) -> Self {
            Self {
                inlines: Ok(value.inlines),
                location: Ok(value.location),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AsciiDocAbstractSemanticGraphAsgHeader {
        authors: Result<Vec<super::Author>, String>,
        location: Result<Option<super::Location>, String>,
        title: Result<Option<super::Inlines>, String>,
    }
    impl Default for AsciiDocAbstractSemanticGraphAsgHeader {
        fn default() -> Self {
            Self {
                authors: Ok(Default::default()),
                location: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl AsciiDocAbstractSemanticGraphAsgHeader {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Author>>,
            T::Error: std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authors: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AsciiDocAbstractSemanticGraphAsgHeader>
        for super::AsciiDocAbstractSemanticGraphAsgHeader
    {
        type Error = String;
        fn try_from(value: AsciiDocAbstractSemanticGraphAsgHeader) -> Result<Self, String> {
            Ok(Self {
                authors: value.authors?,
                location: value.location?,
                title: value.title?,
            })
        }
    }
    impl From<super::AsciiDocAbstractSemanticGraphAsgHeader>
        for AsciiDocAbstractSemanticGraphAsgHeader
    {
        fn from(value: super::AsciiDocAbstractSemanticGraphAsgHeader) -> Self {
            Self {
                authors: Ok(value.authors),
                location: Ok(value.location),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AsciiDocAbstractSemanticGraphAsgVariant1Header {
        authors: Result<Vec<super::Author>, String>,
        location: Result<Option<super::Location>, String>,
        title: Result<Option<super::Inlines>, String>,
    }
    impl Default for AsciiDocAbstractSemanticGraphAsgVariant1Header {
        fn default() -> Self {
            Self {
                authors: Ok(Default::default()),
                location: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl AsciiDocAbstractSemanticGraphAsgVariant1Header {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Author>>,
            T::Error: std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authors: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AsciiDocAbstractSemanticGraphAsgVariant1Header>
        for super::AsciiDocAbstractSemanticGraphAsgVariant1Header
    {
        type Error = String;
        fn try_from(value: AsciiDocAbstractSemanticGraphAsgVariant1Header) -> Result<Self, String> {
            Ok(Self {
                authors: value.authors?,
                location: value.location?,
                title: value.title?,
            })
        }
    }
    impl From<super::AsciiDocAbstractSemanticGraphAsgVariant1Header>
        for AsciiDocAbstractSemanticGraphAsgVariant1Header
    {
        fn from(value: super::AsciiDocAbstractSemanticGraphAsgVariant1Header) -> Self {
            Self {
                authors: Ok(value.authors),
                location: Ok(value.location),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Author {
        address: Result<Option<String>, String>,
        firstname: Result<Option<String>, String>,
        fullname: Result<Option<String>, String>,
        initials: Result<Option<String>, String>,
        lastname: Result<Option<String>, String>,
        middlename: Result<Option<String>, String>,
    }
    impl Default for Author {
        fn default() -> Self {
            Self {
                address: Ok(Default::default()),
                firstname: Ok(Default::default()),
                fullname: Ok(Default::default()),
                initials: Ok(Default::default()),
                lastname: Ok(Default::default()),
                middlename: Ok(Default::default()),
            }
        }
    }
    impl Author {
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address: {}", e));
            self
        }
        pub fn firstname<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.firstname = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for firstname: {}", e));
            self
        }
        pub fn fullname<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.fullname = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fullname: {}", e));
            self
        }
        pub fn initials<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.initials = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for initials: {}", e));
            self
        }
        pub fn lastname<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.lastname = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lastname: {}", e));
            self
        }
        pub fn middlename<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.middlename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for middlename: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Author> for super::Author {
        type Error = String;
        fn try_from(value: Author) -> Result<Self, String> {
            Ok(Self {
                address: value.address?,
                firstname: value.firstname?,
                fullname: value.fullname?,
                initials: value.initials?,
                lastname: value.lastname?,
                middlename: value.middlename?,
            })
        }
    }
    impl From<super::Author> for Author {
        fn from(value: super::Author) -> Self {
            Self {
                address: Ok(value.address),
                firstname: Ok(value.firstname),
                fullname: Ok(value.fullname),
                initials: Ok(value.initials),
                lastname: Ok(value.lastname),
                middlename: Ok(value.middlename),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlockMacro {
        form: Result<super::BlockMacroForm, String>,
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::BlockMacroName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        target: Result<Option<String>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::BlockMacroType, String>,
    }
    impl Default for BlockMacro {
        fn default() -> Self {
            Self {
                form: Err("no value supplied for form".to_string()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                target: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl BlockMacro {
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BlockMacroForm>,
            T::Error: std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BlockMacroName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BlockMacroType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BlockMacro> for super::BlockMacro {
        type Error = String;
        fn try_from(value: BlockMacro) -> Result<Self, String> {
            Ok(Self {
                form: value.form?,
                id: value.id?,
                location: value.location?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                target: value.target?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::BlockMacro> for BlockMacro {
        fn from(value: super::BlockMacro) -> Self {
            Self {
                form: Ok(value.form),
                id: Ok(value.id),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                target: Ok(value.target),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlockMetadata {
        attributes: Result<std::collections::HashMap<String, String>, String>,
        location: Result<Option<super::Location>, String>,
        options: Result<Vec<String>, String>,
        roles: Result<Vec<String>, String>,
    }
    impl Default for BlockMetadata {
        fn default() -> Self {
            Self {
                attributes: Ok(Default::default()),
                location: Ok(Default::default()),
                options: Ok(Default::default()),
                roles: Ok(Default::default()),
            }
        }
    }
    impl BlockMetadata {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, String>>,
            T::Error: std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for options: {}", e));
            self
        }
        pub fn roles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.roles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roles: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BlockMetadata> for super::BlockMetadata {
        type Error = String;
        fn try_from(value: BlockMetadata) -> Result<Self, String> {
            Ok(Self {
                attributes: value.attributes?,
                location: value.location?,
                options: value.options?,
                roles: value.roles?,
            })
        }
    }
    impl From<super::BlockMetadata> for BlockMetadata {
        fn from(value: super::BlockMetadata) -> Self {
            Self {
                attributes: Ok(value.attributes),
                location: Ok(value.location),
                options: Ok(value.options),
                roles: Ok(value.roles),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Break {
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::BreakName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::BreakType, String>,
        variant: Result<super::BreakVariant, String>,
    }
    impl Default for Break {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl Break {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BreakName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BreakType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BreakVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Break> for super::Break {
        type Error = String;
        fn try_from(value: Break) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                location: value.location?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
                variant: value.variant?,
            })
        }
    }
    impl From<super::Break> for Break {
        fn from(value: super::Break) -> Self {
            Self {
                id: Ok(value.id),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscreteHeading {
        id: Result<Option<String>, String>,
        level: Result<u64, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::DiscreteHeadingName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<super::Inlines, String>,
        type_: Result<super::DiscreteHeadingType, String>,
    }
    impl Default for DiscreteHeading {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                level: Err("no value supplied for level".to_string()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DiscreteHeading {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<u64>,
            T::Error: std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DiscreteHeadingName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DiscreteHeadingType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DiscreteHeading> for super::DiscreteHeading {
        type Error = String;
        fn try_from(value: DiscreteHeading) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                level: value.level?,
                location: value.location?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DiscreteHeading> for DiscreteHeading {
        fn from(value: super::DiscreteHeading) -> Self {
            Self {
                id: Ok(value.id),
                level: Ok(value.level),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Dlist {
        id: Result<Option<String>, String>,
        items: Result<Vec<super::DlistItem>, String>,
        location: Result<Option<super::Location>, String>,
        marker: Result<String, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::DlistName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::DlistType, String>,
    }
    impl Default for Dlist {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                items: Err("no value supplied for items".to_string()),
                location: Ok(Default::default()),
                marker: Err("no value supplied for marker".to_string()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Dlist {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::DlistItem>>,
            T::Error: std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn marker<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marker: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DlistName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DlistType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Dlist> for super::Dlist {
        type Error = String;
        fn try_from(value: Dlist) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                items: value.items?,
                location: value.location?,
                marker: value.marker?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Dlist> for Dlist {
        fn from(value: super::Dlist) -> Self {
            Self {
                id: Ok(value.id),
                items: Ok(value.items),
                location: Ok(value.location),
                marker: Ok(value.marker),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DlistItem {
        blocks: Result<Option<super::NonSectionBlockBody>, String>,
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        marker: Result<String, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::DlistItemName, String>,
        principal: Result<Option<super::Inlines>, String>,
        reftext: Result<Option<super::Inlines>, String>,
        terms: Result<Vec<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::DlistItemType, String>,
    }
    impl Default for DlistItem {
        fn default() -> Self {
            Self {
                blocks: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                marker: Err("no value supplied for marker".to_string()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                principal: Ok(Default::default()),
                reftext: Ok(Default::default()),
                terms: Err("no value supplied for terms".to_string()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DlistItem {
        pub fn blocks<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonSectionBlockBody>>,
            T::Error: std::fmt::Display,
        {
            self.blocks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocks: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn marker<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marker: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DlistItemName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn principal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.principal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn terms<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.terms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terms: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DlistItemType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DlistItem> for super::DlistItem {
        type Error = String;
        fn try_from(value: DlistItem) -> Result<Self, String> {
            Ok(Self {
                blocks: value.blocks?,
                id: value.id?,
                location: value.location?,
                marker: value.marker?,
                metadata: value.metadata?,
                name: value.name?,
                principal: value.principal?,
                reftext: value.reftext?,
                terms: value.terms?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DlistItem> for DlistItem {
        fn from(value: super::DlistItem) -> Self {
            Self {
                blocks: Ok(value.blocks),
                id: Ok(value.id),
                location: Ok(value.location),
                marker: Ok(value.marker),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                principal: Ok(value.principal),
                reftext: Ok(value.reftext),
                terms: Ok(value.terms),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineLiteral {
        location: Result<Option<super::Location>, String>,
        name: Result<super::InlineLiteralName, String>,
        type_: Result<super::InlineLiteralType, String>,
        value: Result<String, String>,
    }
    impl Default for InlineLiteral {
        fn default() -> Self {
            Self {
                location: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl InlineLiteral {
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineLiteralName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineLiteralType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InlineLiteral> for super::InlineLiteral {
        type Error = String;
        fn try_from(value: InlineLiteral) -> Result<Self, String> {
            Ok(Self {
                location: value.location?,
                name: value.name?,
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::InlineLiteral> for InlineLiteral {
        fn from(value: super::InlineLiteral) -> Self {
            Self {
                location: Ok(value.location),
                name: Ok(value.name),
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineRef {
        inlines: Result<super::Inlines, String>,
        location: Result<Option<super::Location>, String>,
        name: Result<super::InlineRefName, String>,
        target: Result<String, String>,
        type_: Result<super::InlineRefType, String>,
        variant: Result<super::InlineRefVariant, String>,
    }
    impl Default for InlineRef {
        fn default() -> Self {
            Self {
                inlines: Err("no value supplied for inlines".to_string()),
                location: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                target: Err("no value supplied for target".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl InlineRef {
        pub fn inlines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.inlines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inlines: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineRefName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineRefType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineRefVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InlineRef> for super::InlineRef {
        type Error = String;
        fn try_from(value: InlineRef) -> Result<Self, String> {
            Ok(Self {
                inlines: value.inlines?,
                location: value.location?,
                name: value.name?,
                target: value.target?,
                type_: value.type_?,
                variant: value.variant?,
            })
        }
    }
    impl From<super::InlineRef> for InlineRef {
        fn from(value: super::InlineRef) -> Self {
            Self {
                inlines: Ok(value.inlines),
                location: Ok(value.location),
                name: Ok(value.name),
                target: Ok(value.target),
                type_: Ok(value.type_),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineSpan {
        form: Result<super::InlineSpanForm, String>,
        inlines: Result<super::Inlines, String>,
        location: Result<Option<super::Location>, String>,
        name: Result<super::InlineSpanName, String>,
        type_: Result<super::InlineSpanType, String>,
        variant: Result<super::InlineSpanVariant, String>,
    }
    impl Default for InlineSpan {
        fn default() -> Self {
            Self {
                form: Err("no value supplied for form".to_string()),
                inlines: Err("no value supplied for inlines".to_string()),
                location: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl InlineSpan {
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineSpanForm>,
            T::Error: std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {}", e));
            self
        }
        pub fn inlines<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.inlines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inlines: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineSpanName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineSpanType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InlineSpanVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InlineSpan> for super::InlineSpan {
        type Error = String;
        fn try_from(value: InlineSpan) -> Result<Self, String> {
            Ok(Self {
                form: value.form?,
                inlines: value.inlines?,
                location: value.location?,
                name: value.name?,
                type_: value.type_?,
                variant: value.variant?,
            })
        }
    }
    impl From<super::InlineSpan> for InlineSpan {
        fn from(value: super::InlineSpan) -> Self {
            Self {
                form: Ok(value.form),
                inlines: Ok(value.inlines),
                location: Ok(value.location),
                name: Ok(value.name),
                type_: Ok(value.type_),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct List {
        id: Result<Option<String>, String>,
        items: Result<Vec<super::ListItem>, String>,
        location: Result<Option<super::Location>, String>,
        marker: Result<String, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::ListName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::ListType, String>,
        variant: Result<super::ListVariant, String>,
    }
    impl Default for List {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                items: Err("no value supplied for items".to_string()),
                location: Ok(Default::default()),
                marker: Err("no value supplied for marker".to_string()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl List {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ListItem>>,
            T::Error: std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn marker<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marker: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<List> for super::List {
        type Error = String;
        fn try_from(value: List) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                items: value.items?,
                location: value.location?,
                marker: value.marker?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
                variant: value.variant?,
            })
        }
    }
    impl From<super::List> for List {
        fn from(value: super::List) -> Self {
            Self {
                id: Ok(value.id),
                items: Ok(value.items),
                location: Ok(value.location),
                marker: Ok(value.marker),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListItem {
        blocks: Result<Option<super::NonSectionBlockBody>, String>,
        id: Result<Option<String>, String>,
        location: Result<Option<super::Location>, String>,
        marker: Result<String, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::ListItemName, String>,
        principal: Result<super::Inlines, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<Option<super::Inlines>, String>,
        type_: Result<super::ListItemType, String>,
    }
    impl Default for ListItem {
        fn default() -> Self {
            Self {
                blocks: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                marker: Err("no value supplied for marker".to_string()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                principal: Err("no value supplied for principal".to_string()),
                reftext: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ListItem {
        pub fn blocks<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonSectionBlockBody>>,
            T::Error: std::fmt::Display,
        {
            self.blocks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocks: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn marker<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marker: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListItemName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn principal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.principal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ListItemType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ListItem> for super::ListItem {
        type Error = String;
        fn try_from(value: ListItem) -> Result<Self, String> {
            Ok(Self {
                blocks: value.blocks?,
                id: value.id?,
                location: value.location?,
                marker: value.marker?,
                metadata: value.metadata?,
                name: value.name?,
                principal: value.principal?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ListItem> for ListItem {
        fn from(value: super::ListItem) -> Self {
            Self {
                blocks: Ok(value.blocks),
                id: Ok(value.id),
                location: Ok(value.location),
                marker: Ok(value.marker),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                principal: Ok(value.principal),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LocationBoundary {
        col: Result<u64, String>,
        file: Result<Vec<String>, String>,
        line: Result<std::num::NonZeroU64, String>,
    }
    impl Default for LocationBoundary {
        fn default() -> Self {
            Self {
                col: Err("no value supplied for col".to_string()),
                file: Ok(Default::default()),
                line: Err("no value supplied for line".to_string()),
            }
        }
    }
    impl LocationBoundary {
        pub fn col<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<u64>,
            T::Error: std::fmt::Display,
        {
            self.col = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for col: {}", e));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn line<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::num::NonZeroU64>,
            T::Error: std::fmt::Display,
        {
            self.line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LocationBoundary> for super::LocationBoundary {
        type Error = String;
        fn try_from(value: LocationBoundary) -> Result<Self, String> {
            Ok(Self {
                col: value.col?,
                file: value.file?,
                line: value.line?,
            })
        }
    }
    impl From<super::LocationBoundary> for LocationBoundary {
        fn from(value: super::LocationBoundary) -> Self {
            Self {
                col: Ok(value.col),
                file: Ok(value.file),
                line: Ok(value.line),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Section {
        blocks: Result<Option<super::SectionBody>, String>,
        id: Result<Option<String>, String>,
        level: Result<u64, String>,
        location: Result<Option<super::Location>, String>,
        metadata: Result<Option<super::BlockMetadata>, String>,
        name: Result<super::SectionName, String>,
        reftext: Result<Option<super::Inlines>, String>,
        title: Result<super::Inlines, String>,
        type_: Result<super::SectionType, String>,
    }
    impl Default for Section {
        fn default() -> Self {
            Self {
                blocks: Ok(Default::default()),
                id: Ok(Default::default()),
                level: Err("no value supplied for level".to_string()),
                location: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                reftext: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Section {
        pub fn blocks<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SectionBody>>,
            T::Error: std::fmt::Display,
        {
            self.blocks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocks: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<u64>,
            T::Error: std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Location>>,
            T::Error: std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BlockMetadata>>,
            T::Error: std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SectionName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reftext<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Inlines>>,
            T::Error: std::fmt::Display,
        {
            self.reftext = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reftext: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Inlines>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SectionType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Section> for super::Section {
        type Error = String;
        fn try_from(value: Section) -> Result<Self, String> {
            Ok(Self {
                blocks: value.blocks?,
                id: value.id?,
                level: value.level?,
                location: value.location?,
                metadata: value.metadata?,
                name: value.name?,
                reftext: value.reftext?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Section> for Section {
        fn from(value: super::Section) -> Self {
            Self {
                blocks: Ok(value.blocks),
                id: Ok(value.id),
                level: Ok(value.level),
                location: Ok(value.location),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                reftext: Ok(value.reftext),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
}