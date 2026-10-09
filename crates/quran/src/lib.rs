//! Print Quranic text for a reference query such as `2:255` or `2:12-16`.
//!
//! The text is the Tanzil Quran Text (<https://tanzil.net>), embedded verbatim.

mod ast;
mod data;
mod error;
mod parse;
mod resolve;

pub use ast::{Component, Item, Location, Number, Query, Reference, Set, Span};
pub use data::{DataError, Quran, Text};
pub use error::QueryError;
pub use parse::parse;
pub use resolve::{Line, resolve};

use std::io::{self, Write};

/// Writes `lines` as text: `S:A<TAB>text` per ayah, the Basmala unprefixed.
///
/// # Errors
///
/// Returns any error from `out`.
pub fn write_lines(out: &mut impl Write, lines: &[Line], quran: &Quran) -> io::Result<()> {
    for line in lines {
        match *line {
            Line::Basmala { surah } => {
                if let Some(basmala) = quran.basmala(surah) {
                    writeln!(out, "{basmala}")?;
                }
            }
            Line::Ayah { surah, ayah } => {
                writeln!(out, "{surah}:{ayah}\t{}", quran.ayah(surah, ayah))?;
            }
        }
    }
    Ok(())
}
