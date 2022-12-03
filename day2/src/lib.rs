use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Rps::Scissors && other == &Rps::Rock {
            Some(Ordering::Less)
        } else if self == &Rps::Rock && other == &Rps::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Rps {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

pub fn day2(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(file: &str) -> String {
    let result: u32 = file
        .lines()
        .map(|line| {
            let moves = line
                .split_whitespace()
                .map(|move_str| move_str.parse::<Rps>().unwrap())
                .collect::<Vec<Rps>>();

            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => moves[1] as u32,
                None => {
                    panic!("moves should be comparable")
                },
            }
        })
        .sum();

    result.to_string()
}

fn part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split_whitespace().collect();
            let opponent_move = moves[0].parse::<Rps>().unwrap();

            match moves[1] {
                // lose
                "X" => {
                    let our_move = match opponent_move {
                        Rps::Rock => Rps::Scissors,
                        Rps::Paper => Rps::Rock,
                        Rps::Scissors => Rps::Paper,
                    };
                    our_move as u32
                },
                // Draw
                "Y" => 3 + opponent_move as u32,
                // Win
                "Z" => {
                    let our_move = match opponent_move {
                        Rps::Rock => Rps::Paper,
                        Rps::Paper => Rps::Scissors,
                        Rps::Scissors => Rps::Rock,
                    };
                    6 + our_move as u32
                },
                _ => panic!("Unknown move"),
            }
        })
        .sum();

    result.to_string()
}
