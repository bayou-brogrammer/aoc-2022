#![feature(try_blocks)]
#![feature(control_flow_enum)]

use std::ops::ControlFlow;

use common::*;

pub fn day8(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = (width as i32, height as i32);

    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .fold(vec![], |mut acc, curr| {
            acc.extend(curr);
            acc
        });

    grid.iter()
        .enumerate()
        .map(|(i, ele)| {
            let start = idx_to_point(i, width);
            is_edge_coord(start, size) ||
                Direction::all()
                    .filter(|direction| {
                        DirectionIterator::new(start, size, *direction)
                            .all(|direction_point| is_visible(*ele, grid[idx(direction_point, width as i32)]))
                    })
                    .count() >
                    0
        })
        .filter(|&b| b)
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = (width as i32, height as i32);

    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .fold(vec![], |mut acc, curr| {
            acc.extend(curr);
            acc
        });

    grid.iter()
        .enumerate()
        .map(|(i, ele)| {
            let start = idx_to_point(i, width);
            if is_edge_coord(start, size) {
                0
            } else {
                Direction::all()
                    .map(|direction| {
                        DirectionIterator::new(start, size, direction)
                            .try_fold(0_i32, |count, direction_point| {
                                let other_pt = grid[idx(direction_point, width as i32)];
                                let is_visible = is_visible(*ele, other_pt);

                                if !is_visible || is_edge_coord(direction_point, size) {
                                    ControlFlow::Break(count + 1)
                                } else {
                                    ControlFlow::Continue(count + 1)
                                }
                            })
                            .break_value()
                            .unwrap_or(0)
                    })
                    .product()
            }
        })
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add<(i32, i32)> for GridPoint {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

pub struct DirectionIterator {
    size: (i32, i32),
    point: GridPoint,
    direction: Direction,
}

impl DirectionIterator {
    pub fn new(start: GridPoint, size: (i32, i32), direction: Direction) -> Self {
        Self {
            size,
            direction,
            point: start,
        }
    }
}

impl Iterator for DirectionIterator {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.x <= 0 ||
            self.point.y <= 0 ||
            self.point.x >= self.size.0 - 1 ||
            self.point.y >= self.size.1 - 1
        {
            return None;
        }

        let pt = self.point;
        let coord = self.direction.coord();
        self.point = pt + coord;

        Some(self.point)
    }
}

pub fn idx(pt: GridPoint, width: i32) -> usize { (pt.y * width + pt.x) as usize }

pub fn idx_to_point(idx: usize, width: usize) -> GridPoint {
    GridPoint {
        x: (idx % width) as i32,
        y: (idx / width) as i32,
    }
}

pub fn is_edge_coord(pt: GridPoint, size: (i32, i32)) -> bool {
    pt.x == 0 || pt.y == 0 || pt.x == size.0 - 1 || pt.y == size.1 - 1
}

pub fn is_visible(start: u32, other: u32) -> bool { start > other }
