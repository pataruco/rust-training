use std::process;

use four::{Grid, Instruction, Location, ParseError, Robot};

fn main() -> Result<(), ParseError> {
    let grid = Grid::new(Location { x: 5, y: 5 });

    let robot: Robot = match "1 2 N".parse::<Robot>() {
        Ok(robot) => robot,
        Err(err) => {
            println!("Error parsing robot: {err}");
            process::exit(1)
        }
    };

    let instructions = match Instruction::parse_list("LFRF") {
        Ok(instructions) => instructions,
        Err(err) => {
            println!("Error parsing instructions: {err}");
            process::exit(1)
        }
    };

    let output = match robot.follows(&instructions, &grid) {
        Ok(finished_robot) => finished_robot.to_string(),
        Err(lost_robot) => lost_robot.to_string(),
    };

    println!("{output:?}");

    Ok(())
}
