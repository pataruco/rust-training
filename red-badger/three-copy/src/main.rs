use three_copy::{Direction, Grid, Instruction, Location, Robot};

fn main() {
    let grid = Grid::new(Location { x: 5, y: 5 });

    let robot = Robot {
        location: Location { x: 1, y: 2 },
        direction: Direction::North,
    };
    let instructions = vec![
        Instruction::Left,
        Instruction::Forward,
        Instruction::Right,
        Instruction::Forward,
    ];

    println!("{:?}", robot.follows(&instructions, &grid));
}
