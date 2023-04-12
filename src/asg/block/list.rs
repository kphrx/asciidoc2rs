use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Headline, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum AnyList {
    List {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: ListVariant,
        marker: String,
        title: Option<Headline>,
        location: Option<Location>,
        items: Vec<ListItem>,
    },
    Dlist {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        title: Option<Headline>,
        location: Option<Location>,
        items: Vec<DlistItem>,
    },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ListVariant {
    Callout,
    Ordered,
    Unordered,
}
impl AnyList {
    fn new_callout_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Callout,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_ordered_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Ordered,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_unordered_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Unordered,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_description_list(marker: String) -> Self {
        Self::Dlist {
            node_type: NodeType::Block,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum ListItem {
    ListItem {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        principal: Headline,
        blocks: Option<Vec<NonSectionBlockBody>>,
        location: Option<Location>,
    },
}
impl ListItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self::ListItem {
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: None,
            location: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum DlistItem {
    DlistItem {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        principal: Headline,
        blocks: Option<Vec<NonSectionBlockBody>>,
        location: Option<Location>,
        terms: Vec<Headline>,
    },
}
impl DlistItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self::DlistItem {
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: None,
            location: None,
            terms: Vec::with_capacity(0),
        }
    }
}

impl Block {
    fn new_callout_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_callout_list(marker))
    }

    fn new_ordered_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_ordered_list(marker))
    }

    fn new_unordered_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_unordered_list(marker))
    }

    fn new_description_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_description_list(marker))
    }
}
