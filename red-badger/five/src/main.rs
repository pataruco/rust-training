use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use five::{Grid, Instruction, Location, Robot};

use tokio::{net::TcpListener, runtime::Builder};

fn main() {
    let body = async {
        let app = Router::new()
            .route("/", get(hello_world))
            .route("/robot", post(move_robot));

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    };

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime")
        .block_on(body);
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn move_robot(body: String) -> Result<String, (StatusCode, String)> {
    let grid = Grid::new(Location { x: 5, y: 5 });

    let mut lines = body.lines();

    let (Some(position), Some(instructions)) = (lines.next(), lines.next()) else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Need at least two lines".to_string(),
        ));
    };

    let robot = match position.parse::<Robot>() {
        Ok(robot) => robot,
        Err(err) => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Failed to parse robot position: {}", err),
            ));
        }
    };

    let instructions = match Instruction::parse_list(instructions) {
        Ok(instruction) => instruction,
        Err(err) => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Failed to parse robot instruction: {}", err),
            ));
        }
    };

    let output = match robot.follows(&instructions, &grid) {
        Ok(output) => output,
        Err(err) => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Failed to execute robot instruction: {}", err),
            ));
        }
    };

    Ok(output.to_string())
}

// curl -d "4 2 N"\n"LFFFLRR" -vvv  http://0.0.0.0:3000/robot
