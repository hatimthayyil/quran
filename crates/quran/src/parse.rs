use winnow::ascii::{digit1, space0};
use winnow::combinator::{alt, cut_err, fail, opt, preceded, repeat, separated};
use winnow::error::{ContextError, ParseError, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::stream::{LocatingSlice, Stream};

use crate::ast::{Component, Item, Location, Number, Query, Reference, Set};
use crate::error::QueryError;

type Input<'a> = LocatingSlice<&'a str>;

/// Parses a query into its syntax tree, without checking it against the text.
///
/// # Errors
///
/// Returns [`QueryError::DotDot`] for `..` and [`QueryError::Syntax`] for any
/// other malformed query.
pub fn parse(query: &str) -> Result<Query, QueryError> {
    if let Some(offset) = query.find("..") {
        return Err(QueryError::DotDot {
            span: (offset, 2).into(),
        });
    }
    references
        .parse(LocatingSlice::new(query))
        .map(|references| Query { references })
        .map_err(|error| syntax_error(query, &error))
}

fn syntax_error(query: &str, error: &ParseError<Input<'_>, ContextError>) -> QueryError {
    let offset = error.offset();
    let length = query[offset..].chars().next().map_or(0, char::len_utf8);
    let expected: Vec<String> = error
        .inner()
        .context()
        .filter_map(|context| match context {
            StrContext::Expected(value) => Some(value.to_string()),
            _ => None,
        })
        .collect();
    QueryError::Syntax {
        span: (offset, length).into(),
        expected: format!("expected {}", expected.join(" or ")),
    }
}

fn expected(description: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(description))
}

fn references(input: &mut Input<'_>) -> ModalResult<Vec<Reference>> {
    space0.parse_next(input)?;
    let mut references = vec![cut_err(reference).parse_next(input)?];
    loop {
        let spaced = !space0.parse_next(input)?.is_empty();
        if input.eof_offset() == 0 {
            return Ok(references);
        }
        if opt(',').parse_next(input)?.is_some() {
            space0.parse_next(input)?;
        } else if !spaced {
            return cut_err(fail)
                .context(expected("`,`, a space, `:`, `-` or the end of the query"))
                .parse_next(input);
        }
        references.push(cut_err(reference).parse_next(input)?);
    }
}

fn reference(input: &mut Input<'_>) -> ModalResult<Reference> {
    (location, opt(preceded('-', cut_err(location))))
        .with_span()
        .map(|((start, end), span)| Reference { start, end, span })
        .parse_next(input)
}

fn location(input: &mut Input<'_>) -> ModalResult<Location> {
    (component, repeat(0..=2, preceded(':', cut_err(component))))
        .with_span()
        .map(|((first, rest), span): ((Component, Vec<Component>), _)| {
            let mut components = vec![first];
            components.extend(rest);
            Location { components, span }
        })
        .parse_next(input)
}

fn component(input: &mut Input<'_>) -> ModalResult<Component> {
    alt((
        number.map(Component::Number),
        '*'.span().map(Component::Wildcard),
        set.map(Component::Set),
    ))
    .context(expected("a number, `*` or `{`"))
    .parse_next(input)
}

fn set(input: &mut Input<'_>) -> ModalResult<Set> {
    preceded(
        '{',
        cut_err((
            separated(1.., item, ','),
            '}'.context(expected("`,` or `}`")),
        )),
    )
    .with_span()
    .map(|((items, _), span)| Set { items, span })
    .parse_next(input)
}

fn item(input: &mut Input<'_>) -> ModalResult<Item> {
    (
        cut_err(number.context(expected("a number"))),
        opt(preceded('-', cut_err(number.context(expected("a number"))))),
    )
        .map(|(start, end)| Item { start, end })
        .parse_next(input)
}

fn number(input: &mut Input<'_>) -> ModalResult<Number> {
    let start = input.checkpoint();
    let (digits, span) = digit1.with_span().parse_next(input)?;
    if digits.len() > 1 && digits.starts_with('0') {
        input.reset(&start);
        return cut_err(fail)
            .context(expected("a number without leading zeros"))
            .parse_next(input);
    }
    Ok(Number {
        value: digits.parse().unwrap_or(u32::MAX),
        span,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delimited_whitespace_is_trimmed() {
        let query = parse(" 2:255 ").expect("parses");
        assert_eq!(query.references.len(), 1);
        assert_eq!(query.references[0].span, 1..6);
    }

    #[test]
    fn shorthand_end_keeps_its_written_components() {
        let query = parse("2:12-16").expect("parses");
        let end = query.references[0].end.as_ref().expect("range");
        assert_eq!(end.components.len(), 1);
    }
}
