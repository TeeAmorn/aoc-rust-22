use std::{cmp::max, collections::HashSet};

pub fn solve(input: &Vec<String>) -> String {
    // populate matrix for fixed objects
    let mut seen = HashSet::new();
    let mut max_depth = 0;

    for line in input.into_iter() {
        let mut line_iter = line.split(" -> ");

        let start = line_iter
            .next()
            .unwrap()
            .split(",")
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut start_x = start[0];
        let mut start_y = start[1];

        while let Some(part) = line_iter.next() {
            let coordinate = part
                .split(",")
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut end_x = coordinate[0];
            let mut end_y = coordinate[1];

            if (end_x < start_x) || (end_y < start_y) {
                let tmp_x = start_x;
                start_x = end_x;
                end_x = tmp_x;
                let tmp_y = start_y;
                start_y = end_y;
                end_y = tmp_y;
            }

            max_depth = max(max_depth, end_y);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    seen.insert((x, y));
                }
            }

            start_x = coordinate[0];
            start_y = coordinate[1];
        }
    }

    // start simulating sand drops
    let mut count: u32 = 0;
    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y > max_depth {
                return count.to_string();
            }
            if !seen.contains(&(x, y + 1)) {
                y += 1;
            } else if !seen.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !seen.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                seen.insert((x, y));
                break;
            }
        }
        count += 1;
    }
}
