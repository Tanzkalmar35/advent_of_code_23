use std::collections::HashSet;
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take_till1};
use nom::character::complete::digit1;
use nom::combinator::iterator;
use nom::IResult;
use nom_locate::LocatedSpan;
use crate::custom_error::AocError;
use nom::Parser;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let objects = parse_grid(Span::new(input)).unwrap().1;

    let symbol_map = objects
        .iter()
        .filter_map(|value| match value {
            Value::Empty => None,
            Value::Symbol(sym) => Some(sym.extra),
            Value::Number(_) => None,
        })
        .collect::<HashSet<IVec2>>();

    let result = objects
        .iter()
        .filter_map(|value| {
            let Value::Number(num) = value else {
                return None;
            };
            let surrounding_positions = [
                // east border
                IVec2::new(num.fragment().len() as i32, 0),
                // west border
                IVec2::new(-1, 0),
            ]
                .into_iter()
                .chain(
                    // north border
                    (-1..=num.fragment().len() as i32).map(
                        |x_offset| IVec2::new(x_offset, 1),
                    ),
                )
                .chain(
                    // south border
                    (-1..=num.fragment().len() as i32).map(
                        |x_offset| IVec2::new(x_offset, -1),
                    ),
                )
                .map(|pos| pos + num.extra)
                .collect::<Vec<IVec2>>();

            surrounding_positions
                .iter()
                .any(|pos| symbol_map.contains(pos))
                .then_some(
                    num.fragment()
                        .parse::<u32>()
                        .expect("should be a valid number"),
                )
        })
        .sum::<u32>();

    Ok(result.to_string())
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1; // -1 because these are 1 indexed
    let y = span.location_line() as i32 - 1; // -1 because these are 1 indexed
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1
                .map(|span| with_xy(span))
                .map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|char: char| {
                char.is_ascii_digit() || char != '.' && char != '\n'
            })
                .map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
