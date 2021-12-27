use std::cmp::Eq;

/// 3D movement direction
#[derive(Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Back
}

pub struct Position2D<T> {
    pub x: T,
    pub y: T
}

#[derive(Eq, PartialEq, Debug)]
pub struct Position3D {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Position3D {
    pub fn new() -> Self {
        Position3D {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

impl Default for Position3D {
    fn default() -> Self {
        Position3D::new()
    }
}