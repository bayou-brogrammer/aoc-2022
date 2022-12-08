use std::ops::Range;

pub const NUM_DIRECTIONS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const fn coord(&self) -> (i32, i32) {
        match self {
            Self::East => (1, 0),
            Self::North => (0, -1),
            Self::West => (-1, 0),
            Self::South => (0, 1),
        }
    }

    pub const fn all() -> DirectionIter { DirectionIter::new() }
}

#[macro_export]
macro_rules! make_direction_iter {
    ($col_name:ident, $iter_name:ident, $type:ident, $count:expr) => {
        #[derive(Debug, Clone)]
        /// Iterate over all directions of the respectively-named type of direction
        pub struct $iter_name(Range<u8>);

        impl $iter_name {
            pub const fn new() -> Self { $iter_name(0..$count as u8) }
        }

        impl Iterator for $iter_name {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|n| unsafe { std::mem::transmute(n) })
            }
        }

        /// Represents a collection of the respectively-named type of direction
        #[derive(Debug, Clone, Copy)]
        pub struct $col_name;
        impl IntoIterator for $col_name {
            type IntoIter = $iter_name;
            type Item = $type;

            fn into_iter(self) -> Self::IntoIter { $iter_name::new() }
        }
    };
}

// IntoIter implementations for iterating over all directions of a type. E.g.:
// for direction in CardinalDirections { ... }
make_direction_iter! {Directions, DirectionIter, Direction, NUM_DIRECTIONS}
