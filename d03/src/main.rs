use std::collections::HashSet;

use utils::read_lines;

fn main() {
    let mut sum = 0;
    let lines = read_lines("input.txt").unwrap();
    for line in lines {
        let line = line.unwrap();
        let (lhs, rhs) = line.split_at(line.len() / 2);
        let lhs_set = lhs.chars().collect::<HashSet<char>>();
        let rhs_set = rhs.chars().collect::<HashSet<char>>();

        let mut overlap = lhs_set.intersection(&rhs_set);
        let overlapping_item = *(overlap.nth(0).unwrap());

        let mut overlapping_item_priority = overlapping_item as u32;
        assert_eq!(overlap.count(), 0);

        if overlapping_item_priority >= 'a' as u32 {
            overlapping_item_priority -= 'a' as u32;
        } else {
            overlapping_item_priority -= 'A' as u32;
            overlapping_item_priority += 26;
        }
        overlapping_item_priority += 1;

        println!("{} -> {}", overlapping_item, overlapping_item_priority);

        sum += overlapping_item_priority;
    }

    println!("{}", sum)
}
