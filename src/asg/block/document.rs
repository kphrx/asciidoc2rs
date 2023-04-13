use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, Section, SectionBody};
use crate::asg::{Headline, Location, NodeType};
use crate::Doctype;

use std::collections::HashMap;
use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    attributes: Option<HashMap<String, String>>,
    header: Option<DocumentHeader>,
    blocks: Vec<SectionBody>,
    location: Option<Location>,

    #[serde(skip)]
    doctype: Doctype,
    #[serde(skip)]
    parser: HeaderParser,
    #[serde(skip)]
    is_started_body: bool,
    #[serde(skip)]
    is_preamble: bool,
    #[serde(skip)]
    is_comment: bool,
    #[serde(skip)]
    previous_line: String,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    title: Headline,
    location: Option<Location>,
}
impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Self {
            name: "document".to_owned(),
            node_type: NodeType::Block,
            attributes: None,
            header: None,
            blocks: Vec::with_capacity(0),
            location: None,
            doctype,
            parser: Default::default(),
            is_started_body: false,
            is_preamble: true,
            is_comment: false,
            previous_line: "".to_owned(),
        }
    }

    pub(crate) fn push(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if !self.is_started_body {
            match self.parser.parse_line(line)? {
                LineKind::NotHeader => {
                    self.is_started_body = true;

                    if matches!(self.doctype, Doctype::Manpage) {
                        panic!("require document title for doctype-manpage");
                    }
                }
                LineKind::End => {
                    self.is_started_body = true;

                    if matches!(self.doctype, Doctype::Manpage) && !self.parser.has_title {
                        panic!("require document title for doctype-manpage");
                    }

                    return Ok(());
                }
                LineKind::Comment | LineKind::Skip | LineKind::Wrap => {
                    return Ok(());
                }
                LineKind::Title(document_title) => {
                    let title = Headline::new(&document_title);
                    self.header = Some(DocumentHeader {
                        title,
                        location: None,
                    });
                    self.parser.is_authors_line = true;
                    return Ok(());
                }
                LineKind::Authors(authors) => {
                    self.set_authors(authors);
                    return Ok(());
                }
                LineKind::Revision => {
                    return Ok(());
                }
                LineKind::UnsetAttribute(key) => {
                    self.unset_value(key);
                    return Ok(());
                }
                LineKind::Attribute(key, value) => {
                    self.set_value(key, &value);
                    return Ok(());
                }
            }
        }

        if self.is_preamble {
            if matches!(self.doctype, Doctype::Manpage) && !self.parser.has_title {
                panic!("require document title for doctype-manpage");
            }

            if self.is_comment {
                if line == "////" {
                    self.previous_line = "".to_owned();
                    self.is_comment = false
                }

                return Ok(());
            }

            if let Some(SectionBody::Block(last)) = self.blocks.last_mut() {
                if self.previous_line != "" {
                    self.previous_line = line.to_owned();

                    return Ok(());
                }
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

            if self.previous_line == "" {
                if let Some(heading) = line.strip_prefix("= ") {
                    if !matches!(self.doctype, Doctype::Book) {
                        self.previous_line = line.to_owned();
                        let paragraph = Block::new_paragraph(line);
                        self.blocks.push(SectionBody::Block(paragraph));
                        return Err("level 0 sections can only be used when doctype is book".into());
                    }

                    self.previous_line = line.to_owned();
                    self.is_preamble = false;
                    let section = Section::new(0, heading);
                    self.blocks.push(SectionBody::Section(section));

                    return Ok(());
                }

                if let Some(heading) = line.strip_prefix("== ") {
                    self.previous_line = line.to_owned();
                    self.is_preamble = false;
                    let section = Section::new(1, heading);
                    self.blocks.push(SectionBody::Section(section));

                    return Ok(());
                }
            }

            self.previous_line = line.to_owned();
            let paragraph = Block::new_paragraph(line);
            self.blocks.push(SectionBody::Block(paragraph));

            return Ok(());
        }

        if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
            return last.push(line);
        }

        panic!("not expected non preamble top level block")
    }

    fn parse_preamble(&mut self, line: String) -> Result<(), Box<dyn Error>> {
        if line == "////" {
            self.previous_line = "".to_owned();
            self.is_comment = true;

            return Ok(());
        }

        if line == "" {
            return Ok(());
        }

        return Ok(());
    }

    fn set_authors(&mut self, authors: Vec<Author>) {
        for (i, author) in authors.iter().enumerate() {
            let (author_key, email_key) = if i > 0 {
                let index = i + 1;
                (format!("author_{}", index), format!("email_{}", index))
            } else {
                ("author".to_owned(), "email".to_owned())
            };

            self.set_value(author_key, &author.author);
            if let Some(email) = &author.email {
                self.set_value(email_key, email);
            }
        }
    }

    fn unset_value(&mut self, name: String) {
        if let Some(attrs) = self.attributes.as_mut() {
            attrs.remove(&name.to_lowercase());
        }
    }

    fn set_value(&mut self, name: String, value: &str) {
        if let Some(attrs) = self.attributes.as_mut() {
            attrs.insert(name.to_lowercase(), value.to_owned());
        } else {
            let mut attrs = HashMap::new();
            attrs.insert(name.to_lowercase(), value.to_owned());
            self.attributes = Some(attrs);
        }
    }
}

enum LineKind {
    NotHeader,
    Comment,
    Skip,
    End,
    Title(String),
    Authors(Vec<Author>),
    Revision,
    UnsetAttribute(String),
    Attribute(String, String),
    Wrap,
}

#[derive(Debug)]
struct HeaderParser {
    doctype: Doctype,
    has_title: bool,
    has_attr: bool,
    is_authors_line: bool,
    is_revision_line: bool,
    wrapped_attr: Option<(String, String)>,
}
impl Default for HeaderParser {
    fn default() -> Self {
        Self::new(Doctype::Article)
    }
}
impl HeaderParser {
    fn new(doctype: Doctype) -> Self {
        Self {
            doctype,
            has_title: false,
            has_attr: false,
            is_authors_line: false,
            is_revision_line: false,
            wrapped_attr: None,
        }
    }

    fn parse_line(&mut self, line: &str) -> Result<LineKind, Box<dyn Error>> {
        if line == "" {
            if self.has_title || self.has_attr {
                return Ok(LineKind::End);
            }

            return Ok(LineKind::Skip);
        }

        match self.parse_attribute_line(line)? {
            LineKind::Attribute(key, value) => {
                self.has_attr = true;
                return Ok(LineKind::Attribute(key, value));
            }
            LineKind::Wrap => {
                return Ok(LineKind::Wrap);
            }
            LineKind::Skip => {}
            _ => panic!("not expected value"),
        }

        if line.starts_with("//") && !line.starts_with("///") {
            return Ok(LineKind::Comment);
        }

        match self.parse_implicit_line(line)? {
            LineKind::Authors(a) => {
                return Ok(LineKind::Authors(a));
            }
            LineKind::Revision => {
                return Ok(LineKind::Revision);
            }
            LineKind::End => {}
            _ => panic!("not expected value"),
        }

        if self.has_title {
            return Err("invalid document header".into());
        }

        if let Some(document_title) = line.strip_prefix("= ") {
            self.has_title = true;
            return Ok(LineKind::Title(document_title.to_owned()));
        }

        Ok(LineKind::NotHeader)
    }

    fn parse_attribute_line(&mut self, line: &str) -> Result<LineKind, Box<dyn Error>> {
        let result = if self.wrapped_attr.is_some() {
            self.parse_wrapped_attr(line)?
        } else if let Some((attr_name, attr_value)) =
            line.strip_prefix(":").and_then(|a| a.split_once(": "))
        {
            if Self::is_valid_attribute_name(attr_name.to_owned()) {
                self.wrapped_attr = Some((attr_name.to_owned(), "".to_owned()));
                self.parse_wrapped_attr(&attr_value.to_owned())?
            } else {
                return Err(format!("invalid document attribute: {}", attr_name).into());
            }
        } else if let Some(attr_name) = line.strip_prefix(":").and_then(|a| a.strip_suffix(":")) {
            if let Some(unset_attr) = attr_name.strip_prefix("!").or(attr_name.strip_suffix("!")) {
                if !Self::is_valid_attribute_name(unset_attr.to_owned()) {
                    return Err(format!("invalid document attribute: {}", attr_name).into());
                }
                LineKind::UnsetAttribute(unset_attr.to_owned())
            } else {
                if !Self::is_valid_attribute_name(attr_name.to_owned()) {
                    return Err(format!("invalid document attribute: {}", attr_name).into());
                }
                LineKind::Attribute(attr_name.to_owned(), "".to_owned())
            }
        } else {
            LineKind::Skip
        };

        Ok(result)
    }

    fn is_valid_attribute_name(attr_name: String) -> bool {
        attr_name.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            && !attr_name.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
    }

    fn parse_wrapped_attr(&mut self, line: &str) -> Result<LineKind, Box<dyn Error>> {
        let result = if let Some(wrap_value) = line.strip_suffix(" + \\") {
            let mut value = wrap_value.to_owned();
            value.push_str("\n");

            let Some((_, current_value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            current_value.push_str(&value);

            LineKind::Wrap
        } else if let Some(wrap_value) = line.strip_suffix(" \\") {
            let mut value = wrap_value.to_owned();
            value.push_str(" ");

            let Some((_, current_value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            current_value.push_str(&value);

            LineKind::Wrap
        } else {
            let Some((key, value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            value.push_str(&line);
            let attr = LineKind::Attribute(key.to_owned(), value.to_owned());
            self.wrapped_attr = None;

            attr
        };

        Ok(result)
    }

    fn parse_implicit_line(&mut self, line: &str) -> Result<LineKind, Box<dyn Error>> {
        let result = if self.is_authors_line {
            self.is_authors_line = false;
            self.is_revision_line = true;

            let authors = self.parse_authors_line(line)?;
            LineKind::Authors(authors)
        } else if self.is_revision_line {
            self.is_revision_line = false;

            self.parse_revision_line(line)
        } else {
            LineKind::End
        };

        Ok(result)
    }

    fn parse_authors_line(&mut self, line: &str) -> Result<Vec<Author>, Box<dyn Error>> {
        let split_authors: Vec<&str> = line.split_terminator(';').collect();
        let mut authors: Vec<Author> = Vec::with_capacity(split_authors.len());
        for author in split_authors {
            let author = match author.split_once(" <") {
                None => Author::new(author.to_owned(), None),
                Some((a, e)) => {
                    if let Some(email) = e.strip_suffix('>') {
                        Author::new(a.to_owned(), Some(email.to_owned()))
                    } else {
                        return Err("Invalid author format".into());
                    }
                }
            };

            authors.push(author)
        }

        Ok(authors)
    }

    fn parse_revision_line(&mut self, line: &str) -> LineKind {
        LineKind::End
    }
}

#[derive(Debug)]
struct Author {
    author: String,
    email: Option<String>,
}
impl Author {
    fn new(author: String, email: Option<String>) -> Self {
        Self { author, email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &'static str) -> Result<Document, Box<dyn Error>> {
        let mut document = Document::new(Doctype::Article);

        for line in text.lines() {
            document.push(line)?;
        }

        Ok(document)
    }

    #[test]
    fn document_header() {
        let document = parse("// this comment line is ignored\n= Document Title\nKismet R. Lee <kismet@asciidoctor.org>\n:description: The document's description.\n:sectanchors:\n:url-repo: https://my-git-repo.com\n\nThe document body starts here.").unwrap();

        assert_eq!(
            Some(Headline::new("Document Title").heading()),
            document.header.map(|h| h.title.heading())
        );
        let mut attrs = HashMap::new();
        attrs.insert("author".to_owned(), "Kismet R. Lee".to_owned());
        attrs.insert("email".to_owned(), "kismet@asciidoctor.org".to_owned());
        attrs.insert(
            "description".to_owned(),
            "The document's description.".to_owned(),
        );
        attrs.insert("sectanchors".to_owned(), "".to_owned());
        attrs.insert("url-repo".to_owned(), "https://my-git-repo.com".to_owned());
        assert_eq!(Some(attrs), document.attributes);
        assert_eq!(1, document.blocks.len());
    }

    #[test]
    #[should_panic]
    fn illegal_level0_section_block() {
        parse(
            "= Document Title\n\n= Illegal Level 0 Section (violates rule #1)\n\n== First Section",
        )
        .unwrap();
    }

    #[test]
    fn paragraph_blocks() {
        let document = parse("Paragraphs don't require any special markup in AsciiDoc.\nA paragraph is just one or more lines of consecutive text.\n\nTo begin a new paragraph, separate it by at least one empty line from the previous paragraph or block.").unwrap();

        assert_eq!(2, document.blocks.len());
    }
}
