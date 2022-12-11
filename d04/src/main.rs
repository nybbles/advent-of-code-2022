use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
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

#[allow(dead_code)]
fn is_fully_contained(assignment_pair: &AssignmentPair) -> bool {
    let (assignment1, assignment2) = assignment_pair;
    (assignment1.0 <= assignment2.0 && assignment1.1 >= assignment2.1)
        || (assignment2.0 <= assignment1.0 && assignment2.1 >= assignment1.1)
}

fn has_overlap(assignment_pair: &AssignmentPair) -> bool {
    let (assignment1, assignment2) = assignment_pair;

    let first_assignment;
    let second_assignment;
    if assignment1.0 <= assignment2.0 {
        first_assignment = assignment1;
        second_assignment = assignment2;
    } else {
        first_assignment = assignment2;
        second_assignment = assignment1;
    }

    first_assignment.1 >= second_assignment.0
}

fn main() {
    let assignment_pairs = read_lines("input.txt").unwrap().map(|line| {
        assignment_pair(line.unwrap().as_str())
            .map(|x| x.1)
            .unwrap()
    });

    let overlap_count = assignment_pairs.filter(|x| has_overlap(x)).count();
    println!("{}", overlap_count);
}
