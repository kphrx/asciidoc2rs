#![doc = include_str!("../README.md")]

pub mod asg;
mod parser;

use parser::{HeaderLineKind, HeaderParser, SectionLineKind, SectionParser};

use asg::block::{Author, Document, DocumentHeader, SectionBody, TitleAndAuthors};
use asg::Inlines;

use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Default)]
pub enum Doctype {
    #[default]
    Article,
    Book,
    Manpage,
}

#[derive(Debug)]
enum LineKind {
    Unknown,
    Empty,
    CommentMarker,
    CommentDelimiter(String),
    ExampleDelimiter(String),
    SidebarDelimiter(String),
    QuoteDelimiter(String),
    ListingDelimiter(String),
    LiteralDelimiter(String),
    PassthroughDelimiter(String),
    OpenDelimiter(String),
    HeadingMarker {
        level: usize,
        title: String,
    },
    UnorderedListMarker {
        marker: String,
        principal: String,
    },
    OrderedListMarker {
        marker: String,
        principal: String,
    },
    OffsetOrderedListMarker {
        offset: usize,
        principal: String,
    },
    CalloutListMarker {
        marker: String,
        principal: String,
    },
    DescriptionListMarker {
        marker: String,
        term: String,
        principal: Option<String>,
    },
    AttributeLine(Vec<String>),
}

impl LineKind {
    fn parse(line: String) -> Self {
        let line = line.trim_end_matches(' ');

        if line.is_empty() {
            return Self::Empty;
        }

        if line.starts_with("//") {
            if !line.starts_with("///") {
                return Self::CommentMarker;
            }

            if line.starts_with("////") && !line.contains(|c: char| c != '/') {
                return Self::CommentDelimiter(line.to_owned());
            }
        }

        if line.starts_with("====") && !line.contains(|c: char| c != '=') {
            return Self::ExampleDelimiter(line.to_owned());
        }

        if line.starts_with("****") && !line.contains(|c: char| c != '*') {
            return Self::SidebarDelimiter(line.to_owned());
        }

        if line.starts_with("____") && !line.contains(|c: char| c != '_') {
            return Self::QuoteDelimiter(line.to_owned());
        }

        if line.starts_with("----") && !line.contains(|c: char| c != '-') {
            return Self::ListingDelimiter(line.to_owned());
        }

        if line.starts_with("....") && !line.contains(|c: char| c != '.') {
            return Self::LiteralDelimiter(line.to_owned());
        }

        if line.starts_with("++++") && !line.contains(|c: char| c != '+') {
            return Self::PassthroughDelimiter(line.to_owned());
        }

        if line == "--" || (line.starts_with("~~~~") && !line.contains(|c: char| c != '~')) {
            return Self::OpenDelimiter(line.to_owned());
        }

        if let Some((marker, title)) = line.split_once("= ") {
            if marker.is_empty() || !marker.contains(|c: char| c != '=') {
                return Self::HeadingMarker {
                    level: marker.len(),
                    title: title.to_owned(),
                };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("* ") {
            let mut marker = list_marker.trim_start_matches(['\t', ' ']).to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            if marker.is_empty() || !marker.contains(|c: char| c != '*') {
                marker.push('*');
                return Self::UnorderedListMarker { marker, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("- ") {
            let mut marker = list_marker.trim_start_matches(['\t', ' ']).to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            // Unknown `-` list marker can be repeated infinitely.
            if marker.is_empty() {
                marker.push('-');
                return Self::UnorderedListMarker { marker, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once(". ") {
            let mut marker = list_marker.trim_start_matches(['\t', ' ']).to_owned();
            let principal = principal.trim_start_matches(' ').to_owned();
            if marker.is_empty() || !marker.contains(|c: char| c != '.') {
                marker.push('.');
                return Self::OrderedListMarker { marker, principal };
            }

            let offset: usize = marker.parse().unwrap_or(0);
            if 0 < offset && offset <= 9 {
                return Self::OffsetOrderedListMarker { offset, principal };
            }
        }

        if let Some((list_marker, principal)) = line.split_once("> ") {
            if let Some(marker) = list_marker
                .trim_start_matches(['\t', ' '])
                .strip_prefix('<')
                .to_owned()
            {
                let principal = principal.trim_start_matches(' ').to_owned();
                if marker == "." {
                    return Self::CalloutListMarker {
                        marker: "<.>".to_owned(),
                        principal,
                    };
                }

                if let Ok(number) = marker.parse::<usize>() {
                    return Self::CalloutListMarker {
                        marker: format!("<{}>", number),
                        principal,
                    };
                }
            };
        }

        if let Some((dlist_term, primary_text)) = line.split_once(":: ") {
            let dlist_term = dlist_term.trim_start_matches(['\t', ' ']);
            let term = dlist_term.trim_end_matches(':').to_owned();
            let principal = Some(primary_text.trim_start_matches(' ').to_owned());
            if !term.ends_with(' ') {
                let marker = ":".repeat(dlist_term.len() - term.len() + 2);

                return Self::DescriptionListMarker {
                    marker,
                    term,
                    principal,
                };
            }
        }

        if line.ends_with("::") {
            let dlist_term = line.trim_start_matches(['\t', ' ']);
            let term = dlist_term.trim_end_matches(':').to_owned();
            let marker = ":".repeat(dlist_term.len() - term.len());

            return Self::DescriptionListMarker {
                marker,
                term,
                principal: None,
            };
        }

        if let Some(attr_list) = line
            .strip_prefix('[')
            .and_then(|l| l.strip_suffix(']'))
            .map(|l| l.split(',').map(String::from).collect())
        {
            return Self::AttributeLine(attr_list);
        }

        Self::Unknown
    }
}

#[derive(Default)]
pub struct Parser<'input> {
    text: &'input str,
    doctype: Doctype,
    state: State,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    pub fn new_with_doctype(text: &'input str, doctype: Doctype) -> Self {
        Self {
            text,
            doctype,
            ..Default::default()
        }
    }

    pub fn parse_to_inlines(self) -> Result<Inlines, String> {
        let mut inlines = Inlines::new();
        for line in self.text.lines() {
            inlines.push(line)?;
        }

        Ok(inlines)
    }

    pub fn parse_to_asg(mut self) -> Result<Document, String> {
        for (index, line) in self.text.lines().enumerate() {
            self.push_line(line, index + 1)?;
        }
        let doc = self.finish()?;

        Ok(doc)
    }

    fn push_line(&mut self, line: &str, _num: usize) -> Result<(), String> {
        match self.state {
            State::Start => {
                if line.is_empty() || (line.starts_with("//") && !line.starts_with("///")) {
                    Ok(())
                } else {
                    self.state = State::Header(Default::default(), Default::default());
                    self.push_line(line, _num)
                }
            }
            State::Header(ref mut parser, ref mut header) => match parser.parse_line(line)? {
                HeaderLineKind::NotHeader => {
                    if matches!(self.doctype, Doctype::Manpage)
                        && header
                            .clone()
                            .and_then(|h| h.header)
                            .and_then(|t| t.title)
                            .is_none()
                    {
                        Err("require document title for doctype-manpage".into())
                    } else {
                        let doc = Document::new_with_header(header.as_deref().cloned());
                        self.state = State::Body(Default::default(), Box::new(doc));
                        self.push_line(line, _num)
                    }
                }
                HeaderLineKind::End => {
                    if matches!(self.doctype, Doctype::Manpage)
                        && header
                            .clone()
                            .and_then(|h| h.header)
                            .and_then(|t| t.title)
                            .is_none()
                    {
                        Err("require document title for doctype-manpage".into())
                    } else {
                        let doc = Document::new_with_header(header.as_deref().cloned());
                        self.state = State::Body(Default::default(), Box::new(doc));
                        Ok(())
                    }
                }
                HeaderLineKind::Title(title) => {
                    if let Some(DocumentHeader {
                        header: Some(TitleAndAuthors { title: Some(_), .. }),
                        ..
                    }) = header.as_deref()
                    {
                        Err("invalid document header".into())
                    } else if let Some(DocumentHeader {
                        header: ref mut title_and_authors,
                        ..
                    }) = header.as_deref_mut()
                    {
                        if let Some(taa) = title_and_authors {
                            taa.title = Some(Inlines::new_with_line(&title));
                            Ok(())
                        } else {
                            *title_and_authors = Some(TitleAndAuthors {
                                title: Some(Inlines::new_with_line(&title)),
                                ..Default::default()
                            });
                            Ok(())
                        }
                    } else {
                        *header = Some(Box::new(DocumentHeader {
                            header: Some(TitleAndAuthors {
                                title: Some(Inlines::new_with_line(&title)),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                        Ok(())
                    }
                }
                HeaderLineKind::Authors(authors) => {
                    let a = authors
                        .into_iter()
                        .map(|a| Author {
                            fullname: Some(a.fullname),
                            address: a.email,
                            ..Default::default()
                        })
                        .collect();
                    if let Some(DocumentHeader {
                        header: Some(ref mut title_and_authors),
                        ..
                    }) = header.as_deref_mut()
                    {
                        title_and_authors.authors = Some(a);
                        Ok(())
                    } else {
                        panic!("cannot call")
                    }
                }
                HeaderLineKind::Revision(number, date, remark) => {
                    if let Some(DocumentHeader {
                        attributes: ref mut attrs,
                        ..
                    }) = header.as_deref_mut()
                    {
                        attrs.insert("revnumber".to_string(), Some(number));
                        if let Some(d) = date {
                            attrs.insert("revdate".to_string(), Some(d));
                        }
                        if let Some(r) = remark {
                            attrs.insert("revremark".to_string(), Some(r));
                        }
                    } else {
                        panic!("cannot call")
                    }
                    Ok(())
                }
                HeaderLineKind::Attribute(name, value) => {
                    if let Some(DocumentHeader {
                        attributes: ref mut attrs,
                        ..
                    }) = header.as_deref_mut()
                    {
                        attrs.insert(name, Some(value));
                    } else {
                        let mut attrs = HashMap::new();
                        attrs.insert(name, Some(value));
                        *header = Some(Box::new(DocumentHeader {
                            attributes: attrs,
                            ..Default::default()
                        }));
                    }
                    Ok(())
                }
                HeaderLineKind::UnsetAttribute(name) => {
                    /*
                     * NOTE: In JSON Schema, the attribute allows either `string` or `null`. `false` is not allowed, and it is unclear whether the string `"false"` is what it is intended to mean, so this case is treated as a "hard unset".
                     *
                     * > Attributes set via the API can have the value `false` or `null` (or equivalent for the programming language, such as `nil` or `undefined`).
                     * > - The value `false` means to soft unset the attribute on the document.
                     * > - The value `null` means to hard unset the attribute on the document.
                     */
                    if let Some(DocumentHeader {
                        attributes: ref mut attrs,
                        ..
                    }) = header.as_deref_mut()
                    {
                        attrs.insert(name, None);
                    } else {
                        let mut attrs = HashMap::new();
                        attrs.insert(name, None);
                        *header = Some(Box::new(DocumentHeader {
                            attributes: attrs,
                            ..Default::default()
                        }));
                    }
                    Ok(())
                }
                HeaderLineKind::ClearAttribute(name) => {
                    if let Some(DocumentHeader {
                        attributes: ref mut attrs,
                        ..
                    }) = header.as_deref_mut()
                    {
                        attrs.remove(&name);
                    } else {
                        *header = Some(Default::default());
                    }
                    Ok(())
                }
                HeaderLineKind::Wrap | HeaderLineKind::Comment => Ok(()),
            },
            State::Body(ref mut parser, ref mut doc) => match parser.parse_line(line, 0)? {
                SectionLineKind::End(0, title) => match self.doctype {
                    Doctype::Book => {
                        *parser = SectionParser::Section(
                            Default::default(),
                            0,
                            title,
                            Vec::with_capacity(0),
                        );
                        Ok(())
                    }
                    _ => Err("level 0 sections can only be used when doctype is book".to_string()),
                },
                SectionLineKind::End(..) => panic!("cannot call"),
                SectionLineKind::SectionEnd(section, 0, title) => match self.doctype {
                    Doctype::Book => {
                        doc.blocks.push(SectionBody::Section(section));
                        *parser = SectionParser::Section(
                            Default::default(),
                            0,
                            title,
                            Vec::with_capacity(0),
                        );
                        Ok(())
                    }
                    _ => Err("level 0 sections can only be used when doctype is book".to_string()),
                },
                SectionLineKind::SectionEnd(section, l, title) => {
                    doc.blocks.push(SectionBody::Section(section));
                    *parser =
                        SectionParser::Section(Default::default(), l, title, Vec::with_capacity(0));
                    Ok(())
                }
                SectionLineKind::NestedEnd(block) => {
                    doc.blocks.push(block);
                    Ok(())
                }
                SectionLineKind::Skip | SectionLineKind::Comment | SectionLineKind::Nest => Ok(()),
            },
        }
    }

    fn finish(self) -> Result<Document, String> {
        match self.state {
            State::Start => {
                if matches!(self.doctype, Doctype::Manpage) {
                    Err("require document title for doctype-manpage".to_string())
                } else {
                    Ok(Default::default())
                }
            }
            State::Header(_, header) => {
                if matches!(self.doctype, Doctype::Manpage)
                    && header
                        .clone()
                        .and_then(|h| h.header)
                        .and_then(|t| t.title)
                        .is_none()
                {
                    Err("require document title for doctype-manpage".to_string())
                } else {
                    Ok(Document::new_with_header(header.as_deref().cloned()))
                }
            }
            State::Body(parser, mut doc) => {
                if let Some(body) = parser.finish() {
                    doc.blocks.push(body);
                }
                Ok(*doc)
            }
        }
    }

    pub fn parse_from_asg(text: &'input str) -> Result<Document, Box<dyn Error>> {
        let doc = serde_json::from_str(text)?;

        Ok(doc)
    }
}

#[derive(Default)]
enum State {
    #[default]
    Start,
    Header(HeaderParser, Option<Box<DocumentHeader>>),
    Body(SectionParser, Box<Document>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_kind_empty_or_comment() {
        assert!(matches!(LineKind::parse("".to_owned()), LineKind::Empty));
        assert!(matches!(
            LineKind::parse("// inline comment".to_owned()),
            LineKind::CommentMarker
        ));
        assert!(
            matches!(LineKind::parse("/////".to_owned()), LineKind::CommentDelimiter(x) if x == "/////")
        );
    }

    #[test]
    fn line_kind_section() {
        assert!(
            matches!(LineKind::parse("= heading level 0".to_owned()), LineKind::HeadingMarker { level, title } if level == 0 && title == "heading level 0")
        );
        assert!(
            matches!(LineKind::parse("== heading level 1".to_owned()), LineKind::HeadingMarker { level, title } if level == 1 && title == "heading level 1")
        );
        assert!(
            matches!(LineKind::parse("=== heading level 2".to_owned()), LineKind::HeadingMarker { level, title } if level == 2 && title == "heading level 2")
        );
        assert!(
            matches!(LineKind::parse("==== heading level 3".to_owned()), LineKind::HeadingMarker { level, title } if level == 3 && title == "heading level 3")
        );
        assert!(
            matches!(LineKind::parse("===== heading level 4".to_owned()), LineKind::HeadingMarker { level, title } if level == 4 && title == "heading level 4")
        );
        assert!(
            matches!(LineKind::parse("====== heading level 5".to_owned()), LineKind::HeadingMarker { level, title } if level == 5 && title == "heading level 5")
        );
        assert!(
            matches!(LineKind::parse("======= heading level 6".to_owned()), LineKind::HeadingMarker { level, title } if level == 6 && title == "heading level 6")
        );
    }

    #[test]
    fn line_kind_unordered_list() {
        assert!(
            matches!(LineKind::parse("** unordered list".to_owned()), LineKind::UnorderedListMarker { marker, principal } if marker == "**" && principal == "unordered list")
        );
        assert!(
            matches!(LineKind::parse("- unordered list".to_owned()), LineKind::UnorderedListMarker { marker, principal } if marker == "-" && principal == "unordered list")
        );
    }

    #[test]
    fn line_kind_ordered_list() {
        assert!(
            matches!(LineKind::parse("\t\t... ordered list".to_owned()), LineKind::OrderedListMarker { marker, principal } if marker == "..." && principal == "ordered list")
        );
        assert!(
            matches!(LineKind::parse("  5. ordered list".to_owned()), LineKind::OffsetOrderedListMarker { offset, principal } if offset == 5 && principal == "ordered list")
        );
    }

    #[test]
    fn line_kind_dlist() {
        assert!(
            matches!(LineKind::parse(" term::".to_owned()), LineKind::DescriptionListMarker { marker, term, principal } if marker == "::" && term == "term" && principal.is_none())
        );
        assert!(
            matches!(LineKind::parse("term:: description list".to_owned()), LineKind::DescriptionListMarker { marker, term, principal } if marker == "::" && term == "term" && principal == Some("description list".to_owned()))
        );
    }

    #[test]
    fn line_kind_callout() {
        assert!(
            matches!(LineKind::parse("<.> callout list".to_owned()), LineKind::CalloutListMarker { marker, principal } if marker == "<.>" && principal == "callout list")
        );
        assert!(
            matches!(LineKind::parse("<3> callout list".to_owned()), LineKind::CalloutListMarker { marker, principal } if marker == "<3>" && principal == "callout list")
        );
    }

    #[test]
    fn line_kind_block_delimiter() {
        assert!(
            matches!(LineKind::parse("====".to_owned()), LineKind::ExampleDelimiter(x) if x == "====")
        );

        assert!(
            matches!(LineKind::parse("****".to_owned()), LineKind::SidebarDelimiter(x) if x == "****")
        );

        assert!(
            matches!(LineKind::parse("____".to_owned()), LineKind::QuoteDelimiter(x) if x == "____")
        );

        assert!(
            matches!(LineKind::parse("----".to_owned()), LineKind::ListingDelimiter(x) if x == "----")
        );

        assert!(
            matches!(LineKind::parse("....".to_owned()), LineKind::LiteralDelimiter(x) if x == "....")
        );

        assert!(
            matches!(LineKind::parse("++++".to_owned()), LineKind::PassthroughDelimiter(x) if x == "++++")
        );

        assert!(
            matches!(LineKind::parse("~~~~".to_owned()), LineKind::OpenDelimiter(x) if x == "~~~~")
        );
        assert!(
            matches!(LineKind::parse("--".to_owned()), LineKind::OpenDelimiter(x) if x == "--")
        );
    }
}
