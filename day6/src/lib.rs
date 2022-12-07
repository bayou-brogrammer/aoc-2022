use std::collections::BTreeSet;

pub fn day6(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<_>>();
            slice.len() == set.len()
        })
        .map(|(i, _slice)| i + 1 + 3)
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<_>>();
            slice.len() == set.len()
        })
        .map(|(i, _slice)| i + 14)
        .unwrap()
        .to_string()
}
