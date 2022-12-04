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

enum RoundEnd {
    Win,
    Lose,
    Draw,
}

const NHANDSHAPES: i32 = 3;

fn score_for_handshape(hs: &HandShape) -> i32 {
    match hs {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    }
}

fn score_for_roundend(re: &RoundEnd) -> i32 {
    match re {
        RoundEnd::Win => 6,
        RoundEnd::Draw => 3,
        RoundEnd::Lose => 0,
    }
}

fn get_handshape_score_for_desired_roundend(
    opponent_hs: &HandShape,
    desired_roundend: &RoundEnd,
) -> i32 {
    let offset = match desired_roundend {
        RoundEnd::Win => 1,
        RoundEnd::Draw => 0,
        RoundEnd::Lose => -1,
    };
    (((score_for_handshape(opponent_hs) - 1 + offset) + NHANDSHAPES) % NHANDSHAPES) + 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> (HandShape, RoundEnd) {
    let hands: Vec<&str> = line.split(" ").collect();
    assert_eq!(hands.len(), 2);
    let opponent_handshape = match hands[0] {
        "A" => Ok(HandShape::Rock),
        "B" => Ok(HandShape::Paper),
        "C" => Ok(HandShape::Scissors),
        other => Err(format!("Invalid handshape: {}", other)),
    };
    let desired_roundend = match hands[1] {
        "X" => Ok(RoundEnd::Lose),
        "Y" => Ok(RoundEnd::Draw),
        "Z" => Ok(RoundEnd::Win),
        other => Err(format!("Invalid round end: {}", other)),
    };

    (opponent_handshape.unwrap(), desired_roundend.unwrap())
}

fn main() {
    let mut total_score = 0;
    let lines = read_lines("input.txt").unwrap();
    for line in lines {
        let (opponent_handshape, desired_roundend) = parse_line(line.unwrap());
        let handshape_score =
            get_handshape_score_for_desired_roundend(&opponent_handshape, &desired_roundend);

        total_score += handshape_score + score_for_roundend(&desired_roundend);
    }

    println!("{}", total_score)
}
