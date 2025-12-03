use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;

    let ranges = content.split(',');

    let mut total_sum: i64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        let (start, end) = match bounds.len() {
            2 => (bounds[0].trim(), bounds[1].trim()),
            _ => continue,
        };

        let start_num: i64 = start.parse()?;
        let end_num: i64 = end.parse()?;

        for number in start_num..=end_num {
            if has_repeated_pattern(&number.to_string()) {
                total_sum += number;
            }
        }
    }

    println!("Total Sum: {total_sum}");
    Ok(())
}

fn has_repeated_pattern(s: &str) -> bool {
    let len = s.len();

    for chunk_len in divisors::get_divisors(len) {
        let chunk = &s[..chunk_len];
        let bytes = chunk.as_bytes();
        let sbytes = s.as_bytes();

        if (0..len)
            .step_by(chunk_len)
            .all(|i| &sbytes[i..i + chunk_len] == bytes)
        {
            return true;
        }
    }

    false
}
