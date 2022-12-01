use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut max_calories = 0;
    let mut curr_calories = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                if curr_calories > max_calories {
                    max_calories = curr_calories;
                }
                curr_calories = 0;
            } else {
                let calories = line.parse::<i32>().unwrap();
                curr_calories = curr_calories + calories;
            }
        }
    }
    println!("Max calories is {}", max_calories)
}
