use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};

pub fn day5(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();

    for Move { number, from, to } in moves.iter() {
        let len = crate_stacks[*from as usize].len();
        let drained =
            crate_stacks[*from as usize].drain((len - *number as usize)..).rev().collect::<Vec<&str>>();

        crate_stacks[*to as usize].extend(drained);
    }

    crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();

    for Move { number, from, to } in moves.iter() {
        let len = crate_stacks[*from as usize].len();
        let drained = crate_stacks[*from as usize].drain((len - *number as usize)..).collect::<Vec<&str>>();

        crate_stacks[*to as usize].extend(drained);
    }

    crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect::<String>()
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, r) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, r))
}

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}

fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((input, Move {
        number,
        to: to - 1,
        from: from - 1,
    }))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];
    (0..=crates_horizontal.len()).for_each(|_| {
        crates_vertical.push(vec![]);
    });

    crates_horizontal.iter().rev().for_each(|vec| {
        vec.iter().enumerate().for_each(|(i, c)| {
            crates_vertical[i].push(*c);
        });
    });

    let final_crates: Vec<Vec<&str>> =
        crates_vertical.iter().map(|vec| vec.iter().filter_map(|v| *v).collect()).collect();

    Ok((input, (final_crates, moves)))
}
