use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

pub type Assignments = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, (start, end)))
}

fn section_assignments(input: &str) -> IResult<&str, Assignments> {
    let (input, ranges) = separated_list1(newline, line)(input)?;

    Ok((input, ranges))
}

pub fn day4(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();
    let (_, assignments) = section_assignments(&file).unwrap();

    println!("p1: {}", part1(&assignments));
    println!("p2: {}", part2(&assignments));
}

fn part1(assignments: &Assignments) -> String {
    assignments
        .iter()
        .filter(|(range_a, range_b)| {
            range_a
                .clone()
                .into_iter()
                .all(|a| range_b.contains(&a) || range_b.clone().into_iter().all(|b| range_a.contains(&b)))
        })
        .count()
        .to_string()
}

fn part2(assignments: &Assignments) -> String {
    assignments
        .iter()
        .filter(|(range_a, range_b)| {
            range_a
                .clone()
                .into_iter()
                .any(|a| range_b.contains(&a) || range_b.clone().into_iter().any(|b| range_a.contains(&b)))
        })
        .count()
        .to_string()
}
