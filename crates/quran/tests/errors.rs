mod common;

use common::error;
use miette::{Diagnostic, SourceSpan};
use quran::QueryError;

fn code(query: &str) -> String {
    error(query)
        .code()
        .map(|code| code.to_string())
        .unwrap_or_default()
}

fn span(query: &str) -> SourceSpan {
    let error = error(query);
    let mut labels = error.labels().expect("error has a label");
    labels.next().expect("error has a label").inner().to_owned()
}

#[test]
fn ayah_out_of_bounds() {
    assert_eq!(code("2:300"), "quran::bounds::ayah");
    assert_eq!(span("2:300"), (2, 3).into());
    let help = error("2:300").help().map(|h| h.to_string());
    assert_eq!(help.as_deref(), Some("surah 2 has 286 ayahs"));
}

#[test]
fn surah_out_of_bounds() {
    for query in ["115", "0", "{1,115}", "2:1-115:1", "99999999999"] {
        assert_eq!(code(query), "quran::bounds::surah", "{query}");
    }
    let help = error("115").help().map(|h| h.to_string());
    assert_eq!(help.as_deref(), Some("surahs are numbered 1 to 114"));
}

#[test]
fn ayah_zero_is_out_of_bounds() {
    assert_eq!(code("2:0"), "quran::bounds::ayah");
}

#[test]
fn pattern_position_missing_everywhere_is_an_error() {
    assert_eq!(code("*:287"), "quran::bounds::ayah");
    assert_eq!(code("2:{1,300}"), "quran::bounds::ayah");
    assert_eq!(code("{103,108}:4"), "quran::bounds::ayah");
}

#[test]
fn reversed_ranges() {
    for query in ["2:16-12", "3-1", "3:1-2:5", "2:{12-10}"] {
        assert_eq!(code(query), "quran::range::reversed", "{query}");
    }
}

#[test]
fn mixed_levels() {
    assert_eq!(code("2-3:5"), "quran::range::mixed-levels");
}

#[test]
fn pattern_at_the_level_a_range_spans() {
    for query in ["2:{1,5}-10", "2:*-10", "*-3", "2:1-*", "*:1-*:3"] {
        assert_eq!(code(query), "quran::range::pattern", "{query}");
    }
}

#[test]
fn malformed_syntax() {
    for (query, offset) in [
        ("2:", 2),
        (":5", 0),
        ("2::5", 2),
        ("2:5-", 4),
        ("", 0),
        ("2:3,", 4),
        ("2x", 1),
        ("2:{}", 3),
        ("2:{1,5", 6),
        ("02", 0),
        ("2:5:3:1", 5),
    ] {
        assert_eq!(code(query), "quran::syntax", "{query}");
        assert_eq!(span(query).offset(), offset, "{query}");
    }
}

#[test]
fn dot_dot_is_rejected_with_a_hint() {
    assert_eq!(code("2:{1..3}"), "quran::syntax::dotdot");
    assert_eq!(span("2:1..5"), (3, 2).into());
    let help = error("2:1..5").help().map(|h| h.to_string());
    assert!(help.is_some_and(|h| h.contains('-')));
}

#[test]
fn word_level_is_not_supported_yet() {
    assert!(matches!(
        error("2:255:3"),
        QueryError::WordUnsupported { .. }
    ));
    assert_eq!(code("2:255:3"), "quran::word::unsupported");
    assert_eq!(span("2:255:3"), (6, 1).into());
}
