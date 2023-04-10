use super::{Block, BlockTree};

#[derive(Clone)]
pub(crate) struct Section<'line> {
    blocks: Vec<Box<dyn Block<'line> + 'line>>,
    level: usize,
    title: &'line str,
}

impl<'line> Section<'line> {
    pub(crate) fn new(level: usize, title: &'line str) -> Self {
        Self {
            blocks: Vec::with_capacity(1),
            level,
            title,
        }
    }
}

impl<'line> Block<'line> for Section<'line> {
    fn children(&self) -> BlockTree<'line> {
        BlockTree::Compound(self.blocks.clone())
    }

    fn push(&mut self, line: &'line str) {}

    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_block() {
        let mut section = Section::new(0, "");

        for line in "== Level 1 Section Title\n\n=== Level 2 Section Title\n\n==== Level 3 Section Title\n\n===== Level 4 Section Title\n\n====== Level 5 Section Title\n\n== Another Level 1 Section Title".lines() {
            section.push(line);
        }
        section.close();

        let BlockTree::Compound(blocks) = section.children() else { panic!() };
        assert_eq!(2, blocks.len());
    }

    #[test]
    fn nested_section_block() {
        let mut section = Section::new(0, "");

        for line in "== First Section\n\nContent of first section\n\n=== Nested Section\n\nContent of nested section\n\n== Second Section\n\nContent of second section".lines() {
            section.push(line);
        }
        section.close();

        let BlockTree::Compound(blocks) = section.children() else { panic!() };
        assert_eq!(2, blocks.len());
    }

    #[test]
    #[should_panic]
    fn illegal_level_skipped_section_block() {
        let mut section = Section::new(0, "");

        for line in "== First Section\n\n==== Illegal Nested Section (violates rule #2)".lines() {
            section.push(line);
        }
        section.close();
    }

    #[test]
    fn paragraph_block() {
        let mut section = Section::new(1, "Paragraph blocks");

        for line in "Paragraphs don't require any special markup in AsciiDoc.\nA paragraph is just one or more lines of consecutive text.\n\nTo begin a new paragraph, separate it by at least one empty line from the previous paragraph or block.".lines() {
            section.push(line);
        }
        section.close();

        let BlockTree::Compound(blocks) = section.children() else { panic!() };
        assert_eq!(2, blocks.len());
    }
}
