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
            Self::Line => {
                if line.is_empty() {
                    Ok(HeaderLineKind::End)
                } else if line.starts_with("//") && !line.starts_with("///") {
                    Ok(HeaderLineKind::Comment)
                } else if let Some((name, value)) =
                    line.strip_prefix(':').and_then(|a| a.split_once(": "))
                {
                    let name = name.to_lowercase();
                    Self::validate_attribute_name(&name)?;
                    if let Some(wrapped_value) = value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        Ok(HeaderLineKind::Attribute(name, value.to_string()))
                    }
                } else if let Some(attr_name) =
                    line.strip_prefix(':').and_then(|a| a.strip_suffix(':'))
                {
                    Self::parse_attribute_name(attr_name)
                } else if let Some(title) = line.strip_prefix("= ") {
                    let result = HeaderLineKind::Title(title.to_string());
                    *self = Self::AuthorLine;
                    Ok(result)
                } else {
                    Ok(HeaderLineKind::NotHeader)
                }
            }
            Self::AuthorLine => {
                if line.is_empty() {
                    *self = Self::Line;
                    Ok(HeaderLineKind::End)
                } else if line.starts_with("//") && !line.starts_with("///") {
                    Ok(HeaderLineKind::Comment)
                } else if let Some((name, value)) =
                    line.strip_prefix(':').and_then(|a| a.split_once(": "))
                {
                    let name = name.to_lowercase();
                    Self::validate_attribute_name(&name)?;
                    if let Some(wrapped_value) = value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        *self = Self::Line;
                        Ok(HeaderLineKind::Attribute(name, value.to_string()))
                    }
                } else if let Some(attr_name) =
                    line.strip_prefix(':').and_then(|a| a.strip_suffix(':'))
                {
                    *self = Self::Line;
                    Self::parse_attribute_name(attr_name)
                } else {
                    *self = Self::RevisionLine;
                    Ok(HeaderLineKind::Authors(Self::parse_author_line(line)?))
                }
            }
            Self::RevisionLine => {
                if line.is_empty() {
                    *self = Self::Line;
                    Ok(HeaderLineKind::End)
                } else if line.starts_with("//") && !line.starts_with("///") {
                    Ok(HeaderLineKind::Comment)
                } else if let Some((name, value)) =
                    line.strip_prefix(':').and_then(|a| a.split_once(": "))
                {
                    let name = name.to_lowercase();
                    Self::validate_attribute_name(&name)?;
                    if let Some(wrapped_value) = value.strip_suffix(" + \\") {
                        *self = Self::Wrapped(name, format!("{}\n", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else if let Some(wrapped_value) = value.strip_suffix(" \\") {
                        *self = Self::Wrapped(name, format!("{} ", wrapped_value));
                        Ok(HeaderLineKind::Wrap)
                    } else {
                        *self = Self::Line;
                        Ok(HeaderLineKind::Attribute(name, value.to_string()))
                    }
                } else if let Some(attr_name) =
                    line.strip_prefix(':').and_then(|a| a.strip_suffix(':'))
                {
                    *self = Self::Line;
                    Self::parse_attribute_name(attr_name)
                } else {
                    *self = Self::Line;
                    Self::parse_revision_line(line)
                }
            }
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

    fn validate_attribute_name(attr_name: &str) -> Result<(), String> {
        if attr_name.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            && !attr_name.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
        {
            Ok(())
        } else {
            Err(format!("invalid document attribute: {}", attr_name))
        }
    }

    fn parse_attribute_name(attr_name: &str) -> Result<HeaderLineKind, String> {
        if let Some(clear_attr) = attr_name
            .strip_prefix('!')
            .and_then(|a| a.strip_suffix('!'))
        {
            /*
             * NOTE: Asciidoctor
             *
             * > **FI** Introduce `!name!` to clear an attribute so it’s like it was never declared (this is different from unsetting it).
             */
            Self::validate_attribute_name(clear_attr)?;
            Ok(HeaderLineKind::ClearAttribute(
                clear_attr.to_lowercase().to_string(),
            ))
        } else if let Some(unset_attr) = attr_name.strip_prefix('!').or(attr_name.strip_suffix('!'))
        {
            Self::validate_attribute_name(unset_attr)?;
            Ok(HeaderLineKind::UnsetAttribute(
                unset_attr.to_lowercase().to_string(),
            ))
        } else {
            Self::validate_attribute_name(attr_name)?;
            Ok(HeaderLineKind::Attribute(
                attr_name.to_lowercase().to_string(),
                "".to_string(),
            ))
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
    Attributed(Vec<String>),
    Paragraph(String),
    List(ListVariant, String, Vec<String>),
    Dlist(String, Vec<(String, Option<String>)>),
    Section(Box<SectionParser>, usize, String, Vec<SectionBody>),
    DelimitedComment(String),
    Delimited(Box<DelimitedParser>, BlockVariant, String, Vec<Block>),
    DelimitedLeaf(LeafVariant, String, Vec<String>),
}
impl SectionParser {
    pub(crate) fn parse_line(
        &mut self,
        line: &str,
        level: usize,
    ) -> Result<SectionLineKind, String> {
        match self {
            Self::Line => match LineKind::parse(line.to_string()) {
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
                LineKind::AttributeLine(attr_list) => {
                    *self = Self::Attributed(attr_list);
                    Ok(SectionLineKind::Nest)
                }
                LineKind::ExampleDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        BlockVariant::Example,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::SidebarDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        BlockVariant::Sidebar,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::QuoteDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        BlockVariant::Quote,
                        delimiter,
                        Vec::with_capacity(0),
                    );
                    Ok(SectionLineKind::Nest)
                }
                LineKind::OpenDelimiter(delimiter) => {
                    *self = Self::Delimited(
                        Default::default(),
                        BlockVariant::Open,
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
                LineKind::Unknown => {
                    *self = Self::Paragraph(line.to_string());
                    Ok(SectionLineKind::Nest)
                }
            },
            Self::Attributed(l) => {
                let (parent_variant, leaf_variant) = match l.first().map(String::as_str) {
                    Some("CAUTION") => (Some(BlockVariant::Caution), None),
                    Some("IMPORTANT") => (Some(BlockVariant::Important), None),
                    Some("NOTE") => (Some(BlockVariant::Note), None),
                    Some("TIP") => (Some(BlockVariant::Tip), None),
                    Some("WARNING") => (Some(BlockVariant::Warning), None),
                    Some("example") => (Some(BlockVariant::Example), None),
                    Some("sidebar") => (Some(BlockVariant::Sidebar), None),
                    Some("quote") => (Some(BlockVariant::Quote), None),
                    Some("open") => (Some(BlockVariant::Open), None),

                    Some("listing") => (None, Some(LeafVariant::Listing)),
                    Some("literal") => (None, Some(LeafVariant::Literal)),
                    Some("pass") => (None, Some(LeafVariant::Passthrough)),
                    Some("source") => (None, Some(LeafVariant::Source)),
                    Some("stem") => (None, Some(LeafVariant::Stem)),
                    Some("verse") => (None, Some(LeafVariant::Verse)),

                    Some("comment") => (None, Some(LeafVariant::Comment)),
                    _ => (None, None),
                };
                match (
                    LineKind::parse(line.to_string()),
                    parent_variant,
                    leaf_variant,
                ) {
                    (LineKind::Empty | LineKind::CommentMarker, _, _) => {
                        *self = Self::Line;
                        Err("attribute lists deny?".to_string())
                    }
                    (LineKind::CommentDelimiter(delimiter), _, _) => {
                        *self = Self::DelimitedComment(delimiter);
                        Ok(SectionLineKind::Comment)
                    }
                    (
                        LineKind::ExampleDelimiter(delimiter),
                        Some(
                            variant @ (BlockVariant::Caution
                            | BlockVariant::Important
                            | BlockVariant::Note
                            | BlockVariant::Tip
                            | BlockVariant::Warning
                            | BlockVariant::Example),
                        ),
                        None,
                    ) => {
                        *self = Self::Delimited(
                            Default::default(),
                            variant,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::ExampleDelimiter(delimiter), None, None) => {
                        *self = Self::Delimited(
                            Default::default(),
                            BlockVariant::Example,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::SidebarDelimiter(delimiter),
                        Some(BlockVariant::Sidebar) | None,
                        None,
                    ) => {
                        *self = Self::Delimited(
                            Default::default(),
                            BlockVariant::Sidebar,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::QuoteDelimiter(delimiter),
                        Some(BlockVariant::Quote) | None,
                        None,
                    ) => {
                        *self = Self::Delimited(
                            Default::default(),
                            BlockVariant::Quote,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::QuoteDelimiter(delimiter), None, Some(LeafVariant::Verse)) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Verse,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::OpenDelimiter(delimiter), Some(variant), None) => {
                        *self = Self::Delimited(
                            Default::default(),
                            variant,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::OpenDelimiter(delimiter), None, None) => {
                        *self = Self::Delimited(
                            Default::default(),
                            BlockVariant::Open,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::OpenDelimiter(delimiter),
                        None,
                        Some(
                            variant @ (LeafVariant::Listing
                            | LeafVariant::Literal
                            | LeafVariant::Source
                            | LeafVariant::Passthrough
                            | LeafVariant::Verse),
                        ),
                    ) => {
                        *self = Self::DelimitedLeaf(variant, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::OpenDelimiter(delimiter), None, Some(LeafVariant::Comment)) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Comment,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Comment)
                    }
                    (
                        LineKind::ListingDelimiter(delimiter),
                        None,
                        Some(variant @ (LeafVariant::Listing | LeafVariant::Literal)),
                    ) => {
                        *self = Self::DelimitedLeaf(variant, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::ListingDelimiter(delimiter), None, None) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Listing,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::LiteralDelimiter(delimiter),
                        None,
                        Some(variant @ (LeafVariant::Listing | LeafVariant::Literal)),
                    ) => {
                        *self = Self::DelimitedLeaf(variant, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::LiteralDelimiter(delimiter), None, None) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Literal,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    (
                        LineKind::PassthroughDelimiter(delimiter),
                        None,
                        Some(variant @ (LeafVariant::Passthrough | LeafVariant::Stem)),
                    ) => {
                        *self = Self::DelimitedLeaf(variant, delimiter, Vec::with_capacity(0));
                        Ok(SectionLineKind::Nest)
                    }
                    (LineKind::PassthroughDelimiter(delimiter), None, None) => {
                        *self = Self::DelimitedLeaf(
                            LeafVariant::Passthrough,
                            delimiter,
                            Vec::with_capacity(0),
                        );
                        Ok(SectionLineKind::Nest)
                    }
                    _ => todo!(),
                }
            }
            Self::Paragraph(l) => match LineKind::parse(line.to_string()) {
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
            Self::List(v, m, i) => {
                match LineKind::parse(line.to_string()) {
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
                match LineKind::parse(line.to_string()) {
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
            Self::DelimitedComment(delimiter) => match LineKind::parse(line.to_string()) {
                LineKind::CommentDelimiter(d) if d == *delimiter => {
                    *self = Self::Line;
                    Ok(SectionLineKind::Comment)
                }
                _ => Ok(SectionLineKind::Comment),
            },
            Self::Delimited(parser, variant, delimiter, blocks) => match parser.parse_line(line)? {
                DelimitedLineKind::Delimiter(d) if d == *delimiter => {
                    let block = variant.to_block(delimiter.to_string(), mem::take(blocks));
                    *self = Self::Line;
                    Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                }
                _ => Ok(SectionLineKind::Nest),
            },
            Self::DelimitedLeaf(variant, delimiter, lines) => {
                match LineKind::parse(line.to_string()) {
                    LineKind::QuoteDelimiter(d)
                    | LineKind::ListingDelimiter(d)
                    | LineKind::LiteralDelimiter(d)
                    | LineKind::PassthroughDelimiter(d)
                    | LineKind::OpenDelimiter(d)
                        if d == *delimiter =>
                    {
                        let block = variant.to_block(delimiter.to_string(), &lines.join("\n"));
                        *self = Self::Line;
                        if let Some(block) = block {
                            Ok(SectionLineKind::NestedEnd(SectionBody::Block(block)))
                        } else {
                            Ok(SectionLineKind::Comment)
                        }
                    }
                    _ => match variant {
                        LeafVariant::Comment => Ok(SectionLineKind::Comment),
                        _ => {
                            lines.push(line.to_string());
                            Ok(SectionLineKind::Nest)
                        }
                    },
                }
            }
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
pub(crate) enum DelimitedParser {
    #[default]
    Line,
    DelimitedComment(String),
    Delimited(Box<DelimitedParser>, BlockVariant, String, Vec<Block>),
}
impl DelimitedParser {
    pub(crate) fn parse_line(&mut self, line: &str) -> Result<DelimitedLineKind, String> {
        match self {
            Self::Line => match LineKind::parse(line.to_string()) {
                LineKind::Empty | LineKind::CommentMarker => Ok(DelimitedLineKind::Skip),
                LineKind::CommentDelimiter(delimiter) => {
                    *self = Self::DelimitedComment(delimiter);
                    Ok(DelimitedLineKind::Comment)
                }
                LineKind::HeadingMarker { level, title } => Ok(DelimitedLineKind::NestedEnd(
                    Block::Heading(DiscreteHeading::new(level, &title)),
                )),
                k => todo!("parse_line DelimitedParser::Line: LineKind::{:?}", k),
            },
            Self::DelimitedComment(delimiter) => match LineKind::parse(line.to_string()) {
                LineKind::CommentDelimiter(d) if d == *delimiter => {
                    *self = Self::Line;
                    Ok(DelimitedLineKind::Comment)
                }
                _ => Ok(DelimitedLineKind::Comment),
            },
            Self::Delimited(parser, variant, delimiter, blocks) => match parser.parse_line(line)? {
                DelimitedLineKind::Delimiter(d) if d == *delimiter => {
                    let block = variant.to_block(delimiter.to_string(), mem::take(blocks));
                    *self = Self::Line;
                    Ok(DelimitedLineKind::NestedEnd(block))
                }
                _ => Ok(DelimitedLineKind::Nest),
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum DelimitedLineKind {
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

#[derive(Debug)]
pub(crate) enum BlockVariant {
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
impl BlockVariant {
    fn to_block(&self, delimiter: String, blocks: Vec<Block>) -> Block {
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

#[derive(Debug)]
pub(crate) enum LeafVariant {
    Listing,
    Literal,
    Passthrough,
    Source,
    Stem,
    Verse,
    Comment,
}
impl LeafVariant {
    fn to_block(&self, delimiter: String, value: &str) -> Option<Block> {
        match self {
            Self::Listing => Some(Block::new_delimited_listing(delimiter, value)),
            Self::Literal => Some(Block::new_delimited_literal(delimiter, value)),
            Self::Passthrough => Some(Block::new_delimited_pass(delimiter, value)),
            Self::Source => todo!(),
            Self::Stem => Some(Block::new_delimited_stem(delimiter, value)),
            Self::Verse => Some(Block::new_delimited_verse(delimiter, value)),
            Self::Comment => None,
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
            assert_matches!(p.parse_line("", 0), Ok(SectionLineKind::Skip))
        }
    }

    mod delimited_parser {
        use super::*;

        #[test]
        fn line_prase_line_empty() {
            let mut p = DelimitedParser::Line;
            assert_matches!(p.parse_line(""), Ok(DelimitedLineKind::Skip))
        }
    }
}
