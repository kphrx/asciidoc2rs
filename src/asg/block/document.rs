use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, Section, SectionBody};
use crate::asg::{Inlines, Location, NodeType};
use crate::{Doctype, LineKind};

use std::collections::HashMap;
use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename = "document")]
pub struct Document {
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(flatten)]
    pub(crate) header: Option<DocumentHeader>,
    #[serde(default)]
    pub(crate) blocks: Vec<SectionBody>,
    location: Option<Location>,

    #[serde(skip)]
    doctype: Doctype,
    #[serde(skip)]
    current_block: Option<Block>,
    #[serde(skip)]
    comment_delimiter: Option<String>,
    #[serde(skip)]
    previous_line: String,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DocumentHeader {
    pub(crate) header: Option<TitleAndAuthors>,
    pub(crate) attributes: HashMap<String, Option<String>>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct TitleAndAuthors {
    pub(crate) title: Option<Inlines>,
    pub(crate) authors: Option<Vec<Author>>,
    pub(crate) location: Option<Location>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Author {
    pub(crate) fullname: Option<String>,
    pub(crate) initials: Option<String>,
    pub(crate) firstname: Option<String>,
    pub(crate) middlename: Option<String>,
    pub(crate) lastname: Option<String>,
    pub(crate) address: Option<String>,
}
impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Self {
            node_type: NodeType::Block,
            header: None,
            blocks: Vec::with_capacity(0),
            location: None,
            doctype,
            current_block: None,
            comment_delimiter: None,
            previous_line: "".to_owned(),
        }
    }

    pub(crate) fn end(&mut self) {
        if let Some(current) = self.current_block.as_mut() {
            current.end();
            self.blocks.push(SectionBody::Block(current.clone()));
            self.current_block = None;
        } else if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
            last.end();
        }
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if self.comment_delimiter.is_some() {
            if matches!(LineKind::parse(line.to_owned()), LineKind::CommentDelimiter(x) if matches!(&self.comment_delimiter, Some(y) if x == *y))
            {
                self.previous_line = "".to_owned();
                self.comment_delimiter = None;
            }

            return Ok(());
        }

        if let Some(current) = self.current_block.as_mut() {
            if current.is_delimited_block() {
                self.previous_line = line.to_owned();

                if LineKind::parse(line.to_owned()).block_delimiter() != current.delimiter() {
                    current.push(line)?;

                    return Ok(());
                }

                current.end();
                self.blocks.push(SectionBody::Block(current.clone()));
                self.current_block = None;

                return Ok(());
            }

            if let Block::AnyList(_list) = current {
                match LineKind::parse(line.to_owned()) {
                    LineKind::Empty => {
                        if self.previous_line == "//" {
                            self.previous_line = "".to_owned();
                            current.end();
                            self.blocks.push(SectionBody::Block(current.clone()));
                            self.current_block = None;

                            return Ok(());
                        }

                        self.previous_line = "".to_owned();

                        return Ok(());
                    }
                    LineKind::CommentMarker => {
                        if self.previous_line.is_empty() {
                            self.previous_line = "//".to_owned();
                        }

                        return Ok(());
                    }
                    _ => {
                        if !self.previous_line.is_empty() {
                            self.previous_line = line.to_owned();
                            current.push(line)?;

                            return Ok(());
                        }

                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;
                    }
                }
            } else {
                match LineKind::parse(line.to_owned()) {
                    LineKind::Empty => {
                        self.previous_line = "".to_owned();
                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;

                        return Ok(());
                    }
                    LineKind::CommentMarker => {
                        self.previous_line = "".to_owned();

                        return Ok(());
                    }
                    LineKind::CommentDelimiter(delimiter) => {
                        self.comment_delimiter = Some(delimiter);

                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;

                        return Ok(());
                    }
                    _ => {
                        self.previous_line = line.to_owned();

                        return current.push(line);
                    }
                }
            }
        }

        if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
            if let LineKind::HeadingMarker { level, title } = LineKind::parse(line.to_owned()) {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();

                    return last.push(line);
                }

                if level == 0 && !matches!(self.doctype, Doctype::Book) {
                    self.previous_line = line.to_owned();
                    let section = Section::new(0, &title);
                    self.blocks.push(SectionBody::Section(Box::new(section)));

                    return Err("level 0 sections can only be used when doctype is book".into());
                }

                if level <= last.level {
                    self.previous_line = line.to_owned();
                    let section = Section::new(level, &title);
                    self.blocks.push(SectionBody::Section(Box::new(section)));

                    return Ok(());
                }
            }

            self.previous_line = line.to_owned();

            return last.push(line);
        }

        self.parse_preamble(line)
    }

    fn parse_preamble(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match LineKind::parse(line.to_owned()) {
            LineKind::Empty => {
                self.previous_line = "".to_owned();

                Ok(())
            }
            LineKind::CommentMarker => {
                self.previous_line = "".to_owned();

                Ok(())
            }
            LineKind::CommentDelimiter(delimiter) => {
                self.comment_delimiter = Some(delimiter);

                Ok(())
            }
            LineKind::HeadingMarker { level, title } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                if level > 1 {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Err("cannot skip section level".into());
                }

                if level == 0 && !matches!(self.doctype, Doctype::Book) {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Err("level 0 sections can only be used when doctype is book".into());
                }

                self.previous_line = line.to_owned();
                let section = Section::new(level, &title);
                self.blocks.push(SectionBody::Section(Box::new(section)));

                Ok(())
            }
            LineKind::UnorderedListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let unordered_list = Block::new_unordered_list(marker, principal);
                self.current_block = Some(unordered_list);

                Ok(())
            }
            LineKind::OrderedListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(marker, principal);
                self.current_block = Some(ordered_list);

                Ok(())
            }
            LineKind::OffsetOrderedListMarker { offset, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(format!("{}.", offset), principal);
                self.current_block = Some(ordered_list);

                Ok(())
            }
            LineKind::CalloutListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let callout_list = Block::new_callout_list(marker, principal);
                self.current_block = Some(callout_list);

                Ok(())
            }
            LineKind::DescriptionListMarker {
                marker,
                term,
                principal,
            } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let description_list = Block::new_description_list(marker, term, principal);
                self.current_block = Some(description_list);

                Ok(())
            }
            LineKind::Unknown => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                Ok(())
            }
            _ => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                Ok(())
            }
        }
    }
}
