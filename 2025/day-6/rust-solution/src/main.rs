use std::error;
use std::fs;

const FILE_PATH: &str = "../problem.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;

    let total = part_two(&content);

    println!("Total: {total}");

    Ok(())
}

fn part_two(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap();

    // Build a padded grid of chars
    let mut grid: Vec<Vec<char>> = Vec::new();
    for &line in &lines {
        let mut row: Vec<char> = line.chars().collect();
        row.resize(width, ' ');
        grid.push(row);
    }

    let mut total = 0;
    let mut col = 0;

    while col < width {
        // Skip separator columns (all spaces)
        if grid.iter().all(|row| row[col] == ' ') {
            col += 1;
            continue;
        }

        // We are at the start of a problem: find its column range [start, end)
        let start = col;
        while col < width && !grid.iter().all(|row| row[col] == ' ') {
            col += 1;
        }
        let end = col;

        // Find the operator for this problem (in the bottom row somewhere in [start, end))
        let mut op = ' ';
        for c in start..end {
            let ch = grid[height - 1][c];
            if ch == '+' || ch == '*' {
                op = ch;
                break;
            }
        }
        if op != '+' && op != '*' {
            panic!("No operator found in problem starting at column {start}");
        }

        // Collect numbers from each column in left-to-right order
        let mut values: Vec<i64> = Vec::new();
        for c in start..end {
            let mut digits = String::new();
            for r in 0..height - 1 {
                let ch = grid[r][c];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                let value = digits.parse::<i64>().unwrap();
                values.push(value);
            }
        }

        // Reverse for right-to-left reading
        values.reverse();

        // Debug (optional)
        for v in &values {
            println!("Value {v}");
        }
        println!("Operator {op}\n");

        // Solve this problem
        let mut result = values[0];
        for &v in &values[1..] {
            match op {
                '+' => result += v,
                '*' => result *= v,
                _ => unreachable!(),
            }
        }

        total += result;
    }

    total
}

fn _part_one() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(FILE_PATH)?;

    // Create a vector for totals, init to 0s the length
    // of different problems
    //
    // IMPORTANT: Split by whitespace, since there is not guarantee
    // of how much space there is between numbers/operands
    let num_problems = content.lines().next().unwrap().split_whitespace().count();
    let operands: Vec<&str> = content.lines().last().unwrap().split_whitespace().collect();

    let mut results = vec![0; num_problems];

    for (row_idx, row) in content.lines().enumerate() {
        for (idx, val) in row.split_whitespace().enumerate() {
            let parse_result = val.parse();
            let val_num = match parse_result {
                Ok(number) => number,
                Err(_) => {
                    continue;
                }
            };

            if row_idx == 0 {
                // First row, just insert the values
                results[idx] = val_num;
                continue;
            }

            if operands[idx] == "+" {
                results[idx] += val_num;
            } else {
                results[idx] *= val_num;
            }
        }
    }

    let total: i64 = results.iter().sum();

    println!("Total: {total}");

    Ok(())
}
