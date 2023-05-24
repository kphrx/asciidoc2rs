use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::block::Block;
use super::block::{Body as NonSectionBlockBody, Metadata};
use super::inline::Inline;
use super::Location;
use super::ASG;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "ASG", into = "ASG")]
pub(crate) struct Section {
    title: Vec<Inline>,
    level: usize,
    metadata: Option<Metadata>,
    blocks: Vec<Body>,
    location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum Body {
    Section(Section),
    Block(NonSectionBlockBody),
}

impl From<ASG> for Section {
    fn from(asg: ASG) -> Section {
        if let ASG::Block(Block::Section(section)) = asg {
            section
        } else {
            panic!()
        }
    }
}
impl From<Section> for ASG {
    fn from(val: Section) -> Self {
        ASG::Block(Block::Section(val))
    }
}
