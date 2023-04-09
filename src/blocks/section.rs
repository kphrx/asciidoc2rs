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
}
