use dyn_clone::{clone_trait_object, DynClone};

pub trait CompoundBlock: Block {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Block>>>;
}

pub trait Block: DynClone {
    fn push(&mut self, text: &'static str);
}
clone_trait_object!(Block);

pub trait Inline: DynClone {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Inline>>>;
}
clone_trait_object!(Inline);

#[derive(Clone)]
pub struct Document {
    blocks: Vec<Box<dyn Block>>,
    doctype: Doctype,
}

impl Document {
    fn new(doctype: Doctype) -> Self {
        let blocks = Vec::with_capacity(1);

        Document { blocks, doctype }
    }

    fn title(self) -> Option<&'static str> {
        match self.doctype {
            Doctype::Manpage => panic!("require document title"),
            _ => false,
        };

        None
    }

    fn authors(self) -> Vec<Author> {
        Vec::with_capacity(0)
    }

    fn revision(self) -> Option<Revision> {
        None
    }

    fn description(self) -> Option<String> {
        None
    }
}

impl Block for Document {
    fn push(&mut self, text: &'static str) {
    }
}

impl CompoundBlock for Document {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Block>>> {
        Box::new(self.blocks.clone().into_iter())
    }
}

pub struct Author {
    name: String,
    email: String,
}

pub struct Revision {
    number: String,
    date: String,
    remark: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Doctype {
    Article,
    Book,
    Manpage,
}

pub struct Parser {
    text: &'static str,
    doctype: Doctype,
}

impl Parser {
    pub fn new(text: &'static str) -> Self {
        Self::new_with_doctype(text, Doctype::Article)
    }

    pub fn new_with_doctype(text: &'static str, doctype: Doctype) -> Self {
        Self { text, doctype }
    }

    pub fn parse(self) -> Document {
        let mut document = Document::new(self.doctype);

        let mut is_comment = false;

        for line in self.text.lines() {
            if line == "////" {
                if is_comment {
                    is_comment = false
                } else {
                    is_comment = true
                }

                continue;
            }
            if is_comment {
                continue;
            }
            if line.starts_with("//") && !line.starts_with("///") {
                continue;
            }

            document.push(line);
        }

        document
    }

    pub fn parse_inline(self) -> Vec<Box<dyn Inline>> {
        Vec::with_capacity(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_header() {
        let parser = Parser::new("// this comment line is ignored\n= Document Title\nKismet R. Lee <kismet@asciidoctor.org>\n:description: The document's description.\n:sectanchors:\n:url-repo: https://my-git-repo.com\n\nThe document body starts here.");

        let document = parser.parse();
        assert_eq!(Some("Document Title"), document.clone().title());
        let authors = document.clone().authors();
        assert_eq!(1, authors.len());
        let author = &authors[0];
        assert_eq!("Kismet R. Lee", author.name);
        assert_eq!("kismet@asciidoctor.org", author.email);
        assert_eq!(
            Some("The document's description.".to_string()),
            document.clone().description()
        );
    }

    #[test]
    fn paragraph_block() {
        let parser = Parser::new("Paragraphs don't require any special markup in AsciiDoc.\nA paragraph is just one or more lines of consecutive text.\n\nTo begin a new paragraph, separate it by at least one empty line from the previous paragraph or block.");

        let document = parser.parse();
        assert_eq!(2, document.iter().count());
        assert_eq!(None, document.clone().title());

    }

    #[test]
    fn section_block() {
        let parser = Parser::new("= Document Title (Level 0)\n\n== Level 1 Section Title\n\n=== Level 2 Section Title\n\n==== Level 3 Section Title\n\n===== Level 4 Section Title\n\n====== Level 5 Section Title\n\n== Another Level 1 Section Title");

        let document = parser.parse();
        assert_eq!(1, document.iter().count());
        assert_eq!(Some("Document Title (Level 0)"), document.clone().title());
    }

    #[test]
    #[should_panic]
    fn illegal_section_block() {
        Parser::new("= Document Title\n\n= Illegal Level 0 Section (violates rule #1)\n\n== First Section\n\n==== Illegal Nested Section (violates rule #2)");
    }

    #[test]
    fn nested_section_block() {
        let parser = Parser::new("== First Section\n\nContent of first section\n\n=== Nested Section\n\nContent of nested section\n\n== Second Section\n\nContent of second section");

        let document = parser.parse();
        assert_eq!(2, document.iter().count());
        assert_eq!(None, document.clone().title());
    }
}
