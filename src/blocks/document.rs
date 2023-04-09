use std::collections::HashMap;

use super::{Block, BlockTree, Doctype};

#[derive(Clone)]
pub(crate) struct Document {
    blocks: Vec<Box<dyn Block>>,
    doctype: Doctype,
    body_started: bool,
    pub(crate) metadata: DocumentMetadata,
}

impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Self {
            blocks: Vec::with_capacity(1),
            doctype,
            body_started: false,
            metadata: Default::default(),
        }
    }

    pub(crate) fn title(self) -> Option<&'static str> {
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

    pub(crate) fn authors(self) -> Vec<Author> {
        self.metadata.authors.unwrap_or(Vec::with_capacity(0))
    }

    pub(crate) fn revision(self) -> Option<Revision> {
        self.metadata.revision
    }

    pub(crate) fn description(self) -> Option<&'static str> {
        self.metadata.get_value("description").map(|s| s.trim())
    }

    fn parse_header(&mut self, text: &'static str) {
        if text == "" {
            if self.metadata.has_title() {
                self.body_started = true;
            }

            return;
        }

        if self.metadata.parse_attribute_line(text) {
            return;
        }

        if self.metadata.parse_implicit_line(text) {
            return;
        }

        if let Some(document_title) = text.strip_prefix("= ") {
            self.metadata.set_title(document_title);

            return;
        }

        if self.metadata.has_title() {
            panic!("invalid document header: {}", text);
        }

        self.body_started = true;

        self.parse_body(text)
    }

    fn parse_body(&mut self, text: &'static str) {
        if !self.metadata.has_title() && matches!(self.doctype, Doctype::Manpage) {
            panic!("require document title for doctype-manpage");
        }

        if let Some(level0_heading) = text.strip_prefix("= ") {
            panic!("Illegal Level 0 Section");
        }
    }
}

impl Block for Document {
    fn children(&self) -> BlockTree {
        BlockTree::Compound(self.blocks.clone())
    }

    fn push(&mut self, text: &'static str) {
        if !self.body_started {
            self.parse_header(text);

            return;
        }

        self.parse_body(text)
    }
}

#[derive(Clone)]
pub(crate) struct DocumentMetadata {
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

    pub(crate) fn has_title(&self) -> bool {
        self.title.is_some()
    }

    fn set_value(&mut self, name: &'static str, value: &'static str) -> Option<&'static str> {
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

    pub(crate) fn get_value(&self, name: &'static str) -> Option<&'static str> {
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

    fn parse_implicit_line(&mut self, text: &'static str) -> bool {
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

    fn parse_attribute_line(&mut self, text: &'static str) -> bool {
        let attribute_line = if self.current_attr.is_some() {
            self.parse_wrapped_attr(text);
            true
        } else if text.starts_with(':') {
            self.parse_attr(text)
        } else {
            false
        };

        if self.implicit_line && attribute_line {
            self.implicit_line = false
        }

        attribute_line
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
                self.set_value(attr_name, attr_value);

                return true;
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

            self.set_value(set_bool_attr, "");

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

#[derive(Clone)]
pub(crate) struct Author {
    pub(crate) name: &'static str,
    pub(crate) email: Option<&'static str>,
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
pub(crate) struct Revision {
    number: String,
    date: String,
    remark: String,
}
