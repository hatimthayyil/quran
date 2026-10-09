use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// An invalid query. Attach the query as source code to render the span.
#[derive(Clone, Debug, Diagnostic, Error, PartialEq, Eq)]
pub enum QueryError {
    #[error("invalid query: {expected}")]
    #[diagnostic(code(quran::syntax))]
    Syntax {
        #[label("here")]
        span: SourceSpan,
        expected: String,
    },

    #[error("`..` is not a range operator")]
    #[diagnostic(
        code(quran::syntax::dotdot),
        help("write ranges with `-`, for example `2:1-5` or `2:{{1-3}}`")
    )]
    DotDot {
        #[label("use `-` here")]
        span: SourceSpan,
    },

    #[error("word-level references are not supported yet")]
    #[diagnostic(
        code(quran::word::unsupported),
        help("use a surah or an ayah, for example `2:255`")
    )]
    WordUnsupported {
        #[label("word")]
        span: SourceSpan,
    },

    #[error("surah {value} does not exist")]
    #[diagnostic(code(quran::bounds::surah), help("surahs are numbered 1 to 114"))]
    SurahOutOfBounds {
        value: u32,
        #[label("out of bounds")]
        span: SourceSpan,
    },

    #[error("ayah {value} does not exist")]
    #[diagnostic(code(quran::bounds::ayah))]
    AyahOutOfBounds {
        value: u32,
        #[label("out of bounds")]
        span: SourceSpan,
        #[help]
        help: String,
    },

    #[error("range ends before it starts")]
    #[diagnostic(
        code(quran::range::reversed),
        help("write the start first, for example `2:12-16`")
    )]
    Reversed {
        #[label("reversed")]
        span: SourceSpan,
    },

    #[error("range end is more specific than its start")]
    #[diagnostic(
        code("quran::range::mixed-levels"),
        help("start and end must name the same level, for example `2:5-3:5` or `2-3`")
    )]
    MixedLevels {
        #[label("mixed levels")]
        span: SourceSpan,
    },

    #[error("set or wildcard at the level the range spans")]
    #[diagnostic(
        code(quran::range::pattern),
        help("use sets and wildcards only above the range, for example `*:2-4` or `{{2,3}}:1-5`")
    )]
    PatternInRange {
        #[label("not a number")]
        span: SourceSpan,
    },
}
