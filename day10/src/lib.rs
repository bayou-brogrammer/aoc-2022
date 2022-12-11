use std::{collections::BTreeMap, ops::RangeInclusive};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    *,
};

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}
use Instruction::*;

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, vecs) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i32).map(Add),
        )),
    )(input)?;

    Ok((input, vecs))
}

pub fn day10(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    let notable_cycles = [20, 60, 100, 140, 180, 220];
    let (_, instructions) = instruction_set(&file).unwrap();

    println!("p1: {}", part1(notable_cycles, &instructions));
    println!("p2:\n {}", part2(&instructions));
}

fn part1(notable_cycles: [u32; 6], instructions: &[Instruction]) -> String {
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();

    instructions.iter().for_each(|instruction| {
        if notable_cycles.contains(&(cycles + 1)) {
            scores.insert(cycles + 1, (cycles as i32 + 1) * x);
        }

        if notable_cycles.contains(&(cycles + 2)) {
            scores.insert(cycles + 2, (cycles as i32 + 2) * x);
        }

        cycles += instruction.cycles();
        match instruction {
            Noop => {},
            Add(num) => {
                x += num;
            },
        };
    });

    scores.values().sum::<i32>().to_string()
}

fn part2(instructions: &[Instruction]) -> String {
    let computer = instructions.iter().fold(Computer::new(), |mut computer, instruction| {
        computer.interpret(instruction);
        computer
    });

    computer.to_string()
}

struct Cycle<'a> {
    pixel: u32,
    computer: &'a mut Computer,
}

impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) { self.computer.cycles += 1; }
}

struct Computer {
    x: i32,
    cycles: u32,
    pixels: String,
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels.chars().chunks(40).into_iter().map(|chunk| chunk.collect::<String>()).join("\n")
        )
    }
}

impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycles: 0,
            pixels: "".to_string(),
        }
    }

    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            pixel: self.cycles % 40,
            computer: self,
        }
    }

    fn sprite_range(&self) -> RangeInclusive<i32> { (self.x - 1)..=(self.x + 1) }

    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.cycles() {
            let cycle_guard = self.start_cycle();

            if cycle_guard.computer.sprite_range().contains(&(cycle_guard.pixel as i32)) {
                cycle_guard.computer.pixels.push('#');
            } else {
                cycle_guard.computer.pixels.push('.');
            }
        }

        match instruction {
            Noop => {},
            Add(num) => {
                self.x += num;
            },
        };
    }
}
