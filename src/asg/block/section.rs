use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, SectionBody};
use crate::asg::{Headline, Location, NodeType};

use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    title: Headline,
    level: usize,
    blocks: Vec<SectionBody>,
    location: Option<Location>,

    #[serde(skip)]
    current_block: Option<Block>,
    #[serde(skip)]
    is_comment: bool,
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
            is_comment: false,
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
        if self.is_comment {
            if line == "////" {
                self.previous_line = "".to_owned();
                self.is_comment = false
            }

            return Ok(());
        }

        if let Some(current) = self.current_block.as_mut() {
            if current.is_delimited_block() {
                self.previous_line = line.to_owned();

                if Some(line.to_owned()) != current.delimiter() {
                    current.push(line)?;

                    return Ok(());
                }

                current.end();
                self.blocks.push(SectionBody::Block(current.clone()));
                self.current_block = None;

                return Ok(());
            }

            if line == "////" {
                self.is_comment = true;

                current.end();
                self.blocks.push(SectionBody::Block(current.clone()));
                self.current_block = None;

                return Ok(());
            }

            match current {
                Block::AnyList(list) => {
                    if self.previous_line == "//" && line == "" {
                        self.previous_line = "".to_owned();

                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;

                        return Ok(());
                    }

                    if line.starts_with("//") && !line.starts_with("///") {
                        if self.previous_line == "" {
                            self.previous_line = "//".to_owned();
                        }

                        return Ok(());
                    }

                    if line == "" {
                        self.previous_line = line.to_owned();

                        return Ok(());
                    }

                    return list.push(line);
                }
                _ => {}
            }

            if line == "" {
                self.previous_line = line.to_owned();
                current.end();
                self.blocks.push(SectionBody::Block(current.clone()));
                self.current_block = None;

                return Ok(());
            }

            self.previous_line = line.to_owned();

            return current.push(line);
        }

        if self.previous_line == "" {
            let marker = match self.level {
                0 => "== ",
                1 => "=== ",
                2 => "==== ",
                3 => "===== ",
                4 => "====== ",
                _ => "",
            };

            if marker != "" {
                if let Some(heading) = line.strip_prefix(marker) {
                    self.previous_line = line.to_owned();
                    let section = Section::new(self.level + 1, heading);
                    self.blocks.push(SectionBody::Section(section));

                    return Ok(());
                }

                if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
                    self.previous_line = line.to_owned();

                    return last.push(line);
                }

                if let Some(_) = line
                    .strip_prefix(&("=".to_owned() + marker))
                    .or(line.strip_prefix(&("==".to_owned() + marker)))
                    .or(line.strip_prefix(&("===".to_owned() + marker)))
                    .or(line.strip_prefix(&("====".to_owned() + marker)))
                {
                    if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
                        self.previous_line = line.to_owned();
                        last.push(line)?;
                    }

                    return Err("cannot skip section level: {}".into());
                }
            }
        }

        if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
            self.previous_line = line.to_owned();

            return last.push(line);
        }

        if line == "" {
            self.previous_line = line.to_owned();

            return Ok(());
        }

        if line == "////" {
            self.is_comment = true;

            return Ok(());
        }

        if line.starts_with("//") && !line.starts_with("///") {
            self.previous_line = "".to_owned();

            return Ok(());
        }

        if line.starts_with("* ") {
            self.previous_line = "".to_owned();
            let mut unordered_list = Block::new_unordered_list("*".to_owned());
            unordered_list.push(line)?;
            self.current_block = Some(unordered_list);

            return Ok(());
        }

        self.previous_line = line.to_owned();
        let paragraph = Block::new_paragraph(line);
        self.current_block = Some(paragraph);

        return Ok(());
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
