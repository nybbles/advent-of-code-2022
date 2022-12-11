use itertools::Itertools;
use std::collections::HashSet;
use utils::read_lines;

fn get_item_priority(item: char) -> u32 {
    let mut item_priority = item as u32;

    if item_priority >= 'a' as u32 {
        item_priority -= 'a' as u32;
    } else {
        item_priority -= 'A' as u32;
        item_priority += 26;
    }
    item_priority += 1;
    item_priority
}

fn main() {
    let mut sum = 0;
    let lines = read_lines("input.txt").unwrap();
    let elves_per_group = 3;

    for group in &lines.chunks(elves_per_group) {
        let mut knapsacks = group.map(|ks| ks.unwrap().chars().collect::<HashSet<char>>());
        let first_knapsack = knapsacks.nth(0).unwrap();
        let common_items = knapsacks.fold(first_knapsack, |l, r| {
            l.intersection(&r).map(|x| *x).collect::<HashSet<char>>()
        });
        assert_eq!(common_items.len(), 1);
        let badge = *common_items.iter().nth(0).unwrap();

        let priority = get_item_priority(badge);
        sum += priority;
    }

    println!("{}", sum)
}
