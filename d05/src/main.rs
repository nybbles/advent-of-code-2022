use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{anychar, line_ending, satisfy},
        is_alphanumeric,
    },
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
struct Crate {
    label: char,
}

type MaybeCrate = Option<Crate>;
type CrateStack = Vec<Crate>;

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    delimited(tag("["), anychar, tag("]"))(input).map(|(r, label)| (r, Crate { label }))
}

#[test]
fn test_parse_crate() {
    assert_eq!(parse_crate("[x]"), Ok(("", Crate { label: 'x' })));
}

fn maybe_crate(input: &str) -> IResult<&str, MaybeCrate> {
    alt((
        |i| parse_crate(i).map(|(x, y)| (x, Some(y))),
        |i| tag("   ")(i).map(|(x, _y)| (x, None)),
    ))(input)
}

#[test]
fn test_maybe_crate() {
    assert_eq!(maybe_crate("[y]"), Ok(("", Some(Crate { label: 'y' }))));
    assert_eq!(maybe_crate("   "), Ok(("", None)));
}

fn crate_stack_row(input: &str) -> IResult<&str, Vec<MaybeCrate>> {
    separated_list1(tag(" "), maybe_crate)(input)
}

#[test]
fn test_crate_stack_row() {
    assert_eq!(
        crate_stack_row("[a] [c] [b]"),
        Ok((
            "",
            vec![
                Some(Crate { label: 'a' }),
                Some(Crate { label: 'c' }),
                Some(Crate { label: 'b' }),
            ]
        ))
    );
    assert_eq!(
        crate_stack_row("[a]     [b]"),
        Ok((
            "",
            vec![Some(Crate { label: 'a' }), None, Some(Crate { label: 'b' }),]
        ))
    );
}

fn crate_stack_labels(input: &str) -> IResult<&str, Vec<char>> {
    separated_list1(tag(" "), delimited(tag(" "), anychar, tag(" ")))(input)
}

#[test]
fn test_crate_stack_labels() {
    assert_eq!(crate_stack_labels(" 1   2 "), Ok(("", vec!['1', '2'])));
}

fn crate_stacks_and_labels(input: &str) -> IResult<&str, Vec<Vec<MaybeCrate>>> {
    let result = tuple((
        separated_list1(line_ending, crate_stack_row),
        line_ending,
        crate_stack_labels,
    ))(input);

    result.map(|(leftover, (crate_stack_rows, _, labels))| {
        assert_eq!(
            crate_stack_rows
                .iter()
                .map(|x| x.len())
                .collect::<HashSet<usize>>()
                .len(),
            1
        );
        (leftover, crate_stack_rows)
    })
}

#[test]
fn test_crate_stacks_and_labels() {
    let input = "[a]    
[c] [d]
 1   2 ";

    assert_eq!(
        crate_stacks_and_labels(input),
        Ok((
            "",
            vec![
                vec![Some(Crate { label: 'a' }), None],
                vec![Some(Crate { label: 'c' }), Some(Crate { label: 'd' })],
            ]
        ))
    );
}

fn main() {
    println!("Hello, world!");
}
