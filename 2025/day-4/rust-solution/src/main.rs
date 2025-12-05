// Advent of Code 2025 - Day 4
//
// This solution processes a 2D grid of characters that is actually stored as a single
// UTF-8 file containing newline-terminated rows. The trick is that each line of the file
// ends with a '\n', so despite visually representing an N×M grid, the underlying byte
// representation has an extra byte per line. This means indexing must account for a
// stride of (width + 1).
//
// The grid is interpreted directly as bytes using `fs::read()` so that mutations are
// possible. Using `fs::read_to_string()` or working with `&str` slices would not allow
// mutation, because Rust strings (`&str`) are immutable. The borrow checker enforces
// that as long as any immutable view of the bytes exists, the backing data cannot be
// mutably accessed.
//
// Instead, we decode the UTF-8 once per iteration, extract the structural information
// we need (width & height), and then operate directly over the underlying mutable byte
// buffer.
//
// Important conceptual takeaways:
// - Mutable access requires ownership or `&mut [u8]`, NOT `&str`
// - Borrowing rules matter:
//        - As soon as you hold `&str` slices into a buffer,
//        - you cannot mutably modify that buffer until those slices are gone.
//
// The main simulation loops repeatedly:
//  - Counts adjacent '@' neighbors
//  – If a cell has fewer than 4 neighbors, it becomes 'x'
//  – Loop terminates once no changes occur
//
// **Lessons reinforced from this problem**:
//   - When immutable and mutable borrows conflict, shorten borrow lifetimes explicitly
//   - Avoid holding long-lived references into owned data during mutation
//   - Work on raw byte data when mutation is required
//   - Encapsulate grid indexing into arithmetic instead of slicing strings

use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut bytes = fs::read(FILE_PATH)?;
    let rolls_to_pickup = count_total_pickups(&mut bytes);

    println!("Rolls: {rolls_to_pickup}");
    Ok(())
}

fn count_total_pickups(bytes: &mut [u8]) -> u32 {
    let grid_str = std::str::from_utf8(bytes).expect("Invalid UTF-8");
    let lines: Vec<&str> = grid_str.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    let height = lines.len();
    let width = lines[0].len();
    // We must account for new-line escapge chars at the end of each line
    let stride = width + 1;

    let mut total = 0;

    loop {
        let mut changes = 0;
        for y in 0..height {
            for x in 0..width {
                // Treat 2D grid as flat array (which it is)
                let idx = y * stride + x;
                if bytes[idx] as char == '@' && can_pickup(bytes, x, y, width, height) {
                    bytes[idx] = b'x';
                    changes += 1;
                }
            }
        }

        if changes == 0 {
            // No more rolls to pickup, stop checking
            break;
        }

        total += changes;
    }

    total
}

fn can_pickup(bytes: &[u8], x: usize, y: usize, width: usize, height: usize) -> bool {
    let stride = width + 1;
    let mut adjacent_count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                // Skip the actual cell (not a neighbor)
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                // Out of bounds
                continue;
            }

            // Treating again 2D grid as flat array
            let nidx = ny as usize * stride + nx as usize;
            if bytes[nidx] as char == '@' {
                adjacent_count += 1;
            }
        }
    }

    adjacent_count < 4
}
