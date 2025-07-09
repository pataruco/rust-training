#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Location {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Robot {
    location: Location,
    direction: Direction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Forward,
    Left,
    Right,
}

impl Robot {
    fn move_forward(&mut self) {
        self.location = match self.direction {
            Direction::North => Location {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Direction::East => Location {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Direction::South => Location {
                x: self.location.x,
                y: self.location.y - 1,
            },
            Direction::West => Location {
                x: self.location.x - 1,
                y: self.location.y,
            },
        };
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn follows(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Left => self.turn_left(),
                Instruction::Forward => self.move_forward(),
                Instruction::Right => self.turn_right(),
            }
        }
    }
}

impl Direction {
    #[must_use]
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn directions_turns_left() {
        let north = Direction::North;

        let west = north.turn_left();
        let expected = Direction::West;
        assert_eq!(west, expected);

        let south = west.turn_left();
        let expected = Direction::South;
        assert_eq!(south, expected);

        let east = south.turn_left();
        let expected = Direction::East;
        assert_eq!(east, expected);

        let north = east.turn_left();
        let expected = Direction::North;
        assert_eq!(north, expected);
    }

    #[test]
    fn directions_turns_right() {
        let north = Direction::North;

        let east = north.turn_right();
        let expected = Direction::East;
        assert_eq!(east, expected);

        let south = east.turn_right();
        let expected = Direction::South;
        assert_eq!(south, expected);

        let west = south.turn_right();
        let expected = Direction::West;
        assert_eq!(west, expected);

        let north = west.turn_right();
        let expected = Direction::North;
        assert_eq!(north, expected);
    }

    #[test]
    fn robot_moves_forward() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        robot.move_forward();

        let expected = Location { x: 0, y: 1 };
        assert_eq!(robot.location, expected);
    }

    #[test]
    fn robot_moves_left() {
        let mut robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        robot.turn_left();

        let expected = Direction::West;

        assert_eq!(robot.direction, expected);
    }

    #[test]
    fn robot_follows_instructions() {
        let instructions = vec![Instruction::Left, Instruction::Forward, Instruction::Right];

        let mut robot = Robot {
            location: Location { x: 1, y: 0 },
            direction: Direction::North,
        };

        let expected = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        robot.follows(&instructions);

        assert_eq!(robot, expected);
    }
}
