use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Headline, Location, NodeType};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ListVariant {
    Callout,
    Ordered,
    Unordered,
}
impl AnyList {
    fn new_callout_list(marker: String, principal: String) -> Self {
        let mut items = Vec::with_capacity(1);
        items.push(ListItem::new(marker.clone(), Headline::new(&principal)));

        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Callout,
            marker,
            title: None,
            location: None,
            items,
        }
    }

    fn new_ordered_list(marker: String, principal: String) -> Self {
        let mut items = Vec::with_capacity(1);
        items.push(ListItem::new(marker.clone(), Headline::new(&principal)));

        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Ordered,
            marker,
            title: None,
            location: None,
            items,
        }
    }

    fn new_unordered_list(marker: String, principal: String) -> Self {
        let mut items = Vec::with_capacity(1);
        items.push(ListItem::new(marker.clone(), Headline::new(&principal)));

        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Unordered,
            marker,
            title: None,
            location: None,
            items,
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

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::List { marker, items, .. } => {
                let prefix = marker.to_owned() + " ";
                if let Some(principal) = line.clone().strip_prefix(&prefix) {
                    items.push(ListItem::new(marker.to_owned(), Headline::new(principal)));

                    return Ok(());
                }

                if let Some(last) = items.last_mut() {
                    return last.push(line);
                }

                Err("not expected empty items of list".into())
            }
            Self::Dlist { .. } => Err("not implemented".into()),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListItem {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    marker: String,
    principal: Headline,
    blocks: Option<Vec<NonSectionBlockBody>>,
    location: Option<Location>,
}
impl ListItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self {
            name: "listItem".to_owned(),
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: Some(Vec::with_capacity(0)),
            location: None,
        }
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        Err("not implemented".into())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DlistItem {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    marker: String,
    principal: Headline,
    blocks: Option<Vec<NonSectionBlockBody>>,
    location: Option<Location>,
    terms: Vec<Headline>,
}
impl DlistItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self {
            name: "dlistItem".to_owned(),
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: Some(Vec::with_capacity(0)),
            location: None,
            terms: Vec::with_capacity(0),
        }
    }
}

impl Block {
    pub(crate) fn new_callout_list(marker: String, principal: String) -> Self {
        Self::AnyList(AnyList::new_callout_list(marker, principal))
    }

    pub(crate) fn new_ordered_list(marker: String, principal: String) -> Self {
        Self::AnyList(AnyList::new_ordered_list(marker, principal))
    }

    pub(crate) fn new_unordered_list(marker: String, principal: String) -> Self {
        Self::AnyList(AnyList::new_unordered_list(marker, principal))
    }

    fn new_description_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_description_list(marker))
    }
}
