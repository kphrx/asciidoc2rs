use dyn_clone::{clone_trait_object, DynClone};

pub trait CompoundBlock: Block {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Block>>>;
}

pub trait Block: DynClone {}
clone_trait_object!(Block);

#[derive(Clone)]
pub struct Document {
    blocks: Vec<Box<dyn Block>>,
}

impl Block for Document {}

impl CompoundBlock for Document {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Block>>> {
        Box::new(self.blocks.clone().into_iter())
    }
}

pub struct Parser<'input> {
    text: &'input str,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        Self { text }
    }

    pub fn parse(self) -> Vec<Document> {
        let documents = Vec::with_capacity(0);

        documents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_block() {
        let parser = Parser::new("Paragraphs don't require any special markup in AsciiDoc.\nA paragraph is just one or more lines of consecutive text.\n\nTo begin a new paragraph, separate it by at least one empty line from the previous paragraph or block.");

        let documents = parser.parse();
        assert_eq!(1, documents.len());

        let document = documents[0].clone();
        assert_eq!(2, document.iter().count());
    }
}
