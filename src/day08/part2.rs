pub fn solve(input: &Vec<String>) -> String {
    let rows = input.len();
    let columns = input[0].len();
    let mut result: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    // From left to right
    for (i, row) in input.iter().enumerate() {
        let mut trees: Vec<(usize, u32)> = vec![];
        for (j, h) in row.chars().enumerate() {
            let height = h.to_digit(10).unwrap() as u32;

            while trees.len() > 0 && trees[trees.len() - 1].1 < height {
                trees.pop();
            }

            if trees.len() > 0 {
                result[i][j] = (j as u32) - (trees[trees.len() - 1].0 as u32);
            } else {
                result[i][j] = j as u32;
            }

            trees.push((j, height));
        }
    }

    // From right to left
    for (i, row) in input.iter().enumerate() {
        let mut trees: Vec<(usize, u32)> = vec![];
        for (j, h) in row.chars().rev().enumerate() {
            let height = h.to_digit(10).unwrap() as u32;

            while trees.len() > 0 && trees[trees.len() - 1].1 < height {
                trees.pop();
            }

            if trees.len() > 0 {
                result[i][columns - 1 - j] *= (j as u32) - (trees[trees.len() - 1].0 as u32);
            } else {
                result[i][columns - 1 - j] *= j as u32;
            }

            trees.push((j, height));
        }
    }

    // Iterator for scanning vertically
    let input_iter = input
        .iter()
        .map(|line| line.chars().map(|letter| letter.to_digit(10).unwrap() as i8))
        .flatten();

    // From top to bottom
    let top_to_bottom_iter = (0..rows).map(|row_idx| input_iter.clone().skip(row_idx).step_by(columns));
    for (i, row) in top_to_bottom_iter.enumerate() {
        let mut trees: Vec<(usize, u32)> = vec![];
        for (j, height) in row.enumerate() {
            let height = height as u32;

            while trees.len() > 0 && trees[trees.len() - 1].1 < height {
                trees.pop();
            }

            if trees.len() > 0 {
                result[j][i] *= (j as u32) - (trees[trees.len() - 1].0 as u32);
            } else {
                result[j][i] *= j as u32;
            }

            trees.push((j, height));
        }
    }

    // From bottom to top
    let bottom_to_top_iter = (0..rows).map(|row_idx| input_iter.clone().rev().skip(row_idx).step_by(columns));
    for (i, row) in bottom_to_top_iter.enumerate() {
        let mut trees: Vec<(usize, u32)> = vec![];
        for (j, height) in row.enumerate() {
            let height = height as u32;

            while trees.len() > 0 && trees[trees.len() - 1].1 < height {
                trees.pop();
            }

            if trees.len() > 0 {
                result[rows - 1 - j][columns - 1 - i] *= (j as u32) - (trees[trees.len() - 1].0 as u32);
            } else {
                result[rows - 1 - j][columns - 1 - i] *= j as u32;
            }

            trees.push((j, height));
        }
    }

    result.iter().flatten().max().unwrap().to_string()
}
