use std::ops::Sub;

use crate::GridPoint;

pub fn idx(pt: GridPoint, width: i32) -> usize { (pt.y * width + pt.x) as usize }

pub fn idx_to_point(idx: usize, width: usize) -> GridPoint {
    GridPoint {
        x: (idx % width) as i32,
        y: (idx / width) as i32,
    }
}

pub fn distance(start: GridPoint, end: GridPoint) -> i32 { start.sub(end).abs().max_element() }
