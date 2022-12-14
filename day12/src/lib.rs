use std::cmp::Ordering;

use common::{distance, idx, idx_to_point, GridPoint};
use pathfinding::prelude::*;

pub fn day12(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}
g
pub fn part1(input: &str) -> String {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = (width as i32, height as i32);

    let chars = input.lines().map(|line| line.chars().collect::<Vec<_>>()).fold(vec![], |mut acc, curr| {
        acc.extend(curr);
        acc
    });

    let start_idx = chars.iter().position(|&c| c == 'S').unwrap();
    let end_idx = chars.iter().position(|&c| c == 'E').unwrap();

    let chars = chars
        .iter()
        .map(|&c| match c {
            'S' => 'a',
            'E' => 'z',
            v => v,
        })
        .collect::<Vec<_>>();

    let cells = chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let pt = idx_to_point(i, width);
            Position { c: *c, pt }
        })
        .collect::<Vec<_>>();

    let grid = Grid::new(size, cells);
    let path = grid.astar(&grid[start_idx], &grid[end_idx]).unwrap();

    (path.0.len() - 1).to_string()
}

fn part2(input: &str) -> String {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = (width as i32, height as i32);

    let chars = input.lines().map(|line| line.chars().collect::<Vec<_>>()).fold(vec![], |mut acc, curr| {
        acc.extend(curr);
        acc
    });

    let end_idx = chars.iter().position(|&c| c == 'E').unwrap();

    let chars = chars
        .iter()
        .map(|&c| match c {
            'S' => 'a',
            'E' => 'z',
            v => v,
        })
        .collect::<Vec<_>>();

    let all_starts = chars.iter().enumerate().filter(|(_, &c)| c == 'a').map(|(i, _)| i).collect::<Vec<_>>();

    let cells = chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let pt = idx_to_point(i, width);
            Position { c: *c, pt }
        })
        .collect::<Vec<_>>();

    let grid = Grid::new(size, cells);
    (all_starts
        .iter()
        .filter_map(|&start_idx| grid.astar(&grid[start_idx], &grid[end_idx]).map(|path| path.0.len()))
        .min()
        .unwrap() -
        1)
    .to_string()
}

///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub c: char,
    pub pt: GridPoint,
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, other: (i32, i32)) -> Position {
        Position {
            c: self.c,
            pt: self.pt + other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Position>,
    pub size: (i32, i32),
}

impl Grid {
    pub fn new(size: (i32, i32), cells: Vec<Position>) -> Grid { Grid { cells, size } }

    pub fn astar(&self, start: &Position, end: &Position) -> Option<(Vec<Position>, u32)> {
        astar(
            start,
            |p| self.successors(p),
            |p| distance(p.pt, end.pt) as u32,
            |p| p.pt == end.pt,
        )
    }

    fn successors(&self, pos: &Position) -> Vec<(Position, u32)> {
        let mut successors = Vec::new();

        for direction in common::Direction::all() {
            let p = pos.pt + direction.coord();
            if p.x < 0 || p.y < 0 || p.x >= self.size.0 || p.y >= self.size.1 {
                continue;
            }

            let from = pos.c as u32;
            let other = self[p].c as u32;
            match other.cmp(&from) {
                Ordering::Less | Ordering::Equal => {
                    successors.push((self[p], 1));
                },
                Ordering::Greater => {
                    let diff = i32::abs(from as i32 - other as i32);
                    if diff <= 1 {
                        successors.push((self[p], 1));
                    }
                },
            }
        }

        successors
    }
}

impl std::ops::Index<usize> for Grid {
    type Output = Position;

    fn index(&self, index: usize) -> &Self::Output { &self.cells[index] }
}

impl std::ops::Index<GridPoint> for Grid {
    type Output = Position;

    fn index(&self, pt: GridPoint) -> &Self::Output { &self.cells[idx(pt, self.size.0)] }
}
