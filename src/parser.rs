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
                    && v == ""
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
                    && v == ""
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
                    && v == ""
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
}
