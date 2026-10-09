use std::ops::RangeInclusive;

use crate::ast::{Component, Item, Location, Number, Query, Reference, Span};
use crate::data::Quran;
use crate::error::QueryError;

/// One output line: a surah's Basmala or an ayah.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line {
    Basmala { surah: u16 },
    Ayah { surah: u16, ayah: u16 },
}

/// Resolves `query` against the text into output lines, in query order.
///
/// # Errors
///
/// Returns a [`QueryError`] for a location that does not exist, a reversed
/// range, mixed levels, a pattern at the level a range spans, or a word.
pub fn resolve(query: &Query, quran: &Quran) -> Result<Vec<Line>, QueryError> {
    let mut lines = Vec::new();
    let resolver = Resolver { quran };
    for reference in &query.references {
        for segment in resolver.reference(reference)? {
            segment.emit(quran, &mut lines);
        }
    }
    Ok(lines)
}

/// Consecutive ayahs of one surah.
struct Segment {
    surah: u16,
    ayahs: RangeInclusive<u16>,
}

impl Segment {
    fn emit(&self, quran: &Quran, lines: &mut Vec<Line>) {
        let surah = self.surah;
        let whole = *self.ayahs.start() == 1 && *self.ayahs.end() == quran.ayah_count(surah);
        if whole && quran.basmala(surah).is_some() {
            lines.push(Line::Basmala { surah });
        }
        lines.extend(self.ayahs.clone().map(|ayah| Line::Ayah { surah, ayah }));
    }
}

/// The surahs an ayah component is evaluated in. Under a pattern, a position
/// missing from one bound surah is skipped if it exists in another.
struct Scope {
    surahs: Vec<u16>,
    pattern: bool,
    longest: u16,
}

#[derive(Clone, Copy)]
struct Resolver<'q> {
    quran: &'q Quran,
}

impl Resolver<'_> {
    fn reference(self, reference: &Reference) -> Result<Vec<Segment>, QueryError> {
        let start = &reference.start;
        reject_words(start)?;
        let Some(end) = &reference.end else {
            return self.location(start);
        };
        reject_words(end)?;
        let (start_len, end_len) = (start.components.len(), end.components.len());
        if end_len > start_len {
            return Err(QueryError::MixedLevels {
                span: reference.span.clone().into(),
            });
        }
        let (bound, spanned) = start.components.split_at(start_len - end_len);
        let from = numbers(spanned)?;
        let to = numbers(&end.components)?;
        let span = reference.span.clone();
        match (bound, from.as_slice(), to.as_slice()) {
            ([], [s1], [s2]) => self.surah_range(s1, s2, span),
            ([], [s1, a1], [s2, a2]) => self.cross_range(s1, a1, s2, a2, span),
            ([surah], [a1], [a2]) => {
                let scope = self.scope(surah)?;
                let mut segments = Vec::new();
                for &s in &scope.surahs {
                    segments.extend(self.ayah_range(&scope, s, a1, a2, span.clone())?);
                }
                Ok(segments)
            }
            _ => unreachable!("locations have one or two components"),
        }
    }

    fn location(self, location: &Location) -> Result<Vec<Segment>, QueryError> {
        match location.components.as_slice() {
            [surah] => Ok(self
                .surahs(surah)?
                .into_iter()
                .map(|s| self.whole(s))
                .collect()),
            [surah, ayah] => {
                let scope = self.scope(surah)?;
                let mut segments = Vec::new();
                for &s in &scope.surahs {
                    segments.extend(self.ayahs(&scope, s, ayah)?);
                }
                Ok(segments)
            }
            _ => unreachable!("locations have one or two components"),
        }
    }

    fn whole(self, surah: u16) -> Segment {
        Segment {
            surah,
            ayahs: 1..=self.quran.ayah_count(surah),
        }
    }

    fn scope(self, component: &Component) -> Result<Scope, QueryError> {
        let surahs = self.surahs(component)?;
        let longest = surahs
            .iter()
            .map(|&s| self.quran.ayah_count(s))
            .max()
            .unwrap_or(0);
        Ok(Scope {
            surahs,
            pattern: !matches!(component, Component::Number(_)),
            longest,
        })
    }

    fn surahs(self, component: &Component) -> Result<Vec<u16>, QueryError> {
        match component {
            Component::Number(number) => Ok(vec![surah(number)?]),
            Component::Wildcard(_) => Ok((1..=Quran::SURAHS).collect()),
            Component::Set(set) => {
                let mut surahs = Vec::new();
                for item in &set.items {
                    let end = item.end.as_ref().unwrap_or(&item.start);
                    let (from, to) = (surah(&item.start)?, surah(end)?);
                    if from > to {
                        return Err(reversed(item_span(item)));
                    }
                    surahs.extend(from..=to);
                }
                Ok(surahs)
            }
        }
    }

    fn ayahs(
        self,
        scope: &Scope,
        surah: u16,
        component: &Component,
    ) -> Result<Vec<Segment>, QueryError> {
        match component {
            Component::Number(number) => Ok(self
                .ayah_range(scope, surah, number, number, number.span.clone())?
                .into_iter()
                .collect()),
            Component::Wildcard(_) => Ok(vec![self.whole(surah)]),
            Component::Set(set) => {
                let mut segments = Vec::new();
                for item in &set.items {
                    let end = item.end.as_ref().unwrap_or(&item.start);
                    segments.extend(self.ayah_range(
                        scope,
                        surah,
                        &item.start,
                        end,
                        item_span(item),
                    )?);
                }
                Ok(segments)
            }
        }
    }

    fn ayah_range(
        self,
        scope: &Scope,
        surah: u16,
        from: &Number,
        to: &Number,
        span: Span,
    ) -> Result<Option<Segment>, QueryError> {
        let count = self.quran.ayah_count(surah);
        let limit = if scope.pattern { scope.longest } else { count };
        let help = || {
            if scope.pattern {
                format!("the bound surahs have at most {limit} ayahs")
            } else {
                format!("surah {surah} has {count} ayahs")
            }
        };
        let (a1, a2) = (ayah(from, limit, help)?, ayah(to, limit, help)?);
        if a1 > a2 {
            return Err(reversed(span));
        }
        Ok((a1 <= count).then(|| Segment {
            surah,
            ayahs: a1..=a2.min(count),
        }))
    }

    fn surah_range(
        self,
        from: &Number,
        to: &Number,
        span: Span,
    ) -> Result<Vec<Segment>, QueryError> {
        let (s1, s2) = (surah(from)?, surah(to)?);
        if s1 > s2 {
            return Err(reversed(span));
        }
        Ok((s1..=s2).map(|s| self.whole(s)).collect())
    }

    fn cross_range(
        self,
        s1: &Number,
        a1: &Number,
        s2: &Number,
        a2: &Number,
        span: Span,
    ) -> Result<Vec<Segment>, QueryError> {
        let (s1, s2) = (surah(s1)?, surah(s2)?);
        let in_surah = |number: &Number, s: u16| {
            let count = self.quran.ayah_count(s);
            ayah(number, count, || format!("surah {s} has {count} ayahs"))
        };
        let (a1, a2) = (in_surah(a1, s1)?, in_surah(a2, s2)?);
        if (s1, a1) > (s2, a2) {
            return Err(reversed(span));
        }
        if s1 == s2 {
            return Ok(vec![Segment {
                surah: s1,
                ayahs: a1..=a2,
            }]);
        }
        let mut segments = vec![Segment {
            surah: s1,
            ayahs: a1..=self.quran.ayah_count(s1),
        }];
        segments.extend((s1 + 1..s2).map(|s| self.whole(s)));
        segments.push(Segment {
            surah: s2,
            ayahs: 1..=a2,
        });
        Ok(segments)
    }
}

fn reject_words(location: &Location) -> Result<(), QueryError> {
    match location.components.get(2) {
        Some(word) => Err(QueryError::WordUnsupported {
            span: word.span().into(),
        }),
        None => Ok(()),
    }
}

fn numbers(components: &[Component]) -> Result<Vec<&Number>, QueryError> {
    components
        .iter()
        .map(|component| match component {
            Component::Number(number) => Ok(number),
            other => Err(QueryError::PatternInRange {
                span: other.span().into(),
            }),
        })
        .collect()
}

fn surah(number: &Number) -> Result<u16, QueryError> {
    u16::try_from(number.value)
        .ok()
        .filter(|value| (1..=Quran::SURAHS).contains(value))
        .ok_or_else(|| QueryError::SurahOutOfBounds {
            value: number.value,
            span: number.span.clone().into(),
        })
}

fn ayah(number: &Number, limit: u16, help: impl Fn() -> String) -> Result<u16, QueryError> {
    u16::try_from(number.value)
        .ok()
        .filter(|value| (1..=limit).contains(value))
        .ok_or_else(|| QueryError::AyahOutOfBounds {
            value: number.value,
            span: number.span.clone().into(),
            help: help(),
        })
}

fn item_span(item: &Item) -> Span {
    let end = item.end.as_ref().unwrap_or(&item.start);
    item.start.span.start..end.span.end
}

fn reversed(span: Span) -> QueryError {
    QueryError::Reversed { span: span.into() }
}
