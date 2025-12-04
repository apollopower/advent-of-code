use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut content = fs::read(FILE_PATH)?;
    let mut rolls_to_pickup: u32 = 0;

    let mut current_rolls_to_pickup: u32 = 0;
    loop {
        let grid_string = std::str::from_utf8(&content)?;

        let lines: Vec<Vec<u8>> = grid_string
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();

        let height = lines.len();
        let width = lines[0].len();

        for y in 0..height {
            for x in 0..width {
                //let c = grid_string.lines().nth(y).unwrap().chars().nth(x).unwrap();
                let c = lines[y][x] as char;

                if c == '@' && attempt_pickup_roll(&mut content, x, y, width, height) {
                    current_rolls_to_pickup += 1;
                }
            }
        }

        if current_rolls_to_pickup == 0 {
            break;
        } else {
            rolls_to_pickup += current_rolls_to_pickup;
            current_rolls_to_pickup = 0;
        }
    }

    println!("Rolls: {rolls_to_pickup}");
    Ok(())
}

fn attempt_pickup_roll(bytes: &mut [u8], x: usize, y: usize, width: usize, height: usize) -> bool {
    let current_idx = y * (width + 1) + x;

    // Index = (row * row_length) + column
    let mut roll_num: u32 = 0;

    println!("{x}, {y} => ");

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let num_x = x as isize + dx;
            let num_y = y as isize + dy;

            if num_x < 0 || num_x >= width as isize || num_y < 0 || num_y >= height as isize {
                // Out of bounds
                continue;
            }

            let idx = (num_y * (width as isize + 1) + num_x) as usize;
            let c = bytes[idx] as char;

            if c == '@' {
                roll_num += 1;
            }
        }
    }

    if roll_num < 4 {
        bytes[current_idx] = b'x'; // to bytes, and then I need to convert back to the original grid
        // to mutate it?
        return true;
    }

    false
}
