use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use crate::asg::{Headline, Inline, Location, NodeType, NonSectionBlockBody};

#[derive(Serialize, Debug)]
pub(crate) enum Block {
    BlockParent(BlockParent),
    BlockLeaf(BlockLeaf),
    BlockMacro(BlockMacro),
    BlockBreak(BlockBreak),
    AnyList(AnyList),
}
impl Block {
    fn new_admonition(variant: AdmonitionVariant) -> Self {
        Self::BlockParent(BlockParent::new_admonition(variant))
    }

    fn new_example() -> Self {
        Self::BlockParent(BlockParent::new_example())
    }

    fn new_sidebar() -> Self {
        Self::BlockParent(BlockParent::new_sidebar())
    }

    fn new_open() -> Self {
        Self::BlockParent(BlockParent::new_open())
    }

    fn new_quote() -> Self {
        Self::BlockParent(BlockParent::new_quote())
    }

    fn new_listing() -> Self {
        Self::BlockLeaf(BlockLeaf::new_listing())
    }

    fn new_literal() -> Self {
        Self::BlockLeaf(BlockLeaf::new_literal())
    }

    fn new_paragraph() -> Self {
        Self::BlockLeaf(BlockLeaf::new_paragraph())
    }

    fn new_pass() -> Self {
        Self::BlockLeaf(BlockLeaf::new_pass())
    }

    fn new_stem() -> Self {
        Self::BlockLeaf(BlockLeaf::new_stem())
    }

    fn new_verse() -> Self {
        Self::BlockLeaf(BlockLeaf::new_verse())
    }

    fn new_audio() -> Self {
        Self::BlockMacro(BlockMacro::new_audio())
    }

    fn new_video() -> Self {
        Self::BlockMacro(BlockMacro::new_video())
    }

    fn new_image() -> Self {
        Self::BlockMacro(BlockMacro::new_image())
    }

    fn new_toc() -> Self {
        Self::BlockMacro(BlockMacro::new_toc())
    }

    fn new_page_break() -> Self {
        Self::BlockBreak(BlockBreak::new_page())
    }

    fn new_thematic_break() -> Self {
        Self::BlockBreak(BlockBreak::new_thematic())
    }

    fn new_callout_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_callout_list(marker))
    }

    fn new_ordered_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_ordered_list(marker))
    }

    fn new_unordered_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_unordered_list(marker))
    }

    fn new_description_list(marker: String) -> Self {
        Self::AnyList(AnyList::new_description_list(marker))
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum BlockParent {
    Admonition {
        #[serde(rename = "type")]
        node_type: NodeType,
        delimiter: Option<String>,
        title: Option<Headline>,
        blocks: Vec<NonSectionBlockBody>,
        location: Option<Location>,
        variant: AdmonitionVariant,
    },
    Example(BlockParentBody),
    Sidebar(BlockParentBody),
    Open(BlockParentBody),
    Quote(BlockParentBody),
}
#[derive(Serialize, Debug)]
pub(crate) enum AdmonitionVariant {
    Caution,
    Important,
    Note,
    Tip,
    Warning,
}
impl BlockParent {
    fn new_admonition(variant: AdmonitionVariant) -> Self {
        Self::Admonition {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
            variant,
        }
    }

    fn new_example() -> Self {
        Self::Example(BlockParentBody::new())
    }

    fn new_sidebar() -> Self {
        Self::Sidebar(BlockParentBody::new())
    }

    fn new_open() -> Self {
        Self::Open(BlockParentBody::new())
    }

    fn new_quote() -> Self {
        Self::Quote(BlockParentBody::new())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub(crate) struct BlockParentBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: Option<String>,
    title: Option<Headline>,
    blocks: Vec<NonSectionBlockBody>,
    location: Option<Location>,
}
impl BlockParentBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            blocks: Vec::with_capacity(0),
            location: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum BlockLeaf {
    Listing(BlockLeafBody),
    Literal(BlockLeafBody),
    Paragraph(BlockLeafBody),
    Pass(BlockLeafBody),
    Stem(BlockLeafBody),
    Verse(BlockLeafBody),
}

impl BlockLeaf {
    fn new_listing() -> Self {
        Self::Listing(BlockLeafBody::new())
    }

    fn new_literal() -> Self {
        Self::Literal(BlockLeafBody::new())
    }

    fn new_paragraph() -> Self {
        Self::Paragraph(BlockLeafBody::new())
    }

    fn new_pass() -> Self {
        Self::Pass(BlockLeafBody::new())
    }

    fn new_stem() -> Self {
        Self::Stem(BlockLeafBody::new())
    }

    fn new_verse() -> Self {
        Self::Verse(BlockLeafBody::new())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub(crate) struct BlockLeafBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    delimiter: Option<String>,
    title: Option<Headline>,
    inlines: Vec<Inline>,
    location: Option<Location>,
}

impl BlockLeafBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            delimiter: None,
            title: None,
            inlines: Vec::with_capacity(0),
            location: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum BlockMacro {
    Audio(BlockMacroBody),
    Video(BlockMacroBody),
    Image(BlockMacroBody),
    Toc(BlockMacroBody),
}
impl BlockMacro {
    fn new_audio() -> Self {
        Self::Audio(BlockMacroBody::new())
    }

    fn new_video() -> Self {
        Self::Video(BlockMacroBody::new())
    }

    fn new_image() -> Self {
        Self::Image(BlockMacroBody::new())
    }

    fn new_toc() -> Self {
        Self::Toc(BlockMacroBody::new())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub(crate) struct BlockMacroBody {
    #[serde(rename = "type")]
    node_type: NodeType,
    target: Option<String>,
    title: Option<Headline>,
    location: Option<Location>,
}
impl BlockMacroBody {
    fn new() -> Self {
        Self {
            node_type: NodeType::Block,
            target: None,
            title: None,
            location: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum BlockBreak {
    Break {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: BlockBreakVariant,
        location: Option<Location>,
    },
}
impl BlockBreak {
    fn new_page() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Page,
            location: None,
        }
    }

    fn new_thematic() -> Self {
        Self::Break {
            node_type: NodeType::Block,
            variant: BlockBreakVariant::Thematic,
            location: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum BlockBreakVariant {
    Page,
    Thematic,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum AnyList {
    List {
        #[serde(rename = "type")]
        node_type: NodeType,
        variant: ListVariant,
        marker: String,
        title: Option<Headline>,
        location: Option<Location>,
        items: Vec<ListItem>,
    },
    Dlist {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        title: Option<Headline>,
        location: Option<Location>,
        items: Vec<DlistItem>,
    },
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ListVariant {
    Callout,
    Ordered,
    Unordered,
}
impl AnyList {
    fn new_callout_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Callout,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_ordered_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Ordered,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_unordered_list(marker: String) -> Self {
        Self::List {
            node_type: NodeType::Block,
            variant: ListVariant::Unordered,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }

    fn new_description_list(marker: String) -> Self {
        Self::Dlist {
            node_type: NodeType::Block,
            marker,
            title: None,
            location: None,
            items: Vec::with_capacity(0),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum ListItem {
    ListItem {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        principal: Headline,
        blocks: Option<Vec<NonSectionBlockBody>>,
        location: Option<Location>,
    },
}
impl ListItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self::ListItem {
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: None,
            location: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(tag = "name", rename_all = "camelCase")]
pub(crate) enum DlistItem {
    DlistItem {
        #[serde(rename = "type")]
        node_type: NodeType,
        marker: String,
        principal: Headline,
        blocks: Option<Vec<NonSectionBlockBody>>,
        location: Option<Location>,
        terms: Vec<Headline>,
    },
}
impl DlistItem {
    fn new(marker: String, principal: Headline) -> Self {
        Self::DlistItem {
            node_type: NodeType::Block,
            marker,
            principal,
            blocks: None,
            location: None,
            terms: Vec::with_capacity(0),
        }
    }
}
