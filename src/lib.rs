#![doc = include_str!("../README.md")]

pub mod asg;
mod parser;

use parser::{HeaderLineKind, HeaderParser};

use asg::block::{Author, Document, DocumentHeader};
use asg::Inlines;

use std::collections::HashMap;
use std::error::Error;

use crate::asg::block::TitleAndAuthors;

#[derive(Debug, Clone, Default)]
pub enum Doctype {
    #[default]
    Article,
    Book,
    Manpage,
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

    pub fn parse_to_inlines(self) -> Result<Inlines, Box<dyn Error>> {
        let mut inlines = Inlines::new();
        for line in self.text.lines() {
            inlines.push(line)?;
        }

        Ok(inlines)
    }

    pub fn parse_to_asg(mut self) -> Result<Document, Box<dyn Error>> {
        for (index, line) in self.text.lines().enumerate() {
            self.push_line(line, index + 1)?;
        }
        let doc = self.finish()?;

        Ok(doc)
    }

    fn push_line(&mut self, line: &str, _num: usize) -> Result<(), Box<dyn Error>> {
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
                        let mut doc = Document::new(self.doctype.clone());
                        doc.header = header.as_deref().cloned();
                        self.state = State::Body(Box::new(doc));
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
                        let mut doc = Document::new(self.doctype.clone());
                        doc.header = header.as_deref().cloned();
                        self.state = State::Body(Box::new(doc));
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
            State::Body(ref mut doc) => doc.push(line),
        }
    }

    fn finish(self) -> Result<Document, Box<dyn Error>> {
        match self.state {
            State::Start => {
                if matches!(self.doctype, Doctype::Manpage) {
                    Err("require document title for doctype-manpage".into())
                } else {
                    Ok(Document::new(self.doctype))
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
                    Err("require document title for doctype-manpage".into())
                } else {
                    let mut doc = Document::new(self.doctype);
                    doc.header = header.as_deref().cloned();
                    doc.end();
                    Ok(doc)
                }
            }
            State::Body(mut doc) => {
                doc.end();
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
    Body(Box<Document>),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &'static str) -> Result<Document, Box<dyn Error>> {
        let mut parser = Parser::new_with_doctype(text, Doctype::Article);

        for (index, line) in text.lines().enumerate() {
            parser.push_line(line, index)?;
        }
        let document = parser.finish()?;

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
