use std::collections::HashMap;

use super::{Block, BlockTree, Doctype, Section};

#[derive(Clone)]
pub(crate) struct Document<'line> {
    blocks: Vec<Box<dyn Block<'line> + 'line>>,
    doctype: Doctype,
    body_started: bool,
    is_comment: bool,
    previous_line: &'line str,
    opened_block: Option<Box<dyn Block<'line> + 'line>>,
    metadata: DocumentMetadata<'line>,
}

impl<'line> Document<'line> {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Self {
            blocks: Vec::with_capacity(1),
            doctype,
            body_started: false,
            is_comment: false,
            previous_line: "",
            opened_block: None,
            metadata: Default::default(),
        }
    }

    fn title(self) -> Option<&'line str> {
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

    fn authors(self) -> Vec<Author<'line>> {
        self.metadata.authors.unwrap_or(Vec::with_capacity(0))
    }

    fn revision(self) -> Option<Revision> {
        self.metadata.revision
    }

    fn description(self) -> Option<&'line str> {
        self.metadata.get_value("description").map(|s| s.trim())
    }

    fn parse_header(&mut self, line: &'line str) {
        if line == "" {
            if self.metadata.has_title() {
                self.body_started = true;
            }

            return;
        }

        if !self.metadata.has_title() {
            if line == "////" {
                if self.is_comment {
                    self.is_comment = false
                } else {
                    self.is_comment = true
                }

                return;
            }
            if self.is_comment {
                return;
            }
            if line.starts_with("//") && !line.starts_with("///") {
                return;
            }
        }

        if self.metadata.parse_attribute_line(line) {
            return;
        }

        if self.metadata.parse_implicit_line(line) {
            return;
        }

        if let Some(document_title) = line.strip_prefix("= ") {
            self.metadata.set_title(document_title);

            return;
        }

        if self.metadata.has_title() {
            panic!("invalid document header: {}", line);
        }

        self.body_started = true;

        self.parse_body(line)
    }

    fn parse_body(&mut self, line: &'line str) {
        if !self.metadata.has_title() && matches!(self.doctype, Doctype::Manpage) {
            panic!("require document title for doctype-manpage");
        }

        let previous_line = self.previous_line;
        self.previous_line = line;

        if previous_line == "" {
            if let Some(heading) = line.strip_prefix("= ") {
                if matches!(self.doctype, Doctype::Book) {
                    self.close();
                    self.opened_block = Some(Box::new(Section::new(0, heading)));

                    return;
                }

                panic!("Illegal Level 0 Section");
            }

            if let Some(heading) = line.strip_prefix("== ") {
                self.close();
                self.opened_block = Some(Box::new(Section::new(1, heading)));

                return;
            }
        }
    }
}

impl<'line> Block<'line> for Document<'line> {
    fn children(&self) -> BlockTree<'line> {
        BlockTree::Compound(self.blocks.clone())
    }

    fn push(&mut self, line: &'line str) {
        if !self.body_started {
            self.parse_header(line);

            return;
        }

        self.parse_body(line)
    }

    fn close(&mut self) {
        match self.opened_block.clone() {
            Some(block) => {
                self.blocks.push(block);
                self.opened_block = None
            }
            None => {}
        }
    }
}

#[derive(Clone)]
struct DocumentMetadata<'line> {
    title: Option<&'line str>,
    implicit_line: bool,
    current_attr: Option<&'line str>,
    current_value: Option<&'line str>,
    authors: Option<Vec<Author<'line>>>,
    revision: Option<Revision>,
    custom_attributes: HashMap<&'line str, &'line str>,
}

impl<'line> DocumentMetadata<'line> {
    fn set_title(&mut self, title: &'line str) {
        self.title = Some(title);
        self.implicit_line = true;
    }

    fn has_title(&self) -> bool {
        self.title.is_some()
    }

    fn set_value(&mut self, name: &'line str, value: &'line str) -> Option<&'line str> {
        if name == "author" {
            let mut authors = self.authors.clone().unwrap_or(Vec::with_capacity(1));
            if let Some(author) = authors.get_mut(0) {
                author.name = value;
                authors[0] = author.clone();
            } else {
                authors.push(Author::new_name(value));
            }

            self.authors = Some(authors);

            return Some(value);
        }

        if name == "email" {
            let mut authors = self.authors.clone().unwrap_or(Vec::with_capacity(1));
            if let Some(author) = authors.get_mut(0) {
                author.email = Some(value);
                authors[0] = author.clone();
            } else {
                authors.push(Author::new_with_email("", value));
            }

            self.authors = Some(authors);

            return Some(value);
        }

        self.custom_attributes.insert(name, value)
    }

    fn get_value(&self, name: &'line str) -> Option<&'line str> {
        if name == "author" {
            return self.authors.clone().and_then(|a| a.get(0).map(|a| a.name));
        }

        if name == "email" {
            return self
                .authors
                .clone()
                .and_then(|a| a.get(0).and_then(|a| a.email));
        }

        self.custom_attributes.get(name).map(|s| s.trim())
    }

    fn parse_implicit_line(&mut self, line: &'line str) -> bool {
        if self.implicit_line {
            self.implicit_line = if self.authors.is_none() {
                self.parse_authors_line(line)
            } else if self.revision.is_none() {
                self.parse_revision_line(line)
            } else {
                false
            }
        }

        self.implicit_line
    }

    fn parse_authors_line(&mut self, line: &'line str) -> bool {
        let split_authors: Vec<&str> = line.split_terminator(';').collect();
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

    fn parse_revision_line(&self, line: &'line str) -> bool {
        false
    }

    fn parse_attribute_line(&mut self, line: &'line str) -> bool {
        let attribute_line = if self.current_attr.is_some() {
            self.parse_wrapped_attr(line);
            true
        } else if line.starts_with(':') {
            self.parse_attr(line)
        } else {
            false
        };

        if self.implicit_line && attribute_line {
            self.implicit_line = false
        }

        attribute_line
    }

    fn parse_attr(&mut self, line: &'line str) -> bool {
        if let Some(unset_attr) = line.strip_prefix(":!").and_then(|a| a.strip_suffix(":")) {
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
            line.strip_prefix(":").and_then(|a| a.split_once(": "))
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
                self.set_value(attr_name, attr_value);

                return true;
            }

            self.current_attr = Some(attr_name);

            true
        } else if let Some(unset_attr) = line.strip_prefix(":").and_then(|a| a.strip_suffix("!:")) {
            if !unset_attr.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || unset_attr
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :{}!:", unset_attr);
            }

            self.custom_attributes.remove(unset_attr);

            true
        } else if let Some(set_bool_attr) = line.strip_prefix(":").and_then(|a| a.strip_suffix(":"))
        {
            if !set_bool_attr.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                || set_bool_attr
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .is_some()
            {
                panic!("invalid document attribute: :{}:", set_bool_attr);
            }

            self.set_value(set_bool_attr, "");

            true
        } else {
            false
        }
    }

    fn parse_wrapped_attr(&mut self, line: &'line str) {
        if let Some(wrap_value) = line.strip_suffix(" + \\") {
            wrap_value.to_owned().push_str("\n");
            self.current_value = self.current_value.map(|s| {
                s.to_owned().push_str(wrap_value);

                s
            });

            return;
        }

        if let Some(wrap_value) = line.strip_suffix(" \\") {
            wrap_value.to_owned().push_str(" ");
            self.current_value = self.current_value.map(|s| {
                s.to_owned().push_str(wrap_value);

                s
            });

            return;
        }

        let attr_name = self.current_attr.unwrap();
        let attr_value = self.current_value.unwrap();
        attr_value.to_owned().push_str(line);

        self.current_attr = None;
        self.current_value = None;

        self.custom_attributes.insert(attr_name, attr_value);
    }
}

impl<'line> Default for DocumentMetadata<'line> {
    fn default() -> DocumentMetadata<'line> {
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

#[derive(Clone)]
struct Author<'line> {
    name: &'line str,
    email: Option<&'line str>,
}

impl<'line> Author<'line> {
    fn new_name(name: &'line str) -> Self {
        Self::new(name, None)
    }

    fn new_with_email(name: &'line str, email: &'line str) -> Self {
        Self::new(name, Some(email))
    }

    fn new(name: &'line str, email: Option<&'line str>) -> Self {
        Author {
            name: name.trim(),
            email,
        }
    }
}

#[derive(Clone)]
struct Revision {
    number: String,
    date: String,
    remark: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_header() {
        let mut document = Document::new(Doctype::Article);

        for line in "// this comment line is ignored\n= Document Title\nKismet R. Lee <kismet@asciidoctor.org>\n:description: The document's description.\n:sectanchors:\n:url-repo: https://my-git-repo.com\n\nThe document body starts here.".lines() {
            document.push(line);
        }
        document.close();

        assert_eq!(Some("Document Title"), document.clone().title());
        let authors = document.clone().authors();
        let metadata = document.clone().metadata;
        assert_eq!(1, authors.len());
        let author = &authors[0];
        assert_eq!("Kismet R. Lee", author.name);
        assert_eq!(Some("kismet@asciidoctor.org"), author.email);
        assert_eq!(Some("Kismet R. Lee"), metadata.get_value("author"));
        assert_eq!(Some("kismet@asciidoctor.org"), metadata.get_value("email"));
        assert_eq!(
            Some("The document's description."),
            metadata.get_value("description")
        );
        assert_eq!(Some(""), metadata.get_value("sectanchors"));
        assert_eq!(
            Some("https://my-git-repo.com"),
            metadata.get_value("url-repo")
        );
    }

    #[test]
    fn document_header_author_entry() {
        let mut document = Document::new(Doctype::Article);

        for line in "= Document Title\n:email: kismet@asciidoctor.org\n:author: Kismet R. Lee\n\nThe document body starts here.".lines() {
            document.push(line);
        }
        document.close();

        assert_eq!(Some("Document Title"), document.clone().title());
        let authors = document.clone().authors();
        let metadata = document.clone().metadata;
        assert_eq!(1, authors.len());
        let author = &authors[0];
        assert_eq!("Kismet R. Lee", author.name);
        assert_eq!(Some("kismet@asciidoctor.org"), author.email);
        assert_eq!(Some("Kismet R. Lee"), metadata.get_value("author"));
        assert_eq!(Some("kismet@asciidoctor.org"), metadata.get_value("email"));
    }

    #[test]
    #[should_panic]
    fn illegal_level0_section_block() {
        let mut document = Document::new(Doctype::Article);

        for line in
            "= Document Title\n\n= Illegal Level 0 Section (violates rule #1)\n\n== First Section"
                .lines()
        {
            document.push(line);
        }
        document.close();
    }
}
