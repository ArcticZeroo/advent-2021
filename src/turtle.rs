use crate::world;
use crate::world::Position3D;

/// 3D turtle where Up/Down are on Z, Forward/Back are on X, and Left/Right are on Y
pub struct Turtle3D {
    pub position: world::Position3D,
}

impl Turtle3D {
    pub fn new() -> Self {
        Turtle3D {
            position: Position3D::new()
        }
    }

    pub fn do_move(&mut self, direction: world::MovementDirection, count: i64) {
        match direction {
            world::MovementDirection::Up => self.position.z += count,
            world::MovementDirection::Down => self.position.z -= count,
            world::MovementDirection::Forward => self.position.x += count,
            world::MovementDirection::Back => self.position.x -= count,
            world::MovementDirection::Right => self.position.y += count,
            world::MovementDirection::Left => self.position.y -= count,
        }
    }

    pub fn do_move_once(&mut self, direction: world::MovementDirection) {
        self.do_move(direction, 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::turtle::Turtle3D;
    use crate::world::{MovementDirection, Position3D};

    #[test]
    pub fn moving() {
        let mut turtle = Turtle3D::new();

        assert_eq!(Position3D { x: 0, y: 0, z: 0 }, turtle.position);

        turtle.do_move_once(MovementDirection::Up);
        assert_eq!(Position3D { x: 0, y: 0, z: 1 }, turtle.position);

        turtle.do_move_once(MovementDirection::Up);
        assert_eq!(Position3D { x: 0, y: 0, z: 2 }, turtle.position);

        turtle.do_move_once(MovementDirection::Down);
        assert_eq!(Position3D { x: 0, y: 0, z: 1 }, turtle.position);

        turtle.do_move_once(MovementDirection::Left);
        assert_eq!(Position3D { x: 0, y: -1, z: 1 }, turtle.position);

        turtle.do_move_once(MovementDirection::Back);
        assert_eq!(Position3D { x: -1, y: -1, z: 1 }, turtle.position);

        turtle.do_move_once(MovementDirection::Right);
        assert_eq!(Position3D { x: -1, y: 0, z: 1 }, turtle.position);

        turtle.do_move_once(MovementDirection::Forward);
        assert_eq!(Position3D { x: 0, y: 0, z: 1 }, turtle.position);
    }
}