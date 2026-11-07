use serde::{Deserialize, Serialize};

use crate::asg::Inlines;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename = "heading")]
pub struct DiscreteHeading {
    pub(crate) level: usize,
    pub(crate) title: Inlines,
}
