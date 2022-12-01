use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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
    let mut calorie_counts = BinaryHeap::new();
    let max_retained_counts = 3;

    let mut curr_calories = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                calorie_counts.push(Reverse(curr_calories));
                curr_calories = 0;

                while calorie_counts.len() > max_retained_counts {
                    calorie_counts.pop();
                }
            } else {
                let calories = line.parse::<i32>().unwrap();
                curr_calories = curr_calories + calories;
            }
        }
    }

    let total_calories: i32 = calorie_counts.iter().map(|Reverse(x)| x).sum();

    println!(
        "Total calories of top {} is {}",
        max_retained_counts, total_calories
    )
}
