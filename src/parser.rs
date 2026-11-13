use std::collections::HashMap;
use std::mem;

use crate::asg::block::{AnyList, Block, DiscreteHeading, Section, SectionBody};
use crate::LineKind;

#[derive(Debug, Default)]
pub(crate) enum HeaderParser {
    #[default]
    Line,
    AuthorLine,
    RevisionLine,
    Wrapped(String, String),
}
impl HeaderParser {
    pub(crate) fn parse_line(&mut self, line: &str) -> Result<HeaderLineKind, String> {
        match self {
            Self::Line => match LineKind::parse(line) {
                LineKind::Empty => Ok(HeaderLineKind::End),
                LineKind::CommentMarker => Ok(HeaderLineKind::Comment),
                LineKind::ClearDocumentAttribute(attr_name) => {
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::ClearAttribute(name))
                }
                LineKind::UnsetDocumentAttribute(attr_name) => {
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::UnsetAttribute(name))
                }
                LineKind::BooleanDocumentAttribute(attr_name) => {
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::Attribute(name, "".to_string()))
                }
                LineKind::DocumentAttribute(attr_name, attr_value) => {
                    let name = attr_name.to_lowercase();
                    if let Some(wrapped_value) = attr_value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = attr_value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        Ok(HeaderLineKind::Attribute(name, attr_value))
                    }
                }
                LineKind::HeadingMarker { level: 0, title } => {
                    *self = Self::AuthorLine;
                    Ok(HeaderLineKind::Title(title))
                }
                _ => Ok(HeaderLineKind::NotHeader),
            },
            Self::AuthorLine => match LineKind::parse(line) {
                LineKind::Empty => {
                    *self = Self::Line;
                    Ok(HeaderLineKind::End)
                }
                LineKind::CommentMarker => Ok(HeaderLineKind::Comment),
                LineKind::ClearDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::ClearAttribute(name))
                }
                LineKind::UnsetDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::UnsetAttribute(name))
                }
                LineKind::BooleanDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::Attribute(name, "".to_string()))
                }
                LineKind::DocumentAttribute(attr_name, attr_value) => {
                    let name = attr_name.to_lowercase();
                    if let Some(wrapped_value) = attr_value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = attr_value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        *self = Self::Line;
                        Ok(HeaderLineKind::Attribute(name, attr_value))
                    }
                }
                _ => {
                    *self = Self::RevisionLine;
                    Ok(HeaderLineKind::Authors(Self::parse_author_line(line)?))
                }
            },
            Self::RevisionLine => match LineKind::parse(line) {
                LineKind::Empty => {
                    *self = Self::Line;
                    Ok(HeaderLineKind::End)
                }
                LineKind::CommentMarker => Ok(HeaderLineKind::Comment),
                LineKind::ClearDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::ClearAttribute(name))
                }
                LineKind::UnsetDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::UnsetAttribute(name))
                }
                LineKind::BooleanDocumentAttribute(attr_name) => {
                    *self = Self::Line;
                    let name = attr_name.to_lowercase();
                    Ok(HeaderLineKind::Attribute(name, "".to_string()))
                }
                LineKind::DocumentAttribute(attr_name, attr_value) => {
                    let name = attr_name.to_lowercase();
                    if let Some(wrapped_value) = attr_value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = attr_value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        *self = Self::Line;
                        Ok(HeaderLineKind::Attribute(name, attr_value))
                    }
                }
                _ => {
                    *self = Self::Line;
                    Self::parse_revision_line(line)
                }
            },
            Self::Wrapped(name, value) => {
                if let Some(wrapped_value) = line.strip_suffix(" + \\") {
                    value.push_str(wrapped_value);
                    value.push('\n');
                    Ok(HeaderLineKind::Wrap)
                } else if let Some(wrapped_value) = line.strip_suffix(" \\") {
                    value.push_str(wrapped_value);
                    value.push(' ');
                    Ok(HeaderLineKind::Wrap)
                } else {
                    value.push_str(line);
                    let result = HeaderLineKind::Attribute(name.to_string(), value.to_string());
                    *self = Self::Line;
                    Ok(result)
                }
            }
        }
    }

    fn parse_author_line(line: &str) -> Result<Vec<Author>, String> {
        line.split_terminator(';')
            .map(|a| match a.split_once(" <") {
                Some((a, e)) => {
                    if let Some(email) = e.strip_suffix('>') {
                        Ok(Author {
                            fullname: a.trim().to_string(),
                            email: Some(email.to_owned()),
                        })
                    } else {
                        Err("invalid author format".to_string())
                    }
                }
                None => Ok(Author {
                    fullname: a.trim().to_string(),
                    email: None,
                }),
            })
            .collect()
    }

    fn parse_revision_line(line: &str) -> Result<HeaderLineKind, String> {
        match line.split_once(", ") {
            None => match line.split_once(": ") {
                None => {
                    if let Some(number) = line.strip_prefix('v') {
                        if number.contains(|c: char| c.is_ascii_digit()) {
                            Ok(HeaderLineKind::Revision(
                                number
                                    .trim_start_matches(|c: char| !c.is_ascii_digit())
                                    .trim_end()
                                    .to_string(),
                                None,
                                None,
                            ))
                        } else {
                            Err("invalid revision format".to_string())
                        }
                    } else {
                        Err("invalid revision format".to_string())
                    }
                }
                Some((n, r)) => {
                    if n.contains(|c: char| c.is_ascii_digit()) {
                        Ok(HeaderLineKind::Revision(
                            n.trim_start_matches(|c: char| !c.is_ascii_digit())
                                .trim_end()
                                .to_string(),
                            None,
                            Some(r.to_string()),
                        ))
                    } else {
                        Err("invalid revision format".to_string())
                    }
                }
            },
            Some((n, d)) => {
                if n.contains(|c: char| c.is_ascii_digit()) {
                    match d.split_once(": ") {
                        None => Ok(HeaderLineKind::Revision(
                            n.trim_start_matches(|c: char| !c.is_ascii_digit())
                                .trim_end()
                                .to_string(),
                            Some(d.to_string()),
                            None,
                        )),
                        Some((d, r)) => Ok(HeaderLineKind::Revision(
                            n.trim_start_matches(|c: char| !c.is_ascii_digit())
                                .trim_end()
                                .to_string(),
                            Some(d.to_string()),
                            Some(r.to_string()),
                        )),
                    }
                } else {
                    Err("invalid revision format".to_string())
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum HeaderLineKind {
    NotHeader,
    End,
    Comment,
    Title(String),
    Authors(Vec<Author>),
    Revision(String, Option<String>, Option<String>),
    Attribute(String, String),
    UnsetAttribute(String),
    ClearAttribute(String),
    Wrap,
}

#[derive(Debug)]
pub(crate) struct Author {
    pub(crate) fullname: String,
    pub(crate) email: Option<String>,
}

#[derive(Debug, Default)]
pub(crate) enum SectionParser {
    #[default]
    Line,
    Attributed(Box<[String]>, HashMap<String, String>, String),
    MasqueradingParent(
        ParentVariant,
        Box<[String]>,
        HashMap<String, String>,
        String,
    ),
    MasqueradingLeaf(LeafVariant, Box<[String]>, HashMap<String, String>, String),
    MasqueradingComment(Box<[String]>, HashMap<String, String>, String),
    Paragraph(String),
    AttributedParagraph(Box<[String]>, HashMap<String, String>, String),
    MasqueradingParagraph(LeafVariant, Box<[String]>, HashMap<String, String>, String),
    List(ListVariant, String, Vec<String>),
    Dlist(String, Vec<(String, Option<String>)>),
    Section(Box<SectionParser>, usize, String, Vec<SectionBody>),
    DelimitedComment(String),
    Delimited(Box<ParentParser>, ParentVariant, String, Vec<Block>),
    DelimitedLeaf(LeafVariant, String, Vec<String>),
}
impl SectionParser {
    pub(crate) fn parse_line(
        &mut self,
        line: &str,
        level: usize,
    ) -> Result<SectionLineKind, String> {
        match self {
            Self::Line => match LineKind::parse(line) {
                LineKind::Empty | LineKind::CommentMarker => Ok(SectionLineKind::Skip),
                LineKind::CommentDelimiter(delimiter) => {
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::Comment)
                }
                LineKind::HeadingMarker { level: l, title } => {
                    if level >= l {
                        Ok(SectionLineKind::End(l, title))
                    } else if level + 1 == l {
                        *self = Self::Section(Default::default(), l, title, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    } else {
                        Err("cannot skip section level".to_string())
                    }
                }
                LineKind::AttributeLine(positional, named) => {
                    *self = match positional.first().map(String::as_str) {
                        Some("CAUTION") => Self::MasqueradingParent(
                            ParentVariant::Caution,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("IMPORTANT") => Self::MasqueradingParent(
                            ParentVariant::Important,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("NOTE") => Self::MasqueradingParent(
                            ParentVariant::Note,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("TIP") => Self::MasqueradingParent(
                            ParentVariant::Tip,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("WARNING") => Self::MasqueradingParent(
                            ParentVariant::Warning,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("example") => Self::MasqueradingParent(
                            ParentVariant::Example,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("sidebar") => Self::MasqueradingParent(
                            ParentVariant::Sidebar,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("quote") => Self::MasqueradingParent(
                            ParentVariant::Quote,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("open") => Self::MasqueradingParent(
                            ParentVariant::Open,
                            positional,
                            named,
                            line.to_string(),
                        ),

                        Some("listing") => Self::MasqueradingLeaf(
                            LeafVariant::Listing,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("literal") => Self::MasqueradingLeaf(
                            LeafVariant::Literal,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("pass") => Self::MasqueradingLeaf(
                            LeafVariant::Passthrough,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("source") => Self::MasqueradingLeaf(
                            LeafVariant::Source,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("stem") => Self::MasqueradingLeaf(
                            LeafVariant::Stem,
                            positional,
                            named,
                            line.to_string(),
                        ),
                        Some("verse") => Self::MasqueradingLeaf(
                            LeafVariant::Verse,
                            positional,
                            named,
                            line.to_string(),
                        ),

                        Some("comment") => {
                            Self::MasqueradingComment(positional, named, line.to_string())
                        }
                        _ => Self::Attributed(positional, named, line.to_string()),
                    };
                    Ok(SectionLineKind::Nest)
                }
                LineKind::ExampleDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Example,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::SidebarDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Sidebar,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::QuoteDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Quote,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::OpenDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Open,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::ListingDelimiter(delimiter) => {
                    *self =
                        Self::DelimitedLeaf(LeafVariant::Listing, delimiter, Vec::with_capacity(0));
                    Ok(SectionLineKind::Nest)
                }
                LineKind::LiteralDelimiter(delimiter) => {
                    *self =
                        Self::DelimitedLeaf(LeafVariant::Literal, delimiter, Vec::with_capacity(0));
                    Ok(SectionLineKind::Nest)
                }
                LineKind::PassthroughDelimiter(delimiter) => {
                    *self = Self::DelimitedLeaf(
                        LeafVariant::Passthrough,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::UnorderedListMarker { marker, principal } => {
                    *self = Self::List(ListVariant::Unordered, marker, vec![principal]);
                    Ok(SectionLineKind::Nest)
                }
                LineKind::OrderedListMarker { marker, principal } => {
                    *self = Self::List(ListVariant::Ordered, marker, vec![principal]);
                    Ok(SectionLineKind::Nest)
                }
                LineKind::OffsetOrderedListMarker { offset, principal } => {
                    *self = Self::List(
                        ListVariant::Ordered,
                        format!("{}.", offset),
                        vec![principal],
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::CalloutListMarker { marker, principal } => {
                    *self = Self::List(ListVariant::Callout, marker, vec![principal]);
                    Ok(SectionLineKind::Nest)
                }
                LineKind::DescriptionListMarker {
                    marker,
                    term,
                    principal,
                } => {
                    *self = Self::Dlist(marker, vec![(term, principal)]);
                    Ok(SectionLineKind::Nest)
                }
                LineKind::Unknown
                | LineKind::ClearDocumentAttribute(_)
                | LineKind::UnsetDocumentAttribute(_)
                | LineKind::BooleanDocumentAttribute(_)
                | LineKind::DocumentAttribute(_, _) => {
                    *self = Self::Paragraph(line.to_string());
                    Ok(SectionLineKind::Nest)
                }
            },
            Self::Attributed(positional, named, _raw) => match LineKind::parse(line) {
                LineKind::Empty => {
                    *self = Self::Line;
                    Err("attribute lists deny?".to_string())
                }
                LineKind::CommentMarker => {
                    eprintln!("WARN: attributed comment line");
                    *self = Self::Line;
                    Ok(SectionLineKind::Comment)
                }
                LineKind::CommentDelimiter(delimiter) => {
                    eprintln!("WARN: attributed comment block");
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::Comment)
                }
                LineKind::ExampleDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Example,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::SidebarDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Sidebar,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::QuoteDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Quote,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::OpenDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Open,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::ListingDelimiter(delimiter) => {
                    *self =
                        Self::DelimitedLeaf(LeafVariant::Listing, delimiter, Vec::with_capacity(0));
                    Ok(SectionLineKind::Nest)
                }
                LineKind::LiteralDelimiter(delimiter) => {
                    *self =
                        Self::DelimitedLeaf(LeafVariant::Literal, delimiter, Vec::with_capacity(0));
                    Ok(SectionLineKind::Nest)
                }
                LineKind::PassthroughDelimiter(delimiter) => {
                    *self = Self::DelimitedLeaf(
                        LeafVariant::Passthrough,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::Unknown => {
                    *self = Self::AttributedParagraph(
                        mem::take(positional),
                        mem::take(named),
                        line.to_string(),
                    );
                    Ok(SectionLineKind::Nest)
                }
                _ => todo!(),
            },
            Self::MasqueradingParent(variant, _positional, _named, _raw) => {
                match (LineKind::parse(line), *variant) {
                    (LineKind::Empty, _) => {
                        *self = Self::Line;
                        Err("attribute lists deny?".to_string())
                    }
                    (LineKind::CommentMarker, _) => {
                        eprintln!("WARN: masquerading comment line?");
                        *self = Self::Line;
                        Ok(SectionLineKind::Comment)
                    }
                    (LineKind::CommentDelimiter(delimiter), _) => {
                        eprintln!("WARN: masquerading comment block?");
                        *self = Self::DelimitedComment(delimiter);
                        Ok(SectionLineKind::Comment)
                    }
                    (
                        LineKind::ExampleDelimiter(delimiter),
                        v @ (ParentVariant::Caution
                        | ParentVariant::Important
                        | ParentVariant::Note
                        | ParentVariant::Tip
                        | ParentVariant::Warning
                        | ParentVariant::Example),
                    ) => {
                        *self = Self::Delimited(
                            Default::default(),
                            v,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::SidebarDelimiter(delimiter), ParentVariant::Sidebar) => {
                        *self = Self::Delimited(
                            Default::default(),
                            ParentVariant::Sidebar,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::QuoteDelimiter(delimiter), ParentVariant::Quote) => {
                        *self = Self::Delimited(
                            Default::default(),
                            ParentVariant::Quote,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::OpenDelimiter(delimiter), v) => {
                        *self = Self::Delimited(
                            Default::default(),
                            v,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    _ => todo!(),
                }
            }
            Self::MasqueradingLeaf(variant, positional, named, _raw) => {
                match (LineKind::parse(line), *variant) {
                    (LineKind::Empty, _) => {
                        *self = Self::Line;
                        Err("attribute lists deny?".to_string())
                    }
                    (LineKind::CommentMarker, _) => {
                        eprintln!("WARN: masquerading comment line?");
                        *self = Self::Line;
                        Ok(SectionLineKind::Comment)
                    }
                    (LineKind::CommentDelimiter(delimiter), _) => {
                        eprintln!("WARN: masquerading comment block?");
                        *self = Self::DelimitedComment(delimiter);
                        Ok(SectionLineKind::Comment)
                    }
                    (LineKind::QuoteDelimiter(delimiter), LeafVariant::Verse) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Verse,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::OpenDelimiter(delimiter),
                        v @ (LeafVariant::Listing
                        | LeafVariant::Source
                        | LeafVariant::Literal
                        | LeafVariant::Passthrough
                        | LeafVariant::Verse),
                    ) => {
                        *self = Self::DelimitedLeaf(v, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::ListingDelimiter(delimiter),
                        v @ (LeafVariant::Listing | LeafVariant::Source | LeafVariant::Literal),
                    ) => {
                        *self = Self::DelimitedLeaf(v, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::LiteralDelimiter(delimiter),
                        v @ (LeafVariant::Listing | LeafVariant::Literal),
                    ) => {
                        *self = Self::DelimitedLeaf(v, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::PassthroughDelimiter(delimiter),
                        v @ (LeafVariant::Passthrough | LeafVariant::Stem),
                    ) => {
                        *self = Self::DelimitedLeaf(v, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::Unknown, v) => {
                        *self = Self::MasqueradingParagraph(
                            v,
                            mem::take(positional),
                            mem::take(named),
                            line.to_string(),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    _ => todo!(),
                }
            }
            Self::MasqueradingComment(_positional, _named, _raw) => match LineKind::parse(line) {
                LineKind::Empty => {
                    *self = Self::Line;
                    Err("attribute lists deny?".to_string())
                }
                LineKind::CommentMarker => {
                    *self = Self::Line;
                    Ok(SectionLineKind::Comment)
                }
                LineKind::CommentDelimiter(delimiter) => {
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::Comment)
                }
                LineKind::OpenDelimiter(delimiter) => {
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::Comment)
                }
                _ => todo!(),
            },
            Self::Paragraph(l) => match LineKind::parse(line) {
                LineKind::Empty => {
                    let p = Block::new_paragraph(l);
                    *self = Self::Line;
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(p)))
                }
                LineKind::CommentMarker => Ok(SectionLineKind::Skip),
                LineKind::CommentDelimiter(delimiter) => {
                    let p = Block::new_paragraph(l);
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(p)))
                }
                _ => {
                    l.push('\n');
                    l.push_str(line);
                    Ok(SectionLineKind::Nest)
                }
            },
            Self::AttributedParagraph(_positional, _named, l) => match LineKind::parse(line) {
                LineKind::Empty => {
                    let p = Block::new_paragraph(l);
                    *self = Self::Line;
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(p)))
                }
                LineKind::CommentMarker => Ok(SectionLineKind::Skip),
                LineKind::CommentDelimiter(delimiter) => {
                    let p = Block::new_paragraph(l);
                    *self = Self::DelimitedComment(delimiter);
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(p)))
                }
                _ => {
                    l.push('\n');
                    l.push_str(line);
                    Ok(SectionLineKind::Nest)
                }
            },
            Self::MasqueradingParagraph(variant, _positional, _named, l) => {
                match LineKind::parse(line) {
                    LineKind::Empty => {
                        let block = variant.to_leaf(l);
                        *self = Self::Line;
                        Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                    }
                    LineKind::CommentMarker => Ok(SectionLineKind::Skip),
                    LineKind::CommentDelimiter(delimiter) => {
                        let block = variant.to_leaf(l);
                        *self = Self::DelimitedComment(delimiter);
                        Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                    }
                    _ => {
                        l.push('\n');
                        l.push_str(line);
                        Ok(SectionLineKind::Nest)
                    }
                }
            }
            Self::List(v, m, i) => {
                match LineKind::parse(line) {
                    LineKind::Empty => {
                        // TODO: Separating Lists, Compound List Items and list continuation
                        let list = Self::list_finish(v, m, mem::take(i))?;
                        *self = Self::Line;
                        Ok(SectionLineKind::NestedEnd(SectionBody::Block(Block::AnyList(list))))
                    }
                    LineKind::CommentMarker => todo!("separating lists: require surrounded by empty lines. currently not keep state"),
                    LineKind::UnorderedListMarker { marker, principal } => {
                        // TODO: Nesting List
                        if matches!(v, ListVariant::Unordered) && marker == *m {
                            i.push(principal);
                            Ok(SectionLineKind::Nest)
                        } else {
                            let list = Self::list_finish(v, m, mem::take(i))?;
                            *self = Self::List(ListVariant::Unordered, marker, vec![principal.to_string()]);
                            Ok(SectionLineKind::NestedEnd(SectionBody::Block(Block::AnyList(list))))
                        }
                    },
                    LineKind::OrderedListMarker { marker, principal } => {
                        if matches!(v, ListVariant::Ordered) && marker == *m {
                            i.push(principal);
                            Ok(SectionLineKind::Nest)
                        } else {
                            let list = Self::list_finish(v, m, mem::take(i))?;
                            *self = Self::List(ListVariant::Ordered, marker, vec![principal.to_string()]);
                            Ok(SectionLineKind::NestedEnd(SectionBody::Block(Block::AnyList(list))))
                        }
                    },
                    LineKind::OffsetOrderedListMarker { offset, principal } => {
                        let marker = format!("{}.", offset);
                        if matches!(v, ListVariant::Ordered) && marker == *m {
                            i.push(principal);
                            Ok(SectionLineKind::Nest)
                        } else {
                            let list = Self::list_finish(v, m, mem::take(i))?;
                            *self = Self::List(ListVariant::Ordered, marker, vec![principal.to_string()]);
                            Ok(SectionLineKind::NestedEnd(SectionBody::Block(Block::AnyList(list))))
                        }
                    },
                    LineKind::CalloutListMarker { marker, principal } => {
                        if matches!(v, ListVariant::Callout) && marker == *m {
                            i.push(principal);
                            Ok(SectionLineKind::Nest)
                        } else {
                            let list = Self::list_finish(v, m, mem::take(i))?;
                            *self = Self::List(ListVariant::Callout, marker, vec![principal.to_string()]);
                            Ok(SectionLineKind::NestedEnd(SectionBody::Block(Block::AnyList(list))))
                        }
                    },
                    k => todo!("parse_line SectionParser::List: LineKind::{:?}", k),
                }
            }
            Self::Dlist(m, i) => {
                // TODO: Nesting List
                match LineKind::parse(line) {
                    LineKind::DescriptionListMarker {
                        marker,
                        term,
                        principal,
                    } => {
                        if marker == *m {
                            i.push((term, principal));
                            Ok(SectionLineKind::Nest)
                        } else {
                            let mut items = i.iter_mut();
                            if let Some((t, p)) = items.next() {
                                let mut list = AnyList::new_description_list(
                                    m.to_string(),
                                    t.to_string(),
                                    p.clone(),
                                );
                                for (t, p) in items {
                                    if let Some(principal) = p {
                                        list.push(&format!("{}{} {}", t, m, principal))?;
                                    } else {
                                        list.push(&format!("{}{}", t, m))?;
                                    }
                                }
                                *self = Self::Dlist(marker, vec![(term, principal)]);
                                Ok(SectionLineKind::NestedEnd(SectionBody::Block(
                                    Block::AnyList(list),
                                )))
                            } else {
                                Err("not expected empty items of list".to_string())
                            }
                        }
                    }
                    k => todo!("parse_line SectionParser::Dlist: LineKind::{:?}", k),
                }
            }
            Self::Section(parser, sl, title, blocks) => {
                match parser.parse_line(line, *sl)? {
                    SectionLineKind::End(l, t) => {
                        let sec = Section::new_with_blocks(*sl, title, mem::take(blocks));
                        *self = Self::Line; // end
                        Ok(SectionLineKind::SectionEnd(Box::new(sec), l, t))
                    }
                    SectionLineKind::SectionEnd(nsec, l, t) => {
                        if *sl < l {
                            blocks.push(SectionBody::Section(nsec));
                            **parser = SectionParser::Section(
                                Default::default(),
                                l,
                                t,
                                Default::default(),
                            );
                            Ok(SectionLineKind::Nest)
                        } else {
                            blocks.push(SectionBody::Section(nsec));
                            let sec = Section::new_with_blocks(*sl, title, mem::take(blocks));
                            *self = Self::Line; // end
                            Ok(SectionLineKind::SectionEnd(Box::new(sec), l, t))
                        }
                    }
                    SectionLineKind::NestedEnd(body) => {
                        blocks.push(body);
                        Ok(SectionLineKind::Nest)
                    }
                    SectionLineKind::Skip | SectionLineKind::Nest | SectionLineKind::Comment => {
                        Ok(SectionLineKind::Nest)
                    }
                }
            }
            Self::DelimitedComment(delimiter) => match LineKind::parse(line) {
                LineKind::CommentDelimiter(d) | LineKind::OpenDelimiter(d) if d == *delimiter => {
                    *self = Self::Line;
                    Ok(SectionLineKind::Comment)
                }
                _ => Ok(SectionLineKind::Comment),
            },
            Self::Delimited(parser, variant, delimiter, blocks) => match parser.parse_line(line)? {
                ParentLineKind::Delimiter(d) if d == *delimiter => {
                    let block = variant.to_block(delimiter.to_string(), mem::take(blocks));
                    *self = Self::Line;
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                }
                ParentLineKind::NestedEnd(block) => {
                    blocks.push(block);
                    Ok(SectionLineKind::Nest)
                }
                ParentLineKind::Comment
                | ParentLineKind::Skip
                | ParentLineKind::Delimiter(_)
                | ParentLineKind::Nest => Ok(SectionLineKind::Nest),
            },
            Self::DelimitedLeaf(variant, delimiter, lines) => match LineKind::parse(line) {
                LineKind::QuoteDelimiter(d)
                | LineKind::ListingDelimiter(d)
                | LineKind::LiteralDelimiter(d)
                | LineKind::PassthroughDelimiter(d)
                | LineKind::OpenDelimiter(d)
                    if d == *delimiter =>
                {
                    let block =
                        variant.to_leaf_with_delimiter(delimiter.to_string(), &lines.join("\n"));
                    *self = Self::Line;
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                }
                _ => {
                    lines.push(line.to_string());
                    Ok(SectionLineKind::Nest)
                }
            },
        }
    }

    pub(crate) fn finish(self) -> Option<SectionBody> {
        match self {
            Self::Section(parser, level, title, mut blocks) => {
                if let Some(body) = parser.finish() {
                    blocks.push(body);
                }

                Some(SectionBody::Section(Box::new(Section::new_with_blocks(
                    level, &title, blocks,
                ))))
            }
            Self::Line => None,
            Self::Paragraph(line) => Some(SectionBody::Block(Block::new_paragraph(&line))),
            Self::List(v, m, i) => {
                if let Ok(list) = Self::list_finish(&v, &m, i) {
                    Some(SectionBody::Block(Block::AnyList(list)))
                } else {
                    None
                }
            }
            Self::Dlist(m, i) => {
                let mut items = i.into_iter();
                if let Some((t, p)) = items.next() {
                    let mut list = AnyList::new_description_list(m.clone(), t, p);
                    for (t, p) in items {
                        let principal = if let Some(principal) = p {
                            format!("{}{} {}", t, m, principal)
                        } else {
                            format!("{}{}", t, m)
                        };
                        if list.push(&principal).is_err() {
                            return None;
                        }
                    }
                    Some(SectionBody::Block(Block::AnyList(list)))
                } else {
                    None
                }
            }
            Self::DelimitedComment(delimiter) => {
                eprintln!("WARN: unterminated: `{}`", delimiter);
                None
            }
            Self::Delimited(parser, variant, delimiter, mut blocks) => {
                eprintln!("WARN: unterminated: `{}`", delimiter);
                if let Some(block) = parser.finish() {
                    blocks.push(block)
                }
                Some(SectionBody::Block(variant.to_block(delimiter, blocks)))
            }
            Self::DelimitedLeaf(variant, delimiter, lines) => {
                eprintln!("WARN: unterminated: `{}`", delimiter);
                Some(SectionBody::Block(
                    variant.to_leaf_with_delimiter(delimiter, &lines.join("\n")),
                ))
            }
            _ => todo!("finish: SectionParser::{:?}", self),
        }
    }

    fn list_finish(v: &ListVariant, m: &str, i: Vec<String>) -> Result<AnyList, String> {
        let mut items = i.into_iter();
        if let Some(principal) = items.next() {
            let mut list = v.to_list(m.to_string(), principal);
            for p in items {
                list.push(&format!("{} {}", m, p))?;
            }
            Ok(list)
        } else {
            Err("not expected empty items of list".to_string())
        }
    }
}

#[derive(Debug)]
pub(crate) enum SectionLineKind {
    Comment,
    Skip,
    End(usize, String),
    SectionEnd(Box<Section>, usize, String),
    Nest,
    NestedEnd(SectionBody),
}

#[derive(Debug, Default)]
pub(crate) enum ParentParser {
    #[default]
    Line,
    DelimitedComment(String),
    Delimited(Box<ParentParser>, ParentVariant, String, Vec<Block>),
}
impl ParentParser {
    fn parse_line(&mut self, line: &str) -> Result<ParentLineKind, String> {
        match self {
            Self::Line => match LineKind::parse(line) {
                LineKind::Empty | LineKind::CommentMarker => Ok(ParentLineKind::Skip),
                LineKind::CommentDelimiter(delimiter) => {
                    *self = Self::DelimitedComment(delimiter);
                    Ok(ParentLineKind::Comment)
                }
                LineKind::ExampleDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Example,
                        delimiter.clone(),
                        Vec::with_capacity(0),
                    );
                    Ok(ParentLineKind::Delimiter(delimiter))
                }
                LineKind::SidebarDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Sidebar,
                        delimiter.clone(),
                        Vec::with_capacity(0),
                    );
                    Ok(ParentLineKind::Delimiter(delimiter))
                }
                LineKind::QuoteDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Quote,
                        delimiter.clone(),
                        Vec::with_capacity(0),
                    );
                    Ok(ParentLineKind::Delimiter(delimiter))
                }
                LineKind::OpenDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        ParentVariant::Open,
                        delimiter.clone(),
                        Vec::with_capacity(0),
                    );
                    Ok(ParentLineKind::Delimiter(delimiter))
                }
                LineKind::HeadingMarker { level, title } => Ok(ParentLineKind::NestedEnd(
                    Block::Heading(DiscreteHeading::new(level, &title)),
                )),
                k => todo!("parse_line ParentParser::Line: LineKind::{:?}", k),
            },
            Self::DelimitedComment(delimiter) => match LineKind::parse(line) {
                LineKind::CommentDelimiter(d) if d == *delimiter => {
                    *self = Self::Line;
                    Ok(ParentLineKind::Comment)
                }
                _ => Ok(ParentLineKind::Comment),
            },
            Self::Delimited(parser, variant, delimiter, blocks) => match parser.parse_line(line)? {
                ParentLineKind::Delimiter(d) if d == *delimiter => {
                    let block = variant.to_block(delimiter.to_string(), mem::take(blocks));
                    *self = Self::Line;
                    Ok(ParentLineKind::NestedEnd(block))
                }
                _ => Ok(ParentLineKind::Nest),
            },
        }
    }

    fn finish(self) -> Option<Block> {
        match self {
            Self::Line => None,
            Self::DelimitedComment(delimiter) => {
                eprintln!("WARN: unterminated: `{}`", delimiter);
                None
            }
            Self::Delimited(parser, variant, delimiter, mut blocks) => {
                eprintln!("WARN: unterminated: `{}`", delimiter);
                if let Some(block) = parser.finish() {
                    blocks.push(block)
                }
                Some(variant.to_block(delimiter, blocks))
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParentLineKind {
    Comment,
    Skip,
    Delimiter(String),
    Nest,
    NestedEnd(Block),
}

#[derive(Debug)]
pub(crate) enum ListVariant {
    Callout,
    Ordered,
    Unordered,
}
impl ListVariant {
    fn to_list(&self, marker: String, principal: String) -> AnyList {
        match self {
            Self::Callout => AnyList::new_callout_list(marker, principal),
            Self::Ordered => AnyList::new_ordered_list(marker, principal),
            Self::Unordered => AnyList::new_unordered_list(marker, principal),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ParentVariant {
    Caution,
    Important,
    Note,
    Tip,
    Warning,
    Example,
    Sidebar,
    Quote,
    Open,
}
impl ParentVariant {
    fn to_block(self, delimiter: String, blocks: Vec<Block>) -> Block {
        match self {
            Self::Caution => Block::new_caution(delimiter, blocks),
            Self::Important => Block::new_important(delimiter, blocks),
            Self::Note => Block::new_note(delimiter, blocks),
            Self::Tip => Block::new_tip(delimiter, blocks),
            Self::Warning => Block::new_warning(delimiter, blocks),
            Self::Example => Block::new_example(delimiter, blocks),
            Self::Sidebar => Block::new_sidebar(delimiter, blocks),
            Self::Quote => Block::new_quote(delimiter, blocks),
            Self::Open => Block::new_open(delimiter, blocks),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum LeafVariant {
    Listing,
    Literal,
    Passthrough,
    Source,
    Stem,
    Verse,
}
impl LeafVariant {
    fn to_leaf(self, value: &str) -> Block {
        match self {
            Self::Listing => Block::new_listing(value),
            Self::Literal => Block::new_literal(value),
            Self::Passthrough => Block::new_pass(value),
            Self::Source => todo!(),
            Self::Stem => Block::new_stem(value),
            Self::Verse => Block::new_verse(value),
        }
    }

    fn to_leaf_with_delimiter(self, delimiter: String, value: &str) -> Block {
        match self {
            Self::Listing => Block::new_delimited_listing(delimiter, value),
            Self::Literal => Block::new_delimited_literal(delimiter, value),
            Self::Passthrough => Block::new_delimited_pass(delimiter, value),
            Self::Source => todo!(),
            Self::Stem => Block::new_delimited_stem(delimiter, value),
            Self::Verse => Block::new_delimited_verse(delimiter, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    mod header_parser {
        use super::*;

        #[test]
        fn line_prase_line_empty() {
            let mut p = HeaderParser::Line;
            assert_matches!(p.parse_line(""), Ok(HeaderLineKind::End));
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_title() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line("= The Intrepid Chronicles"),
                Ok(HeaderLineKind::Title(t)) if t == "The Intrepid Chronicles"
            );
            assert_matches!(p, HeaderParser::AuthorLine);
        }

        #[test]
        fn line_prase_line_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":name-of-an-attribute: value of the attribute"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v == "value of the attribute"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_preferred_unset_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":!name:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_unset_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":name!:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_clear_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":!name!:"),
                Ok(HeaderLineKind::ClearAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_boolean_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":name-of-an-attribute:"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v.is_empty()
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn line_prase_line_wrap_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":description: If you have a very long line of text \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "If you have a very long line of text "
            );
        }

        #[test]
        fn line_prase_line_wrap_return_attribute() {
            let mut p = HeaderParser::Line;
            assert_matches!(
                p.parse_line(":description: Write your docs in text, + \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "Write your docs in text,\n"
            );
        }

        #[test]
        fn author_line_prase_line_empty() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(p.parse_line(""), Ok(HeaderLineKind::End));
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_an_author() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line("Kismet R. Lee <kismet@asciidoctor.org>"),
                Ok(HeaderLineKind::Authors(a))
                    if matches!(a.as_slice(), [Author { fullname, email: Some(email) }]
                        if fullname == "Kismet R. Lee"
                        && email == "kismet@asciidoctor.org")
            );
            assert_matches!(p, HeaderParser::RevisionLine);
        }

        #[test]
        fn author_line_prase_line_multiple_authors() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line("Kismet R. Lee <kismet@asciidoctor.org>; B. Steppenwolf; Pax Draeke <pax@asciidoctor.org>"),
                Ok(HeaderLineKind::Authors(a))
                    if matches!(
                        a.as_slice(),
                        [
                            Author { fullname: fullname1, email: Some(email1) },
                            Author { fullname: fullname2, email: None },
                            Author { fullname: fullname3, email: Some(email3) },
                        ]
                            if fullname1 == "Kismet R. Lee" && email1 == "kismet@asciidoctor.org"
                            && fullname2 == "B. Steppenwolf"
                            && fullname3 == "Pax Draeke" && email3 == "pax@asciidoctor.org")
            );
            assert_matches!(p, HeaderParser::RevisionLine);
        }

        #[test]
        fn author_line_prase_line_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":name-of-an-attribute: value of the attribute"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v == "value of the attribute"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_preferred_unset_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":!name:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_unset_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":name!:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_clear_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":!name!:"),
                Ok(HeaderLineKind::ClearAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_boolean_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":name-of-an-attribute:"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v.is_empty()
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn author_line_prase_line_wrap_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":description: If you have a very long line of text \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "If you have a very long line of text "
            );
        }

        #[test]
        fn author_line_prase_line_wrap_return_attribute() {
            let mut p = HeaderParser::AuthorLine;
            assert_matches!(
                p.parse_line(":description: Write your docs in text, + \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "Write your docs in text,\n"
            );
        }

        #[test]
        fn revision_line_prase_line_empty() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(p.parse_line(""), Ok(HeaderLineKind::End));
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_revision() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line("2.9, October 31, 2021: Fall incarnation"),
                Ok(HeaderLineKind::Revision(n, Some(d), Some(r)))
                    if n == "2.9"
                    && d == "October 31, 2021"
                    && r == "Fall incarnation"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_revision_number_only() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line("v7.5"),
                Ok(HeaderLineKind::Revision(n, None, None)) if n == "7.5"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_revision_number_and_date() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line("7.5, 1-29-2020"),
                Ok(HeaderLineKind::Revision(n, Some(d), None))
                    if n == "7.5"
                    && d == "1-29-2020"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_revision_number_and_remark() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line("7.5: A new analysis"),
                Ok(HeaderLineKind::Revision(n, None, Some(r)))
                    if n == "7.5"
                    && r == "A new analysis"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_revision_trim_prefix_characters() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line("LPR55, {docdate}: A Special ⚄ Edition"),
                Ok(HeaderLineKind::Revision(n, Some(d), Some(r)))
                    if n == "55"
                    && d == "{docdate}"
                    && r == "A Special ⚄ Edition"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":name-of-an-attribute: value of the attribute"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v == "value of the attribute"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_preferred_unset_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":!name:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_unset_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":name!:"),
                Ok(HeaderLineKind::UnsetAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_clear_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":!name!:"),
                Ok(HeaderLineKind::ClearAttribute(n)) if n == "name"
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_boolean_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":name-of-an-attribute:"),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "name-of-an-attribute"
                    && v.is_empty()
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn revision_line_prase_line_wrap_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":description: If you have a very long line of text \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "If you have a very long line of text "
            );
        }

        #[test]
        fn revision_line_prase_line_wrap_return_attribute() {
            let mut p = HeaderParser::RevisionLine;
            assert_matches!(
                p.parse_line(":description: Write your docs in text, + \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "Write your docs in text,\n"
            );
        }

        #[test]
        fn wrapped_prase_line_attribute() {
            let mut p = HeaderParser::Wrapped("description".to_string(), "If you have a very long line of text that you need to substitute regularly in a document, you may find it easier to split the value neatly in the header ".to_string());
            assert_matches!(
                p.parse_line("so it remains readable to folks looking at the AsciiDoc source."),
                Ok(HeaderLineKind::Attribute(n, v))
                    if n == "description"
                    && v == "If you have a very long line of text that you need to substitute regularly in a document, you may find it easier to split the value neatly in the header so it remains readable to folks looking at the AsciiDoc source."
            );
            assert_matches!(p, HeaderParser::Line);
        }

        #[test]
        fn wrapped_prase_line_wrap_attribute() {
            let mut p = HeaderParser::Wrapped(
                "description".to_string(),
                "If you have a very long line of text ".to_string(),
            );
            assert_matches!(
                p.parse_line("that you need to substitute regularly in a document, \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "If you have a very long line of text that you need to substitute regularly in a document, "
            );
        }

        #[test]
        fn wrapped_prase_line_wrap_return_attribute() {
            let mut p = HeaderParser::Wrapped(
                "description".to_string(),
                "Write your docs in text,\n".to_string(),
            );
            assert_matches!(
                p.parse_line("AsciiDoc makes it easy, + \\"),
                Ok(HeaderLineKind::Wrap)
            );
            assert_matches!(
                p,
                HeaderParser::Wrapped(n, v)
                    if n == "description"
                    && v == "Write your docs in text,\nAsciiDoc makes it easy,\n"
            );
        }
    }

    mod section_parser {
        use super::*;

        #[test]
        fn line_prase_line_empty() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("", 0), Ok(SectionLineKind::Skip));
            assert_matches!(p, SectionParser::Line);
        }

        #[test]
        fn line_parse_line_attribute_line_caution() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[CAUTION]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Caution, positional, named, raw)
                    if matches!(&*positional, [style] if style == "CAUTION")
                    && named == HashMap::new()
                    && raw == "[CAUTION]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_important() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[IMPORTANT]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Important, positional, named, raw)
                    if matches!(&*positional, [style] if style == "IMPORTANT")
                    && named == HashMap::new()
                    && raw == "[IMPORTANT]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_note() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[NOTE]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Note, positional, named, raw)
                    if matches!(&*positional, [style] if style == "NOTE")
                    && named == HashMap::new()
                    && raw == "[NOTE]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_tip() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[TIP]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Tip, positional, named, raw)
                    if matches!(&*positional, [style] if style == "TIP")
                    && named == HashMap::new()
                    && raw == "[TIP]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_warning() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[WARNING]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Warning, positional, named, raw)
                    if matches!(&*positional, [style] if style == "WARNING")
                    && named == HashMap::new()
                    && raw == "[WARNING]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_example() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[example]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Example, positional, named, raw)
                    if matches!(&*positional, [style] if style == "example")
                    && named == HashMap::new()
                    && raw == "[example]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_sidebar() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[sidebar]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Sidebar, positional, named, raw)
                    if matches!(&*positional, [style] if style == "sidebar")
                    && named == HashMap::new()
                    && raw == "[sidebar]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_quote() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[quote]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Quote, positional, named, raw)
                    if matches!(&*positional, [style] if style == "quote")
                    && named == HashMap::new()
                    && raw == "[quote]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_open() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[open]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingParent(ParentVariant::Open, positional, named, raw)
                    if matches!(&*positional, [style] if style == "open")
                    && named == HashMap::new()
                    && raw == "[open]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_listing() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[listing]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Listing, positional, named, raw)
                    if matches!(&*positional, [style] if style == "listing")
                    && named == HashMap::new()
                    && raw == "[listing]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_literal() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[literal]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Literal, positional, named, raw)
                    if matches!(&*positional, [style] if style == "literal")
                    && named == HashMap::new()
                    && raw == "[literal]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_source() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[source]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Source, positional, named, raw)
                    if matches!(&*positional, [style] if style == "source")
                    && named == HashMap::new()
                    && raw == "[source]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_pass() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[pass]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Passthrough, positional, named, raw)
                    if matches!(&*positional, [style] if style == "pass")
                    && named == HashMap::new()
                    && raw == "[pass]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_stem() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[stem]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Stem, positional, named, raw)
                    if matches!(&*positional, [style] if style == "stem")
                    && named == HashMap::new()
                    && raw == "[stem]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_verse() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[verse]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingLeaf(LeafVariant::Verse, positional, named, raw)
                    if matches!(&*positional, [style] if style == "verse")
                    && named == HashMap::new()
                    && raw == "[verse]"
            );
        }

        #[test]
        fn line_parse_line_attribute_line_comment() {
            let mut p = SectionParser::Line;
            assert_matches!(p.parse_line("[comment]", 0), Ok(SectionLineKind::Nest));
            assert_matches!(
                p,
                SectionParser::MasqueradingComment(positional, named, raw)
                    if matches!(&*positional, [style] if style == "comment")
                    && named == HashMap::new()
                    && raw == "[comment]"
            );
        }
    }

    mod parent_parser {
        use super::*;

        #[test]
        fn line_prase_line_empty() {
            let mut p = ParentParser::Line;
            assert_matches!(p.parse_line(""), Ok(ParentLineKind::Skip));
            assert_matches!(p, ParentParser::Line);
        }
    }
}
