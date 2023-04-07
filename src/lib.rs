use std::collections::HashMap;

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
    body_started: bool,
    metadata: DocumentMetadata,
}

#[derive(Clone)]
struct DocumentMetadata {
    title: Option<&'static str>,
    implicit_line: bool,
    current_attr: Option<&'static str>,
    current_value: Option<&'static str>,
    authors: Option<Vec<Author>>,
    revision: Option<Revision>,
    custom_attributes: HashMap<&'static str, &'static str>,
}

impl DocumentMetadata {
    fn set_title(&mut self, title: &'static str) {
        self.title = Some(title);
        self.implicit_line = true;
    }

    fn has_title(&self) -> bool {
        self.title.is_some()
    }

    fn is_implicit_line(&mut self, text: &'static str) -> bool {
        if self.implicit_line {
            self.implicit_line = if self.authors.is_none() {
                self.parse_authors_line(text)
            } else if self.revision.is_none() {
                self.parse_revision_line(text)
            } else {
                false
            }
        }

        self.implicit_line
    }

    fn parse_authors_line(&mut self, text: &'static str) -> bool {
        let split_authors: Vec<&str> = text.split_terminator(';').collect();
        let mut authors: Vec<Author> = Vec::with_capacity(split_authors.len());
        for author in split_authors {
            let (name, email) = match author.split_once('<') {
                None => (author, None),
                Some((a, e)) => (a, Some(e)),
            };

            if let Some(e) = email.and_then(|e| e.strip_suffix('>')) {
                authors.push(Author::new_with_email(name, e))
            } else if email.is_none() {
                authors.push(Author::new_name(name))
            } else {
                return false;
            }
        }

        self.authors = Some(authors);

        true
    }

    fn parse_revision_line(&self, text: &'static str) -> bool {
        false
    }

    fn is_attribute_line(&mut self, text: &'static str) -> bool {
        if !self.implicit_line {
            if self.current_attr.is_some() {
                self.parse_wrapped_attr(text);
                true
            } else if text.starts_with(':') {
                self.parse_attr(text)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn parse_attr(&mut self, text: &'static str) -> bool {
        if let Some(unset_attr) = text.strip_prefix(":!").and_then(|a| a.strip_suffix(":")) {
            if !unset_attr.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || unset_attr
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :!{}:", unset_attr);
            }

            self.custom_attributes.remove(unset_attr);

            true
        } else if let Some((attr_name, attr_value)) =
            text.strip_prefix(":").and_then(|a| a.split_once(": "))
        {
            if !attr_name.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || attr_name
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :{}:", attr_name);
            }

            if let Some(wrap_value) = attr_value.strip_suffix(" + \\") {
                wrap_value.to_owned().push_str("\n");
                self.current_value = Some(wrap_value)
            } else if let Some(wrap_value) = attr_value.strip_suffix(" \\") {
                wrap_value.to_owned().push_str(" ");
                self.current_value = Some(wrap_value)
            } else {
                self.custom_attributes.insert(attr_name, attr_value);

                return true
            }

            self.current_attr = Some(attr_name);

            true
        } else if let Some(unset_attr) = text.strip_prefix(":").and_then(|a| a.strip_suffix("!:")) {
            if !unset_attr.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || unset_attr
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :{}!:", unset_attr);
            }

            self.custom_attributes.remove(unset_attr);

            true
        } else if let Some(set_bool_attr) = text.strip_prefix(":").and_then(|a| a.strip_suffix(":"))
        {
            if !set_bool_attr.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || set_bool_attr
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :{}:", set_bool_attr);
            }

            self.custom_attributes.insert(set_bool_attr, "");

            true
        } else {
            false
        }
    }

    fn parse_wrapped_attr(&mut self, text: &'static str) {
        if let Some(wrap_value) = text.strip_suffix(" + \\") {
            wrap_value.to_owned().push_str("\n");
            self.current_value = self.current_value.map(|s| {
                s.to_owned().push_str(wrap_value);

                s
            });

            return;
        }

        if let Some(wrap_value) = text.strip_suffix(" \\") {
            wrap_value.to_owned().push_str(" ");
            self.current_value = self.current_value.map(|s| {
                s.to_owned().push_str(wrap_value);

                s
            });

            return;
        }

        let attr_name = self.current_attr.unwrap();
        let attr_value = self.current_value.unwrap();
        attr_value.to_owned().push_str(text);

        self.current_attr = None;
        self.current_value = None;

        self.custom_attributes.insert(attr_name, attr_value);
    }
}

impl Default for DocumentMetadata {
    fn default() -> DocumentMetadata {
        DocumentMetadata {
            title: None,
            implicit_line: false,
            current_attr: None,
            current_value: None,
            authors: None,
            revision: None,
            custom_attributes: HashMap::new(),
        }
    }
}

impl Document {
    fn new(doctype: Doctype) -> Self {
        let blocks = Vec::with_capacity(1);

        Self {
            blocks,
            doctype,
            body_started: false,
            metadata: Default::default(),
        }
    }

    fn title(self) -> Option<&'static str> {
        match self.doctype {
            Doctype::Manpage => {
                if self.metadata.has_title() {
                    self.metadata.title
                } else {
                    panic!("require document title");
                }
            }
            _ => self.metadata.title,
        }
    }

    fn authors(self) -> Vec<Author> {
        self.metadata.authors.unwrap_or(Vec::with_capacity(0))
    }

    fn revision(self) -> Option<Revision> {
        self.metadata.revision
    }

    fn description(self) -> Option<&'static str> {
        self.metadata
            .custom_attributes
            .get("description")
            .map(|s| s.trim())
    }
}

impl Block for Document {
    fn push(&mut self, text: &'static str) {
        if self.body_started
            && !self.metadata.has_title()
            && matches!(self.doctype, Doctype::Manpage)
        {
            panic!("require document title for doctype-manpage");
        }

        if !self.body_started && !self.metadata.has_title() && text == "" {
            return;
        }

        if !self.body_started && text == "" {
            self.body_started = true;
            return;
        }

        if let Some(level0_heading) = text.strip_prefix("= ") {
            if !self.body_started && !self.metadata.has_title() {
                self.metadata.set_title(level0_heading);

                return;
            }

            if !self.body_started {
                panic!("invalid document header: {}", text);
            }

            panic!("Illegal Level 0 Section");
        }

        if !self.body_started {
            if self.metadata.is_implicit_line(text) {
                return;
            }

            if self.metadata.is_attribute_line(text) {
                return;
            }

            if self.metadata.has_title() {
                panic!("invalid document header: {}", text);
            }
        }

        self.body_started = true;
    }
}

impl CompoundBlock for Document {
    fn iter(&self) -> Box<dyn Iterator<Item = Box<dyn Block>>> {
        Box::new(self.blocks.clone().into_iter())
    }
}

#[derive(Clone)]
pub struct Author {
    name: &'static str,
    email: Option<&'static str>,
}

impl Author {
    fn new_name(name: &'static str) -> Self {
        Self::new(name, None)
    }

    fn new_with_email(name: &'static str, email: &'static str) -> Self {
        Self::new(name, Some(email))
    }

    fn new(name: &'static str, email: Option<&'static str>) -> Self {
        Author {
            name: name.trim(),
            email,
        }
    }
}

#[derive(Clone)]
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
        assert_eq!(Some("kismet@asciidoctor.org"), author.email);
        assert_eq!(
            Some("The document's description."),
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
        Parser::new("= Document Title\n\n= Illegal Level 0 Section (violates rule #1)\n\n== First Section\n\n==== Illegal Nested Section (violates rule #2)").parse();
    }

    #[test]
    fn nested_section_block() {
        let parser = Parser::new("== First Section\n\nContent of first section\n\n=== Nested Section\n\nContent of nested section\n\n== Second Section\n\nContent of second section");

        let document = parser.parse();
        assert_eq!(2, document.iter().count());
        assert_eq!(None, document.clone().title());
    }
}
