use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize, value},
    sequence::separated_pair,
    IResult,
};
use utils::read_lines;

type Assignment = (u32, u32);
type AssignmentPair = (Assignment, Assignment);

fn decimal(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[test]
fn test_decimal() {
    assert_eq!(decimal("1234"), Ok(("", 1234)));
    assert_eq!(decimal("1234-4321"), Ok(("-4321", 1234)));
}

fn assignment(input: &str) -> IResult<&str, Assignment> {
    separated_pair(decimal, tag("-"), decimal)(input)
}

#[test]
fn test_assignment() {
    assert_eq!(assignment("1234-4321"), Ok(("", (1234, 4321))));
}

fn assignment_pair(input: &str) -> IResult<&str, AssignmentPair> {
    separated_pair(assignment, tag(","), assignment)(input)
}

#[test]
fn test_assignment_pair() {
    assert_eq!(
        assignment_pair("1234-4321,5678-8765"),
        Ok(("", ((1234, 4321), (5678, 8765))))
    );
}

fn main() {
    for line in read_lines("input.txt").unwrap() {
        let line = line.unwrap();
        let assignment_pair = assignment_pair(line.as_str()).map(|x| x.1);
        println!("{:#?}", assignment_pair.unwrap())
    }
}
