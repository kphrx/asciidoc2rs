#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    NewLine,
    Comment,
    CommentDelimiter(usize), // `////`
    ExampleDelimiter(usize), // `====`
    SidebarDelimiter(usize), // `****`
    QuoteDelimiter(usize),   // `____`
    ListingDelimiter(usize), // `----`
    LiteralDelimiter(usize), // `....`
    PassDelimiter(usize),    // `++++`
    OpenDelimiter(usize),    // `~~~~`
    LegacyOpenDelimiter,     // `--` (legacy)
    TableDelimiter,          // `|===`
    NestedTableDelimiter,    // `!===`
    CsvTableDelimiter,       // `,===`
    DsvTableDelimiter,       // `:===`
    AttributeEntry(String, bool),
    Heading(usize),
    UnorderedList(usize),
    Text(String),
    Strong(bool, bool),
    Emphasis(bool, bool),
    Code(bool, bool),
    Mark(bool, bool),
    Subscript,
    Superscript,
}
