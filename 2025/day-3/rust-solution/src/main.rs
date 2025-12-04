use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

const NUM_DIGITS: usize = 12;

fn main() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;
    let mut total_joltage: u64 = 0;

    for bank in content.lines() {
        let bytes = bank.as_bytes();
        let mut max_num: u64 = 0;

        // Left -> right, narrowing the search window
        let mut start_idx: usize = 0;

        for place in (1..=NUM_DIGITS).rev() {
            // Remember, end_idx needs to be inclusive because of the logic in get_max_by_earliest_key!
            let end_idx = bytes.len() - place;
            let (index, digit) = get_max_by_earliest_key(bytes, start_idx, end_idx);

            max_num += digit * 10u64.pow((place - 1) as u32);
            start_idx = index + 1;
        }

        total_joltage += max_num;
    }

    println!("Total Joltage: {total_joltage}");
    Ok(())
}

fn get_max_by_earliest_key(bytes: &[u8], start: usize, end_inclusive: usize) -> (usize, u64) {
    let mut max_idx = start;
    let mut max_digit = 0;

    for (offset, &b) in bytes[start..=end_inclusive].iter().enumerate() {
        // **IMPORTANT**
        // enumerate() will _always_ start counting from zero! We need to
        // take the offset into account here...
        let idx = start + offset;
        let digit = b - b'0';

        if digit > max_digit {
            max_idx = idx;
            max_digit = digit;
        }

        if digit == 9 {
            // Early exit; we can't do better than 9 for single digits!
            break;
        }
    }
    (max_idx, max_digit as u64)
}
