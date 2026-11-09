use asciidoc2rs::asg::block::{Author, Block, Document, LeafBlock, SectionBody, TitleAndAuthors};
use asciidoc2rs::asg::{Inline, InlineLiteral, Inlines, LiteralValue};
use asciidoc2rs::{Doctype, Parser};
use std::assert_matches;
use std::collections::HashMap;

fn parse(text: &'static str) -> Result<Document, String> {
    let parser = Parser::new_with_doctype(text, Doctype::Article);

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

#[test]
fn section_block() {
    let document = parse("== Level 1 Section Title\n\n=== Level 2 Section Title\n\n==== Level 3 Section Title\n\n===== Level 4 Section Title\n\n====== Level 5 Section Title\n\n== Another Level 1 Section Title").unwrap();

    assert_eq!(2, document.blocks.len());

    let Some(SectionBody::Section(level1_section)) = document.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Level 1 Section Title"),
        level1_section.title
    );
    assert_eq!(1, level1_section.blocks.len());
    let Some(SectionBody::Section(level2_section)) = level1_section.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Level 2 Section Title"),
        level2_section.title
    );
    assert_eq!(1, level2_section.blocks.len());
    let Some(SectionBody::Section(level3_section)) = level2_section.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Level 3 Section Title"),
        level3_section.title
    );
    assert_eq!(1, level3_section.blocks.len());
    let Some(SectionBody::Section(level4_section)) = level3_section.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Level 4 Section Title"),
        level4_section.title
    );
    assert_eq!(1, level4_section.blocks.len());
    let Some(SectionBody::Section(level5_section)) = level4_section.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Level 5 Section Title"),
        level5_section.title
    );
    assert_eq!(0, level5_section.blocks.len());

    let Some(SectionBody::Section(another_level1_section)) = document.blocks.last() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Another Level 1 Section Title"),
        another_level1_section.title
    );
    assert_eq!(0, another_level1_section.blocks.len());
}

#[test]
fn nested_section_block() {
    let document = parse("== First Section\n\nContent of first section\n\n=== Nested Section\n\nContent of nested section\n\n== Second Section\n\nContent of second section").unwrap();

    assert_eq!(2, document.blocks.len());

    let Some(SectionBody::Section(first_section)) = document.blocks.first() else {
        panic!("cannot call");
    };
    assert_eq!(Inlines::new_with_line("First Section"), first_section.title);
    assert_eq!(2, first_section.blocks.len());
    let Some(SectionBody::Block(Block::Leaf(LeafBlock::Paragraph(content_of_first_section)))) =
        first_section.blocks.first()
    else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Content of first section"),
        content_of_first_section.inlines
    );

    let Some(SectionBody::Section(nested_section)) = first_section.blocks.last() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Nested Section"),
        nested_section.title
    );
    assert_eq!(1, nested_section.blocks.len());
    let Some(SectionBody::Block(Block::Leaf(LeafBlock::Paragraph(content_of_nested_section)))) =
        nested_section.blocks.first()
    else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Content of nested section"),
        content_of_nested_section.inlines
    );

    let Some(SectionBody::Section(second_section)) = document.blocks.last() else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Second Section"),
        second_section.title
    );
    assert_eq!(1, second_section.blocks.len());
    let Some(SectionBody::Block(Block::Leaf(LeafBlock::Paragraph(content_of_second_section)))) =
        second_section.blocks.first()
    else {
        panic!("cannot call");
    };
    assert_eq!(
        Inlines::new_with_line("Content of second section"),
        content_of_second_section.inlines
    );
}

#[test]
#[should_panic]
fn illegal_level_skipped_section_block() {
    parse("== First Section\n\n==== Illegal Nested Section (violates rule #2)").unwrap();
}
