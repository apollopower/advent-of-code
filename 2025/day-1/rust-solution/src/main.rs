use std::error;
use std::fs;

const LOCK_LENGTH: i32 = 100;
const START_POSITION: i32 = 50;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    //let file_path = "problem.txt";
    let content = fs::read_to_string(FILE_PATH)?;

    let mut current = START_POSITION;
    let mut zero_count = 0;

    for code in content.lines().filter(|l| l.len() >= 2) {
        let (direction, value) = code.split_at(1);
        let movement: i32 = value.parse()?;
        let movement = match direction {
            "L" => -movement,
            "R" => movement,
            _ => continue,
        };

        let distance_to_zero = if movement < 0 {
            current
        } else {
            (LOCK_LENGTH - current) % LOCK_LENGTH
        };

        let steps = movement.abs();

        let zero_hits = if distance_to_zero == 0 {
            steps / LOCK_LENGTH
        } else if steps < distance_to_zero {
            0
        } else {
            1 + (steps - distance_to_zero) / LOCK_LENGTH
        };

        zero_count += zero_hits;

        // Update position
        current = (current + movement).rem_euclid(LOCK_LENGTH);
    }

    println!("Zero_count: {zero_count}");

    Ok(())
}
