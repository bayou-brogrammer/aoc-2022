use std::collections::HashSet;

use ::lending_iterator::prelude::*;
use common::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        complete::char('L').map(|_| Direction::West),
        complete::char('R').map(|_| Direction::East),
        complete::char('U').map(|_| Direction::North),
        complete::char('D').map(|_| Direction::South),
    ))(input)?;
    Ok((input, dir))
}

fn moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, vecs) = separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;
    let vecs = vecs.iter().flat_map(|(dir, repeat)| vec![*dir; *repeat as usize]).collect();
    Ok((input, vecs))
}

pub fn day9(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    let (_, move_set) = moves(input).unwrap();

    let mut head = GridPoint::new(0, 0);
    let mut tail = GridPoint::new(0, 0);
    let mut tail_positions = HashSet::from([tail]);

    for head_move in move_set.iter() {
        match head_move {
            Direction::West => {
                head.x -= 1;
            },
            Direction::East => {
                head.x += 1;
            },
            Direction::North => {
                head.y += 1;
            },
            Direction::South => {
                head.y -= 1;
            },
        }

        let x_range = (head.x - 1)..=(head.x + 1);
        let y_range = (head.y - 1)..=(head.y + 1);

        let tail_is_connected =
            x_range.clone().cartesian_product(y_range.clone()).any(|tuple| tail.eq(&tuple));

        if !tail_is_connected {
            let mut new_tail = head;

            match head_move {
                Direction::West => {
                    new_tail.x += 1;
                },
                Direction::East => {
                    new_tail.x -= 1;
                },
                Direction::North => {
                    new_tail.y -= 1;
                },
                Direction::South => {
                    new_tail.y += 1;
                },
            }

            tail = new_tail;
            tail_positions.insert(new_tail);
        }
    }

    tail_positions.len().to_string()
}

fn part2(input: &str) -> String {
    let (_, move_set) = moves(input).unwrap();

    let mut rope = [GridPoint::new(0, 0); 10];
    let mut tail_positions = HashSet::from([*rope.last().unwrap()]);

    for head_move in move_set.iter() {
        match head_move {
            Direction::West => {
                rope[0].x -= 1;
            },
            Direction::East => {
                rope[0].x += 1;
            },
            Direction::North => {
                rope[0].y += 1;
            },
            Direction::South => {
                rope[0].y -= 1;
            },
        }

        rope.windows_mut::<2>().for_each(|[ref mut head, ref mut tail]| {
            let x_range = (head.x - 1)..=(head.x + 1);
            let y_range = (head.y - 1)..=(head.y + 1);

            let tail_is_connected = x_range.cartesian_product(y_range).any(|tuple| (*tail).eq(&tuple));

            if !tail_is_connected {
                if head.x == tail.x {
                    if head.y > tail.y {
                        tail.y += 1;
                    } else {
                        tail.y -= 1;
                    }
                } else if head.y == tail.y {
                    if head.x > tail.x {
                        tail.x += 1;
                    } else {
                        tail.x -= 1;
                    }
                } else {
                    let x_range = (head.x - 1)..=(head.x + 1);
                    let y_range = (head.y - 1)..=(head.y + 1);
                    let head_3x3 = x_range.cartesian_product(y_range).collect::<Vec<_>>();

                    let x_range = (tail.x - 1)..=(tail.x + 1);
                    let y_range = (tail.y - 1)..=(tail.y + 1);
                    let maybe_new_tail: Vec<GridPoint> = x_range
                        .cartesian_product(y_range)
                        .filter(|tuple| head_3x3.contains(tuple))
                        .map(|a| a.into())
                        .collect();

                    match maybe_new_tail.len() {
                        2 => {
                            let new_head_cross_positions = [
                                (head.x - 1, head.y),
                                (head.x + 1, head.y),
                                (head.x, head.y - 1),
                                (head.x, head.y + 1),
                            ];

                            let next = maybe_new_tail
                                .iter()
                                .find(|&tuple| new_head_cross_positions.contains(&(tuple.x, tuple.y)))
                                .unwrap();
                            *tail = *next;
                        },
                        1 => {
                            *tail = maybe_new_tail[0];
                        },
                        _ => {
                            panic!("unknown tail length");
                        },
                    };
                }
            }
        });

        tail_positions.insert(*rope.last().unwrap());
    }

    tail_positions.len().to_string()
}
