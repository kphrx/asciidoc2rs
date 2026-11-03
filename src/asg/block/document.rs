use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::{Block, LineKind, Section, SectionBody};
use crate::asg::{Inlines, Location, NodeType};
use crate::Doctype;

use std::collections::HashMap;
use std::error::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", rename = "document")]
pub struct Document {
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(flatten)]
    header: Option<DocumentHeader>,
    #[serde(default)]
    blocks: Vec<SectionBody>,
    location: Option<Location>,

    #[serde(skip)]
    doctype: Doctype,
    #[serde(skip)]
    parser: HeaderParser,
    #[serde(skip)]
    is_started_body: bool,
    #[serde(skip)]
    current_block: Option<Block>,
    #[serde(skip)]
    comment_delimiter: Option<String>,
    #[serde(skip)]
    previous_line: String,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    header: Option<TitleAndAuthors>,
    attributes: HashMap<String, Option<String>>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct TitleAndAuthors {
    title: Option<Inlines>,
    authors: Option<Vec<Author>>,
    location: Option<Location>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Author {
    fullname: Option<String>,
    initials: Option<String>,
    firstname: Option<String>,
    middlename: Option<String>,
    lastname: Option<String>,
    address: Option<String>,
}
impl Document {
    pub(crate) fn new(doctype: Doctype) -> Self {
        Self {
            node_type: NodeType::Block,
            header: None,
            blocks: Vec::with_capacity(0),
            location: None,
            doctype,
            parser: Default::default(),
            is_started_body: false,
            current_block: None,
            comment_delimiter: None,
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
        if !self.is_started_body {
            match self.parser.parse_line(line)? {
                HeaderLineKind::NotHeader => {
                    self.is_started_body = true;

                    if matches!(self.doctype, Doctype::Manpage) {
                        panic!("require document title for doctype-manpage");
                    }
                }
                HeaderLineKind::End => {
                    self.is_started_body = true;

                    if matches!(self.doctype, Doctype::Manpage) && !self.parser.has_title {
                        panic!("require document title for doctype-manpage");
                    }

                    return Ok(());
                }
                HeaderLineKind::Comment | HeaderLineKind::Skip | HeaderLineKind::Wrap => {
                    return Ok(());
                }
                HeaderLineKind::Title(document_title) => {
                    let title = Inlines::new_with_line(&document_title);
                    match self.header {
                        Some(DocumentHeader {
                            header: Some(ref mut h),
                            ..
                        }) => h.title = Some(title),
                        Some(ref mut h) => {
                            h.header = Some(TitleAndAuthors {
                                title: Some(title),
                                ..Default::default()
                            })
                        }
                        None => {
                            self.header = Some(DocumentHeader {
                                header: Some(TitleAndAuthors {
                                    title: Some(title),
                                    ..Default::default()
                                }),
                                attributes: HashMap::new(),
                            })
                        }
                    }
                    self.parser.is_authors_line = true;
                    return Ok(());
                }
                HeaderLineKind::Authors(authors) => {
                    self.set_authors(authors);
                    return Ok(());
                }
                HeaderLineKind::Revision(revnumber, revdate, revremark) => {
                    self.set_value("revnumber".to_owned(), &revnumber);
                    if let Some(date) = revdate {
                        self.set_value("revdate".to_owned(), &date);

                        if let Some(remark) = revremark {
                            self.set_value("revremark".to_owned(), &remark);
                        }
                    }

                    return Ok(());
                }
                HeaderLineKind::UnsetAttribute(key) => {
                    self.unset_value(key);
                    return Ok(());
                }
                HeaderLineKind::Attribute(key, value) => {
                    self.set_value(key, &value);
                    return Ok(());
                }
            }
        }

        if self.comment_delimiter.is_some() {
            if matches!(LineKind::parse(line.to_owned()), LineKind::CommentDelimiter(x) if matches!(&self.comment_delimiter, Some(y) if x == *y))
            {
                self.previous_line = "".to_owned();
                self.comment_delimiter = None;
            }

            return Ok(());
        }

        if let Some(current) = self.current_block.as_mut() {
            if current.is_delimited_block() {
                self.previous_line = line.to_owned();

                if LineKind::parse(line.to_owned()).block_delimiter() != current.delimiter() {
                    current.push(line)?;

                    return Ok(());
                }

                current.end();
                self.blocks.push(SectionBody::Block(current.clone()));
                self.current_block = None;

                return Ok(());
            }

            if let Block::AnyList(_list) = current {
                match LineKind::parse(line.to_owned()) {
                    LineKind::Empty => {
                        if self.previous_line == "//" {
                            self.previous_line = "".to_owned();
                            current.end();
                            self.blocks.push(SectionBody::Block(current.clone()));
                            self.current_block = None;

                            return Ok(());
                        }

                        self.previous_line = "".to_owned();

                        return Ok(());
                    }
                    LineKind::CommentMarker => {
                        if self.previous_line.is_empty() {
                            self.previous_line = "//".to_owned();
                        }

                        return Ok(());
                    }
                    _ => {
                        if !self.previous_line.is_empty() {
                            self.previous_line = line.to_owned();
                            current.push(line)?;

                            return Ok(());
                        }

                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;
                    }
                }
            } else {
                match LineKind::parse(line.to_owned()) {
                    LineKind::Empty => {
                        self.previous_line = "".to_owned();
                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;

                        return Ok(());
                    }
                    LineKind::CommentMarker => {
                        self.previous_line = "".to_owned();

                        return Ok(());
                    }
                    LineKind::CommentDelimiter(delimiter) => {
                        self.comment_delimiter = Some(delimiter);

                        current.end();
                        self.blocks.push(SectionBody::Block(current.clone()));
                        self.current_block = None;

                        return Ok(());
                    }
                    _ => {
                        self.previous_line = line.to_owned();

                        return current.push(line);
                    }
                }
            }
        }

        if let Some(SectionBody::Section(last)) = self.blocks.last_mut() {
            if let LineKind::HeadingMarker { level, title } = LineKind::parse(line.to_owned()) {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();

                    return last.push(line);
                }

                if level == 0 && !matches!(self.doctype, Doctype::Book) {
                    self.previous_line = line.to_owned();
                    let section = Section::new(0, &title);
                    self.blocks.push(SectionBody::Section(Box::new(section)));

                    return Err("level 0 sections can only be used when doctype is book".into());
                }

                if level <= last.level {
                    self.previous_line = line.to_owned();
                    let section = Section::new(level, &title);
                    self.blocks.push(SectionBody::Section(Box::new(section)));

                    return Ok(());
                }
            }

            self.previous_line = line.to_owned();

            return last.push(line);
        }

        self.parse_preamble(line)
    }

    fn parse_preamble(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        match LineKind::parse(line.to_owned()) {
            LineKind::Empty => {
                self.previous_line = "".to_owned();

                Ok(())
            }
            LineKind::CommentMarker => {
                self.previous_line = "".to_owned();

                Ok(())
            }
            LineKind::CommentDelimiter(delimiter) => {
                self.comment_delimiter = Some(delimiter);

                Ok(())
            }
            LineKind::HeadingMarker { level, title } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                if level > 1 {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Err("cannot skip section level".into());
                }

                if level == 0 && !matches!(self.doctype, Doctype::Book) {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Err("level 0 sections can only be used when doctype is book".into());
                }

                self.previous_line = line.to_owned();
                let section = Section::new(level, &title);
                self.blocks.push(SectionBody::Section(Box::new(section)));

                Ok(())
            }
            LineKind::UnorderedListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let unordered_list = Block::new_unordered_list(marker, principal);
                self.current_block = Some(unordered_list);

                Ok(())
            }
            LineKind::OrderedListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(marker, principal);
                self.current_block = Some(ordered_list);

                Ok(())
            }
            LineKind::OffsetOrderedListMarker { offset, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let ordered_list = Block::new_ordered_list(format!("{}.", offset), principal);
                self.current_block = Some(ordered_list);

                Ok(())
            }
            LineKind::CalloutListMarker { marker, principal } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let callout_list = Block::new_callout_list(marker, principal);
                self.current_block = Some(callout_list);

                Ok(())
            }
            LineKind::DescriptionListMarker {
                marker,
                term,
                principal,
            } => {
                if !self.previous_line.is_empty() {
                    self.previous_line = line.to_owned();
                    let paragraph = Block::new_paragraph(line);
                    self.current_block = Some(paragraph);

                    return Ok(());
                }

                self.previous_line = line.to_owned();
                let description_list = Block::new_description_list(marker, term, principal);
                self.current_block = Some(description_list);

                Ok(())
            }
            LineKind::Unknown => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                Ok(())
            }
            _ => {
                self.previous_line = line.to_owned();
                let paragraph = Block::new_paragraph(line);
                self.current_block = Some(paragraph);

                Ok(())
            }
        }
    }

    fn set_authors(&mut self, authors: Vec<Author>) {
        if let Some(ref mut header) = self.header {
            match header.header {
                Some(TitleAndAuthors {
                    authors: Some(ref mut header_authors),
                    ..
                }) => header_authors.extend(authors),
                Some(ref mut h) => h.authors = Some(authors),
                None => {
                    header.header = Some(TitleAndAuthors {
                        authors: Some(authors),
                        ..Default::default()
                    })
                }
            }
        } else {
            self.header = Some(DocumentHeader {
                header: Some(TitleAndAuthors {
                    authors: Some(authors),
                    ..Default::default()
                }),
                attributes: HashMap::new(),
            })
        }
    }

    fn unset_value(&mut self, name: String) {
        if let Some(DocumentHeader {
            attributes: attrs, ..
        }) = self.header.as_mut()
        {
            attrs.insert(name.to_lowercase(), None);
        }
    }

    fn set_value(&mut self, name: String, value: &str) {
        if let Some(DocumentHeader {
            attributes: attrs, ..
        }) = self.header.as_mut()
        {
            attrs.insert(name.to_lowercase(), Some(value.to_owned()));
        } else {
            let mut attrs = HashMap::new();
            attrs.insert(name.to_lowercase(), Some(value.to_owned()));
            self.header = Some(DocumentHeader {
                header: None,
                attributes: attrs,
            });
        }
    }
}

enum HeaderLineKind {
    NotHeader,
    Comment,
    Skip,
    End,
    Title(String),
    Authors(Vec<Author>),
    Revision(String, Option<String>, Option<String>),
    UnsetAttribute(String),
    Attribute(String, String),
    Wrap,
}

#[derive(Debug)]
struct HeaderParser {
    has_title: bool,
    has_attr: bool,
    is_authors_line: bool,
    is_revision_line: bool,
    wrapped_attr: Option<(String, String)>,
}
impl Default for HeaderParser {
    fn default() -> Self {
        Self::new()
    }
}
impl HeaderParser {
    fn new() -> Self {
        Self {
            has_title: false,
            has_attr: false,
            is_authors_line: false,
            is_revision_line: false,
            wrapped_attr: None,
        }
    }

    fn parse_line(&mut self, line: &str) -> Result<HeaderLineKind, Box<dyn Error>> {
        if line.is_empty() {
            if self.has_title || self.has_attr {
                return Ok(HeaderLineKind::End);
            }

            return Ok(HeaderLineKind::Skip);
        }

        match self.parse_attribute_line(line)? {
            HeaderLineKind::Attribute(key, value) => {
                self.has_attr = true;
                return Ok(HeaderLineKind::Attribute(key, value));
            }
            HeaderLineKind::Wrap => {
                return Ok(HeaderLineKind::Wrap);
            }
            HeaderLineKind::Skip => {}
            _ => panic!("not expected value"),
        }

        if line.starts_with("//") && !line.starts_with("///") {
            return Ok(HeaderLineKind::Comment);
        }

        match self.parse_implicit_line(line)? {
            HeaderLineKind::Authors(a) => {
                return Ok(HeaderLineKind::Authors(a));
            }
            HeaderLineKind::Revision(n, d, r) => {
                return Ok(HeaderLineKind::Revision(n, d, r));
            }
            HeaderLineKind::End => {}
            _ => panic!("not expected value"),
        }

        if self.has_title {
            return Err("invalid document header".into());
        }

        if let Some(document_title) = line.strip_prefix("= ") {
            self.has_title = true;
            return Ok(HeaderLineKind::Title(document_title.to_owned()));
        }

        Ok(HeaderLineKind::NotHeader)
    }

    fn parse_attribute_line(&mut self, line: &str) -> Result<HeaderLineKind, Box<dyn Error>> {
        let result = if self.wrapped_attr.is_some() {
            self.parse_wrapped_attr(line)?
        } else if let Some((attr_name, attr_value)) =
            line.strip_prefix(':').and_then(|a| a.split_once(": "))
        {
            if Self::is_valid_attribute_name(attr_name.to_owned()) {
                self.wrapped_attr = Some((attr_name.to_owned(), "".to_owned()));
                self.parse_wrapped_attr(attr_value)?
            } else {
                return Err(format!("invalid document attribute: {}", attr_name).into());
            }
        } else if let Some(attr_name) = line.strip_prefix(':').and_then(|a| a.strip_suffix(':')) {
            if let Some(unset_attr) = attr_name.strip_prefix('!').or(attr_name.strip_suffix('!')) {
                if !Self::is_valid_attribute_name(unset_attr.to_owned()) {
                    return Err(format!("invalid document attribute: {}", attr_name).into());
                }
                HeaderLineKind::UnsetAttribute(unset_attr.to_owned())
            } else {
                if !Self::is_valid_attribute_name(attr_name.to_owned()) {
                    return Err(format!("invalid document attribute: {}", attr_name).into());
                }
                HeaderLineKind::Attribute(attr_name.to_owned(), "".to_owned())
            }
        } else {
            HeaderLineKind::Skip
        };

        Ok(result)
    }

    fn is_valid_attribute_name(attr_name: String) -> bool {
        attr_name.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            && !attr_name.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
    }

    fn parse_wrapped_attr(&mut self, line: &str) -> Result<HeaderLineKind, Box<dyn Error>> {
        let result = if let Some(wrap_value) = line.strip_suffix(" + \\") {
            let mut value = wrap_value.to_owned();
            value.push('\n');

            let Some((_, current_value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            current_value.push_str(&value);

            HeaderLineKind::Wrap
        } else if let Some(wrap_value) = line.strip_suffix(" \\") {
            let mut value = wrap_value.to_owned();
            value.push(' ');

            let Some((_, current_value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            current_value.push_str(&value);

            HeaderLineKind::Wrap
        } else {
            let Some((key, value)) = self.wrapped_attr.as_mut() else {
                panic!("cannot call");
            };
            value.push_str(line);
            let attr = HeaderLineKind::Attribute(key.to_owned(), value.to_owned());
            self.wrapped_attr = None;

            attr
        };

        Ok(result)
    }

    fn parse_implicit_line(&mut self, line: &str) -> Result<HeaderLineKind, Box<dyn Error>> {
        let result = if self.is_authors_line {
            self.is_authors_line = false;
            self.is_revision_line = true;

            let authors = self.parse_authors_line(line)?;
            HeaderLineKind::Authors(authors)
        } else if self.is_revision_line {
            self.is_revision_line = false;

            self.parse_revision_line(line)?
        } else {
            HeaderLineKind::End
        };

        Ok(result)
    }

    fn parse_authors_line(&mut self, line: &str) -> Result<Vec<Author>, Box<dyn Error>> {
        let split_authors: Vec<&str> = line.split_terminator(';').collect();
        let mut authors: Vec<Author> = Vec::with_capacity(split_authors.len());
        for author in split_authors {
            let author = match author.split_once(" <") {
                None => Author {
                    fullname: Some(author.to_owned()),
                    ..Default::default()
                },
                Some((a, e)) => {
                    if let Some(email) = e.strip_suffix('>') {
                        Author {
                            fullname: Some(a.to_owned()),
                            address: Some(email.to_owned()),
                            ..Default::default()
                        }
                    } else {
                        return Err("Invalid author format".into());
                    }
                }
            };

            authors.push(author)
        }

        Ok(authors)
    }

    fn parse_revision_line(&mut self, line: &str) -> Result<HeaderLineKind, Box<dyn Error>> {
        let (revnumber, revdate, revremark) = match line.split_once(", ") {
            None => {
                if let Some(revnumber) = line.strip_prefix('v') {
                    if !revnumber.contains(|c: char| c.is_ascii_digit()) {
                        return Err("Invalid revision format".into());
                    }

                    let (number, remark) = match revnumber.split_once(": ") {
                        None => (revnumber.to_owned(), None),
                        Some((number, remark)) => (number.to_owned(), Some(remark.to_owned())),
                    };

                    (number, None, remark)
                } else {
                    return Err("Invalid revision format".into());
                }
            }
            Some((revnumber, revdate)) => {
                if !revnumber.contains(|c: char| c.is_ascii_digit()) {
                    return Err("Invalid revision format".into());
                }

                let (date, remark) = match revdate.split_once(": ") {
                    None => (Some(revdate.to_owned()), None),
                    Some((date, remark)) => (Some(date.to_owned()), Some(remark.to_owned())),
                };

                (
                    revnumber
                        .trim_start_matches(|c: char| !c.is_ascii_digit())
                        .to_owned(),
                    date,
                    remark,
                )
            }
        };

        Ok(HeaderLineKind::Revision(revnumber, revdate, revremark))
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
        document.end();

        Ok(document)
    }

    #[test]
    fn document_header() {
        let document = parse("// this comment line is ignored\n= Document Title\nKismet R. Lee <kismet@asciidoctor.org>\n:description: The document's description.\n:sectanchors:\n:url-repo: https://my-git-repo.com\n\nThe document body starts here.").unwrap();

        assert_eq!(
            Some(Some(TitleAndAuthors {
                title: Some(Inlines::new_with_line("Document Title")),
                authors: Some(vec![Author {
                    fullname: Some("Kismet R. Lee".to_string()),
                    address: Some("kismet@asciidoctor.org".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            })),
            document.header.as_ref().map(|h| h.header.clone())
        );
        let mut attrs = HashMap::new();
        attrs.insert(
            "description".to_owned(),
            Some("The document's description.".to_owned()),
        );
        attrs.insert("sectanchors".to_owned(), Some("".to_owned()));
        attrs.insert(
            "url-repo".to_owned(),
            Some("https://my-git-repo.com".to_owned()),
        );
        assert_eq!(Some(attrs), document.header.map(|h| h.attributes));
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
    fn revision_line() {
        let document =
            parse("= The Intrepid Chronicles\nKismet Lee\n2.9, October 31, 2021: Fall incarnation")
                .unwrap();

        assert_eq!(
            Some(Some(TitleAndAuthors {
                title: Some(Inlines::new_with_line("The Intrepid Chronicles")),
                authors: Some(vec![Author {
                    fullname: Some("Kismet Lee".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            })),
            document.header.as_ref().map(|h| h.header.clone())
        );
        let mut attrs = HashMap::new();
        attrs.insert("revnumber".to_owned(), Some("2.9".to_owned()));
        attrs.insert("revdate".to_owned(), Some("October 31, 2021".to_owned()));
        attrs.insert("revremark".to_owned(), Some("Fall incarnation".to_owned()));
        assert_eq!(Some(attrs), document.header.map(|h| h.attributes));
    }

    #[test]
    fn number_only_revision_line() {
        let document = parse("= The Intrepid Chronicles\nKismet Lee\nv7.5").unwrap();

        assert_eq!(
            Some(Some(TitleAndAuthors {
                title: Some(Inlines::new_with_line("The Intrepid Chronicles")),
                authors: Some(vec![Author {
                    fullname: Some("Kismet Lee".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            })),
            document.header.as_ref().map(|h| h.header.clone())
        );
        let mut attrs = HashMap::new();
        attrs.insert("revnumber".to_owned(), Some("7.5".to_owned()));
        assert_eq!(Some(attrs), document.header.map(|h| h.attributes));
    }

    #[test]
    fn trim_prefix_char_revision_line() {
        let document =
            parse("= The Intrepid Chronicles\nKismet Lee\nLPR55, {docdate}: A Special ⚄ Edition")
                .unwrap();

        assert_eq!(
            Some(Some(TitleAndAuthors {
                title: Some(Inlines::new_with_line("The Intrepid Chronicles")),
                authors: Some(vec![Author {
                    fullname: Some("Kismet Lee".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            })),
            document.header.as_ref().map(|h| h.header.clone())
        );
        let mut attrs = HashMap::new();
        attrs.insert("revnumber".to_owned(), Some("55".to_owned()));
        attrs.insert("revdate".to_owned(), Some("{docdate}".to_owned()));
        attrs.insert(
            "revremark".to_owned(),
            Some("A Special ⚄ Edition".to_owned()),
        );
        assert_eq!(Some(attrs), document.header.map(|h| h.attributes));
    }

    #[test]
    fn paragraph_blocks() {
        let document = parse("Paragraphs don't require any special markup in AsciiDoc.\nA paragraph is just one or more lines of consecutive text.\n\nTo begin a new paragraph, separate it by at least one empty line from the previous paragraph or block.").unwrap();

        assert_eq!(2, document.blocks.len());
    }
}
