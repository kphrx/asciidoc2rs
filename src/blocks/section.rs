use super::{Block, BlockTree};

#[derive(Clone)]
pub(crate) struct Section {
    blocks: Vec<Box<dyn Block>>,
    marker: &'static str,
    title: &'static str,
}

impl Section {
    pub(crate) fn new(marker: &'static str, title: &'static str) -> Self {
        Self {
            blocks: Vec::with_capacity(1),
            marker,
            title,
        }
    }
}

impl Block for Section {
    fn children(&self) -> BlockTree {
        BlockTree::Compound(self.blocks.clone())
    }

    fn push(&mut self, text: &'static str) {}
}
