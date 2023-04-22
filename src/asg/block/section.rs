use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, LineKind, SectionBody};
use crate::asg::{Headline, Location, NodeType};

use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    title: Headline,
    pub(crate) level: usize,
    blocks: Vec<SectionBody>,
    location: Option<Location>,

    #[serde(skip)]
    current_block: Option<Block>,
    #[serde(skip)]
    comment_delimiter: Option<String>,
    #[serde(skip)]
    previous_line: String,
}
impl Section {
    pub(crate) fn new(level: usize, heading: &str) -> Self {
        Self {
            name: "section".to_owned(),
            node_type: NodeType::Block,
            title: Headline::new(heading),
            level,
            blocks: Vec::with_capacity(0),
            location: None,
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
            if matches!(LineKind::parse(line.to_owned()), LineKind::CommentDelimiter(x) if matches!(&self.comment_delimiter, Some(y) if x == y.to_owned()))
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

            if let Block::AnyList(list) = current {
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
                        if self.previous_line == "" {
                            self.previous_line = "//".to_owned();
                        }

                        return Ok(());
                    }
                    _ => {
                        if self.previous_line != "" {
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
            match LineKind::parse(line.to_owned()) {
                LineKind::HeadingMarker { level, title } => {
                    if self.previous_line != "" {
                        self.previous_line = line.to_owned();

                        return last.push(line);
                    }

                    if level == last.level {
                        self.previous_line = line.to_owned();
                        let section = Section::new(level, &title);
                        self.blocks.push(SectionBody::Section(section));

                        return Ok(());
                    }
                }
                _ => {}
            }

            self.previous_line = line.to_owned();

            return last.push(line);
        }

        return self.parse_content(line);
    }

    fn parse_content(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match LineKind::parse(line.to_owned()) {
            LineKind::Empty => {
                self.previous_line = "".to_owned();

                return Ok(());
            }
            LineKind::CommentMarker => {
                self.previous_line = "".to_owned();

                return Ok(());
            }
            LineKind::CommentDelimiter(delimiter) => {
                self.comment_delimiter = Some(delimiter);

                return Ok(());
            }
            LineKind::HeadingMarker { level, title } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                if level == self.level + 1 {
                    self.previous_line = line.to_owned();
                    let section = Section::new(level, &title);
                    self.blocks.push(SectionBody::Section(section));

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                return Err("cannot skip section level".into());
            }
            LineKind::UnorderedListMarker { marker, principal } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let unordered_list = Block::new_unordered_list(marker, principal);
                self.current_block = Some(unordered_list);

                return Ok(());
            }
            LineKind::OrderedListMarker { marker, principal } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(marker, principal);
                self.current_block = Some(ordered_list);

                return Ok(());
            }
            LineKind::OffsetOrderedListMarker { offset, principal } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(format!("{}.", offset), principal);
                self.current_block = Some(ordered_list);

                return Ok(());
            }
            LineKind::CalloutListMarker { marker, principal } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let callout_list = Block::new_callout_list(marker, principal);
                self.current_block = Some(callout_list);

                return Ok(());
            }
            LineKind::DescriptionListMarker {
                marker,
                term,
                principal,
            } => {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let description_list = Block::new_description_list(marker, term, principal);
                self.current_block = Some(description_list);

                return Ok(());
            }
            LineKind::Unknown => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                return Ok(());
            }
            _ => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::asg::{block::BlockLeaf, inlines::Inline};

    use super::*;

    fn parse(text: &'static str) -> Result<Section, Box<dyn Error>> {
        let mut section = Section::new(0, "");

        for line in text.lines() {
            section.push(line)?;
        }
        section.end();

        Ok(section)
    }

    #[test]
    fn section_block() {
        let section = parse("== Level 1 Section Title\n\n=== Level 2 Section Title\n\n==== Level 3 Section Title\n\n===== Level 4 Section Title\n\n====== Level 5 Section Title\n\n== Another Level 1 Section Title").unwrap();

        assert_eq!(2, section.blocks.len());

        let Some(SectionBody::Section(level1_section)) = section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Level 1 Section Title").heading(),
            level1_section.title.heading()
        );
        assert_eq!(1, level1_section.blocks.len());
        let Some(SectionBody::Section(level2_section)) = level1_section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Level 2 Section Title").heading(),
            level2_section.title.heading()
        );
        assert_eq!(1, level2_section.blocks.len());
        let Some(SectionBody::Section(level3_section)) = level2_section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Level 3 Section Title").heading(),
            level3_section.title.heading()
        );
        assert_eq!(1, level3_section.blocks.len());
        let Some(SectionBody::Section(level4_section)) = level3_section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Level 4 Section Title").heading(),
            level4_section.title.heading()
        );
        assert_eq!(1, level4_section.blocks.len());
        let Some(SectionBody::Section(level5_section)) = level4_section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Level 5 Section Title").heading(),
            level5_section.title.heading()
        );
        assert_eq!(0, level5_section.blocks.len());

        let Some(SectionBody::Section(another_level1_section)) = section.blocks.last() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Another Level 1 Section Title").heading(),
            another_level1_section.title.heading()
        );
        assert_eq!(0, another_level1_section.blocks.len());
    }

    #[test]
    fn nested_section_block() {
        let section = parse("== First Section\n\nContent of first section\n\n=== Nested Section\n\nContent of nested section\n\n== Second Section\n\nContent of second section").unwrap();

        assert_eq!(2, section.blocks.len());

        let Some(SectionBody::Section(first_section)) = section.blocks.first() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("First Section").heading(),
            first_section.title.heading()
        );
        assert_eq!(2, first_section.blocks.len());
        let Some(SectionBody::Block(Block::BlockLeaf(BlockLeaf::Paragraph(content_of_first_section)))) =
            first_section.blocks.first() else {
                panic!("cannot call");
            };
        assert_eq!(
            Inline::new("Content of first section"),
            content_of_first_section.inlines()
        );

        let Some(SectionBody::Section(nested_section)) = first_section.blocks.last() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Nested Section").heading(),
            nested_section.title.heading()
        );
        assert_eq!(1, nested_section.blocks.len());
        let Some(SectionBody::Block(Block::BlockLeaf(BlockLeaf::Paragraph(content_of_nested_section)))) =
            nested_section.blocks.first() else {
                panic!("cannot call");
            };
        assert_eq!(
            Inline::new("Content of nested section"),
            content_of_nested_section.inlines()
        );

        let Some(SectionBody::Section(second_section)) = section.blocks.last() else {
            panic!("cannot call");
        };
        assert_eq!(
            Headline::new("Second Section").heading(),
            second_section.title.heading()
        );
        assert_eq!(1, second_section.blocks.len());
        let Some(SectionBody::Block(Block::BlockLeaf(BlockLeaf::Paragraph(content_of_second_section)))) =
            second_section.blocks.first() else {
                panic!("cannot call");
            };
        assert_eq!(
            Inline::new("Content of second section"),
            content_of_second_section.inlines()
        );
    }

    #[test]
    #[should_panic]
    fn illegal_level_skipped_section_block() {
        parse("== First Section\n\n==== Illegal Nested Section (violates rule #2)").unwrap();
    }
}
