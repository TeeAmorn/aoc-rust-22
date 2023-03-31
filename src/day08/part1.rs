use std::collections::HashSet;

pub fn solve(input: &Vec<String>) -> String {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let columns = input.len();
    let rows = input[0].len();

    // From left to right
    for (i, row) in input.iter().enumerate() {
        let mut running_max: i8 = -1;
        for (j, h) in row.chars().enumerate() {
            let height = h.to_digit(10).unwrap() as i8;
            if height > running_max {
                seen.insert((i, j));
                running_max = height;
            }
        }
    }

    // From right to left
    for (i, row) in input.iter().enumerate() {
        let mut running_max: i8 = -1;
        for (j, h) in row.chars().rev().enumerate() {
            let m = i;
            let n = columns - 1 - j;
            let height = h.to_digit(10).unwrap() as i8;
            if height > running_max {
                seen.insert((m, n));
                running_max = height;
            }
        }
    }

    // Iterator for scanning vertically
    let input_iter = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|letter| letter.to_digit(10).unwrap() as i8)
        })
        .flatten();

    // From top to bottom
    let top_to_bottom_iter =
        (0..rows).map(|row_idx| input_iter.clone().skip(row_idx).step_by(columns));
    for (i, row) in top_to_bottom_iter.enumerate() {
        let mut running_max: i8 = -1;
        for (j, height) in row.enumerate() {
            let m = j;
            let n = i;
            if height > running_max {
                seen.insert((m, n));
                running_max = height;
            }
        }
    }

    // From bottom to top
    let bottom_to_top_iter =
        (0..rows).map(|row_idx| input_iter.clone().rev().skip(row_idx).step_by(columns));
    for (i, row) in bottom_to_top_iter.enumerate() {
        let mut running_max: i8 = -1;
        for (j, height) in row.enumerate() {
            let m = rows - 1 - j;
            let n = columns - 1 - i;
            if height > running_max {
                seen.insert((m, n));
                running_max = height;
            }
        }
    }

    seen.len().to_string()
}
