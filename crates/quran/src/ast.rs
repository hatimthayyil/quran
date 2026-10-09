use std::ops::Range;

/// Byte offsets of a node in the query.
pub type Span = Range<usize>;

/// A parsed query: references combined by union, in query order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub references: Vec<Reference>,
}

/// A location, or a range from `start` to `end` as written (before shorthand expansion).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub start: Location,
    pub end: Option<Location>,
    pub span: Span,
}

/// One to three `:`-separated components: surah, ayah, word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub components: Vec<Component>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Component {
    Number(Number),
    Wildcard(Span),
    Set(Set),
}

impl Component {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Number(number) => number.span.clone(),
            Self::Wildcard(span) => span.clone(),
            Self::Set(set) => set.span.clone(),
        }
    }
}

/// A brace set such as `{1,5,10-12}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Set {
    pub items: Vec<Item>,
    pub span: Span,
}

/// A set item: a number or an inclusive range of numbers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub start: Number,
    pub end: Option<Number>,
}

/// A number as written; values too large for `u32` are `u32::MAX`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    pub value: u32,
    pub span: Span,
}
