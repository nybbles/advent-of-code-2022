use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

const NHANDSHAPES: i32 = 3;

fn score_for_handshape(hs: &HandShape) -> i32 {
    match hs {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    }
}

fn score_for_round(opponent_hs: &HandShape, our_hs: &HandShape) -> i32 {
    match (score_for_handshape(opponent_hs) - score_for_handshape(our_hs) + NHANDSHAPES)
        % NHANDSHAPES
    {
        1 => 0, // loss
        2 => 6, // win
        0 => 3, // draw
        other => panic!(
            "Logic error: {}, opponent_hs: {:#?}, our_hs: {:#?}",
            other, opponent_hs, our_hs
        ),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> (HandShape, HandShape) {
    let hands: Vec<&str> = line.split(" ").collect();
    assert_eq!(hands.len(), 2);
    let first_handshape = match hands[0] {
        "A" => Ok(HandShape::Rock),
        "B" => Ok(HandShape::Paper),
        "C" => Ok(HandShape::Scissors),
        other => Err(format!("Invalid handshape: {}", other)),
    };
    let second_handshape = match hands[1] {
        "X" => Ok(HandShape::Rock),
        "Y" => Ok(HandShape::Paper),
        "Z" => Ok(HandShape::Scissors),
        other => Err(format!("Invalid handshape: {}", other)),
    };

    (first_handshape.unwrap(), second_handshape.unwrap())
}

fn main() {
    let mut total_score = 0;
    let lines = read_lines("input.txt").unwrap();
    for line in lines {
        let (opponent_handshape, our_handshape) = parse_line(line.unwrap());
        total_score += score_for_handshape(&our_handshape)
            + score_for_round(&opponent_handshape, &our_handshape)
    }

    println!("{}", total_score)
}
