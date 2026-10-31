use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, NonSectionBlockBody};
use crate::asg::{Inlines, Location, NodeType, NonEmptyVec};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum AnyList {
    List {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: ListVariant,
        marker: String,
        title: Option<Inlines>,
        location: Option<Location>,
        items: NonEmptyVec<ListItem>,
    },
    Dlist {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        title: Option<Inlines>,
        location: Option<Location>,
        items: NonEmptyVec<DlistItem>,

        #[serde(skip)]
        current_terms: Vec<String>,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ListVariant {
    Callout,
    Ordered,
    Unordered,
}
impl AnyList {
    fn new_callout_list(marker: String, principal: String) -> Self {
        let items = NonEmptyVec::new(ListItem::new(
            marker.clone(),
            Inlines::new_with_line(&principal),
        ));

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
        let items = NonEmptyVec::new(ListItem::new(
            marker.clone(),
            Inlines::new_with_line(&principal),
        ));

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
        let items = NonEmptyVec::new(ListItem::new(
            marker.clone(),
            Inlines::new_with_line(&principal),
        ));

        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Unordered,
            marker,
            title: None,
            location: None,
            items,
        }
    }

    fn new_description_list(marker: String, term: String, principal: Option<String>) -> Self {
        let mut items = NonEmptyVec::with_capacity(1);
        let current_terms = vec![term.clone()];
        if let Some(principal) = principal {
            items.push(DlistItem::new(
                marker.clone(),
                current_terms.to_owned(),
                Inlines::new_with_line(&principal),
            ));
        }

        Self::Dlist {
            node_type: NodeType::Block,
            marker,
            title: None,
            location: None,
            items,
            current_terms,
        }
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::List { marker, items, .. } => {
                let prefix = marker.to_owned() + " ";
                if let Some(principal) = line.trim_start_matches(['\t', ' ']).strip_prefix(&prefix)
                {
                    items.push(ListItem::new(
                        marker.to_owned(),
                        Inlines::new_with_line(principal),
                    ));

                    return Ok(());
                }

                if let Some(last) = items.last_mut() {
                    return last.push(line);
                }

                Err("not expected empty items of list".into())
            }
            Self::Dlist {
                marker,
                items,
                current_terms,
                ..
            } => {
                let delimiter = marker.to_owned() + " ";

                if !current_terms.is_empty() {
                    if let Some((term, principal)) = line.split_once(&delimiter) {
                        current_terms.push(term.to_owned());
                        let principal = principal.trim_start_matches(' ');
                        items.push(DlistItem::new(
                            marker.clone(),
                            current_terms.to_owned(),
                            Inlines::new_with_line(principal),
                        ));
                        current_terms.clear();

                        return Ok(());
                    }

                    if let Some((term, "")) = line.split_once(&marker.to_owned()) {
                        current_terms.push(term.to_owned());

                        return Ok(());
                    }

                    let principal = line.trim_start_matches(['\t', ' ']);
                    items.push(DlistItem::new(
                        marker.clone(),
                        current_terms.to_owned(),
                        Inlines::new_with_line(principal),
                    ));
                    current_terms.clear();

                    return Ok(());
                }

                if let Some((term, principal)) = line.split_once(&delimiter) {
                    let principal = principal.trim_start_matches(' ');
                    items.push(DlistItem::new(
                        marker.clone(),
                        vec![term.to_owned()],
                        Inlines::new_with_line(principal),
                    ));

                    return Ok(());
                }

                if let Some((term, "")) = line.split_once(&marker.to_owned()) {
                    current_terms.push(term.to_owned());

                    return Ok(());
                }

                if let Some(last) = items.last_mut() {
                    return last.push(line);
                }

                Err("not expected empty items of list".into())
            }
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
    principal: Inlines,
    blocks: Option<Vec<NonSectionBlockBody>>,
    location: Option<Location>,
}
impl ListItem {
    fn new(marker: String, principal: Inlines) -> Self {
        Self {
            name: "listItem".to_owned(),
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: Some(Vec::with_capacity(0)),
            location: None,
        }
    }

    pub(crate) fn push(&mut self, _line: &str) -> Result<(), Box<dyn Error>> {
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
    principal: Inlines,
    blocks: Option<Vec<NonSectionBlockBody>>,
    location: Option<Location>,
    terms: NonEmptyVec<Inlines>,
}
impl DlistItem {
    fn new(marker: String, _terms: Vec<String>, principal: Inlines) -> Self {
        Self {
            name: "dlistItem".to_owned(),
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: Some(Vec::with_capacity(0)),
            location: None,
            terms: NonEmptyVec::with_capacity(1),
        }
    }

    pub(crate) fn push(&mut self, _line: &str) -> Result<(), Box<dyn Error>> {
        Err("not implemented".into())
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

    pub(crate) fn new_description_list(
        marker: String,
        term: String,
        principal: Option<String>,
    ) -> Self {
        Self::AnyList(AnyList::new_description_list(marker, term, principal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unordered_list() {
        let mut list = Block::new_unordered_list("*".to_owned(), "item 1".to_owned());

        list.push("* item 2");
        list.push("  * item 3");

        let Block::AnyList(AnyList::List {
            variant,
            marker,
            items,
            ..
        }) = list
        else {
            panic!("not expected")
        };
        let mut items = items.to_owned();

        assert!(matches!(variant, ListVariant::Unordered));
        assert_eq!("*", marker);
        assert_eq!(3, items.len());

        let item_3 = items.pop().unwrap();
        let item_2 = items.pop().unwrap();
        let item_1 = items.pop().unwrap();

        assert_eq!(Inlines::new_with_line("item 1"), item_1.principal);
        assert_eq!(Inlines::new_with_line("item 2"), item_2.principal);
        assert_eq!(Inlines::new_with_line("item 3"), item_3.principal);
    }

    #[test]
    fn description_list() {
        let mut list = Block::new_description_list(
            "::".to_owned(),
            "term 1".to_owned(),
            Some("description 1".to_owned()),
        );

        list.push("term 2::");
        list.push("  description 2");
        list.push("term 3::");
        list.push("    term 4:: description 3-4");

        let Block::AnyList(AnyList::Dlist { marker, items, .. }) = list else {
            panic!("not expected")
        };
        let mut items = items.to_owned();

        assert_eq!("::", marker);
        assert_eq!(3, items.len());

        let item_3 = items.pop().unwrap();
        let item_2 = items.pop().unwrap();
        let item_1 = items.pop().unwrap();

        assert_eq!(Inlines::new_with_line("description 1"), item_1.principal);
        assert_eq!(Inlines::new_with_line("description 2"), item_2.principal);
        assert_eq!(Inlines::new_with_line("description 3-4"), item_3.principal);
    }
}
