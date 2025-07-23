use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Grid(Location);

impl Grid {
    pub fn new(location: Location) -> Self {
        Grid(location)
    }
}

#[must_use]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Robot {
    pub location: Location,
    pub direction: Direction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LostRobot {
    location: Location,
    direction: Direction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FinishedRobot {
    location: Location,
    direction: Direction,
}

impl From<Robot> for LostRobot {
    fn from(robot: Robot) -> Self {
        LostRobot {
            location: robot.location,
            direction: robot.direction,
        }
    }
}

impl From<Robot> for FinishedRobot {
    fn from(robot: Robot) -> Self {
        FinishedRobot {
            location: robot.location,
            direction: robot.direction,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Forward,
    Left,
    Right,
}

impl Robot {
    pub fn move_forward(mut self, Grid(Location { x, y }): &Grid) -> Result<Self, LostRobot> {
        self.location = match self.direction {
            Direction::North if self.location.y + 1 < *y => Location {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Direction::East if self.location.x + 1 < *x => Location {
                x: self.location.x + 1,
                y: self.location.y,
            },
            Direction::South if self.location.y >= 1 => Location {
                x: self.location.x,
                y: self.location.y - 1,
            },
            Direction::West if self.location.x >= 1 => Location {
                x: self.location.x - 1,
                y: self.location.y,
            },
            _ => {
                return Err(self.clone().into());
            }
        };
        Ok(self)
    }

    fn turn_left(mut self) -> Self {
        self.direction = self.direction.turn_left();
        return self;
    }

    fn turn_right(mut self) -> Self {
        self.direction = self.direction.turn_right();
        return self;
    }

    pub fn follows(
        self,
        instructions: &[Instruction],
        grid: &Grid,
    ) -> Result<FinishedRobot, LostRobot> {
        let mut robot = self;
        for instruction in instructions {
            robot = match instruction {
                Instruction::Left => robot.turn_left(),
                Instruction::Right => robot.turn_right(),
                Instruction::Forward => robot.move_forward(grid)?,
            };
        }
        Ok(robot.into())
    }

    pub fn parse(input: &str) -> Result<Robot, ParseError> {
        let mut parts = input.split_whitespace();

        if let (Some(x), Some(y), Some(d)) = (parts.next(), parts.next(), parts.next()) {
            let x = x.parse()?;
            let y = y.parse()?;
            let direction = match d {
                "N" => Direction::North,
                "E" => Direction::East,
                "S" => Direction::South,
                "W" => Direction::West,
                _ => return Err(ParseError::InvalidDirection),
            };

            let robot = Robot {
                direction,
                location: Location { x, y },
            };
            Ok(robot)
        } else {
            Err(ParseError::InvalidInput)
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid direction")]
    InvalidDirection,
    #[error("Invalid coordinate")]
    InvalidCoordinate(#[from] ParseIntError),
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
    fn parse() {
        let robot = Robot::parse("1    2   N").unwrap();

        let expected = Robot {
            direction: Direction::North,
            location: Location { x: 1, y: 2 },
        };

        assert_eq!(robot, expected);
    }

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
        let grid = Grid::new(Location { x: 5, y: 5 });
        let robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        let expected_robot = robot.move_forward(&grid).unwrap();

        let expected = Location { x: 0, y: 1 };
        assert_eq!(expected_robot.location, expected);
    }

    #[test]
    fn robot_moves_left() {
        let robot = Robot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        let expected_robot = robot.turn_left();

        let expected = Direction::West;

        assert_eq!(expected_robot.direction, expected);
    }

    #[test]
    fn robot_follows_instructions() {
        let grid = Grid::new(Location { x: 5, y: 5 });
        let instructions = vec![Instruction::Left, Instruction::Forward, Instruction::Right];

        let robot = Robot {
            location: Location { x: 1, y: 0 },
            direction: Direction::North,
        };

        let expected = FinishedRobot {
            location: Location { x: 0, y: 0 },
            direction: Direction::North,
        };

        let moved_robot = robot.follows(&instructions, &grid).unwrap();

        assert_eq!(moved_robot, expected);
    }

    #[test]
    fn robot_follow_multiple_instructions_and_falls_off_the_grid() {
        let grid = Grid::new(Location { x: 5, y: 5 });
        let robot = Robot {
            location: Location { x: 0, y: 2 },
            direction: Direction::North,
        };

        let instructions = vec![Instruction::Left, Instruction::Forward];

        let movement = robot.follows(&instructions, &grid);

        let expected = LostRobot {
            location: Location { x: 0, y: 2 },
            direction: Direction::West,
        };

        assert!(movement.is_err());
        assert_eq!(movement.unwrap_err(), expected)
    }
}
