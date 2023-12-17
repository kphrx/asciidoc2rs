#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    NewLine,
    Comment,
    CommentDelimiter,        // `////`
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
    Heading(usize),
    Text(String),
    StrongOpen,
    StrongClose,
    EmphasisOpen,
    EmphasisClose,
    CodeOpen,
    CodeClose,
    MarkOpen,
    MarkClose,
    SubscriptOpen,
    SubscriptClose,
    SuperscriptOpen,
    SuperscriptClose,
}
