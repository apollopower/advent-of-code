use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;
    let (ranges, _) = content.split_once("\n\n").unwrap();
    let mut valid_ids: u64 = 0;

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges.lines() {
        let (min, max) = range.split_once("-").unwrap();
        let min_num: u64 = min.parse()?;
        let max_num: u64 = max.parse()?;
        merged_ranges.push((min_num, max_num));
    }

    merged_ranges.sort_by_key(|r| r.0);

    let mut merged_final: Vec<(u64, u64)> = Vec::new();

    for (min, max) in merged_ranges {
        if let Some(last) = merged_final.last_mut() {
            if min <= last.1 {
                last.1 = last.1.max(max);
            } else {
                merged_final.push((min, max));
            }
        } else {
            merged_final.push((min, max));
        }
    }

    for (min, max) in merged_final {
        valid_ids += max - min + 1;
    }

    println!("Valid Ids: {valid_ids}");

    Ok(())
}

fn _part_one() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;
    let (ranges, ids) = content.split_once("\n\n").unwrap();
    let mut valid_ids: u32 = 0;

    for id in ids.lines() {
        let id_num: u64 = id.parse()?;

        for range in ranges.lines() {
            let (min, max) = range.split_once("-").unwrap();
            let min_num: u64 = min.parse()?;
            let max_num: u64 = max.parse()?;

            if id_num >= min_num && id_num <= max_num {
                valid_ids += 1;

                // Avoid "double counting" any other
                // valid ranges
                break;
            }
        }
    }

    println!("Valid Ids: {valid_ids}");

    Ok(())
}
