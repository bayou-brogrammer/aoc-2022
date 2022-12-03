#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn day3(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let a = &line[0..sack_length];
            let b = &line[sack_length..(sack_length * 2)];

            scores
                .get(&a.chars().find(|c| b.contains(*c)).expect("No common char found"))
                .expect("No score found")
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[x, y, z]| {
            scores
                .get(&x.chars().find(|c| y.contains(*c) && z.contains(*c)).expect("No common char found"))
                .expect("No score found")
        })
        .sum::<usize>()
        .to_string()
}
