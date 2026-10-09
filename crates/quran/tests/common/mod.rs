#![allow(dead_code)]

use std::sync::LazyLock;

use quran::{Line, QueryError, Quran, Text};

pub static QURAN: LazyLock<Quran> =
    LazyLock::new(|| Quran::load(Text::Uthmani).expect("embedded text loads"));

pub fn try_lines(query: &str) -> Result<Vec<String>, QueryError> {
    let query = quran::parse(query)?;
    let lines = quran::resolve(&query, &QURAN)?;
    Ok(lines
        .into_iter()
        .map(|line| match line {
            Line::Basmala { surah } => format!("b{surah}"),
            Line::Ayah { surah, ayah } => format!("{surah}:{ayah}"),
        })
        .collect())
}

pub fn lines(query: &str) -> Vec<String> {
    try_lines(query).unwrap_or_else(|error| panic!("{query}: {error:?}"))
}

pub fn error(query: &str) -> QueryError {
    match try_lines(query) {
        Ok(lines) => panic!("{query}: expected an error, got {} lines", lines.len()),
        Err(error) => error,
    }
}

pub fn ayahs(surah: u16, range: std::ops::RangeInclusive<u16>) -> Vec<String> {
    range.map(|ayah| format!("{surah}:{ayah}")).collect()
}
