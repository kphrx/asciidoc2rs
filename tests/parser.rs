use asciidoc2rs::asg::block::{Author, Document, TitleAndAuthors};
use asciidoc2rs::asg::{Inline, InlineLiteral, Inlines, LiteralValue};
use asciidoc2rs::{Doctype, Parser};
use std::assert_matches;
use std::collections::HashMap;
use std::error::Error;

fn parse(text: &'static str) -> Result<Document, Box<dyn Error>> {
    let mut parser = Parser::new_with_doctype(text, Doctype::Article);

    let document = parser.parse_to_asg()?;

    Ok(document)
}

#[test]
fn document_header() {
    let document = parse("// this comment line is ignored\n= Document Title\nKismet R. Lee <kismet@asciidoctor.org>\n:description: The document's description.\n:sectanchors:\n:url-repo: https://my-git-repo.com\n\nThe document body starts here.").unwrap();

    assert_matches!(
        document.header.as_ref().map(|h| h.header.clone()),
        Some(Some(TitleAndAuthors {
            title: Some(Inlines(i)),
            authors: Some(a),
            ..
        })) if matches!(
                i.as_slice(),
                [Inline::Literal(InlineLiteral::Text(LiteralValue { value: title, .. }))]
                    if title == "Document Title")
            && matches!(
                a.as_slice(),
                [Author { fullname: Some(fullname), address: Some(address), .. }]
                    if fullname == "Kismet R. Lee"
                    && address == "kismet@asciidoctor.org")
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
    parse("= Document Title\n\n= Illegal Level 0 Section (violates rule #1)\n\n== First Section")
        .unwrap();
}

#[test]
fn revision_line() {
    let document =
        parse("= The Intrepid Chronicles\nKismet Lee\n2.9, October 31, 2021: Fall incarnation")
            .unwrap();

    assert_matches!(
        document.header.as_ref().map(|h| h.header.clone()),
        Some(Some(TitleAndAuthors {
            title: Some(Inlines(i)),
            authors: Some(a),
            ..
        })) if matches!(
                i.as_slice(),
                [Inline::Literal(InlineLiteral::Text(LiteralValue { value: title, .. }))]
                    if title == "The Intrepid Chronicles")
            && matches!(a.as_slice(), [Author { fullname: Some(fullname), .. }]
                if fullname == "Kismet Lee")
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

    assert_matches!(
        document.header.as_ref().map(|h| h.header.clone()),
        Some(Some(TitleAndAuthors {
            title: Some(Inlines(i)),
            authors: Some(a),
            ..
        })) if matches!(
                i.as_slice(),
                [Inline::Literal(InlineLiteral::Text(LiteralValue { value: title, .. }))]
                    if title == "The Intrepid Chronicles")
            && matches!(a.as_slice(), [Author { fullname: Some(fullname), .. }]
                if fullname == "Kismet Lee")
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

    assert_matches!(
        document.header.as_ref().map(|h| h.header.clone()),
        Some(Some(TitleAndAuthors {
            title: Some(Inlines(i)),
            authors: Some(a),
            ..
        })) if matches!(
                i.as_slice(),
                [Inline::Literal(InlineLiteral::Text(LiteralValue { value: title, .. }))]
                    if title == "The Intrepid Chronicles")
            && matches!(a.as_slice(), [Author { fullname: Some(fullname), .. }]
                if fullname == "Kismet Lee")
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
