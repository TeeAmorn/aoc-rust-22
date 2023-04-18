use std::collections::{HashSet, VecDeque};

pub fn solve(input: &Vec<String>) -> String {
    let mut row: i8 = 0;
    let mut col: i8 = 0;
    let mut start: (i8, i8, u32) = (0, 0, 0);
    let mut end: (i8, i8) = (0, 0);
    let map = input
        .iter()
        .map(|line| {
            let res = line
                .chars()
                .map(|c| {
                    let n = match c {
                        'S' => {
                            start = (row, col, 0);
                            0
                        }
                        'E' => {
                            end = (row, col);
                            25
                        }
                        _ => c as i8 - 97,
                    };
                    col += 1;
                    n
                })
                .collect::<Vec<_>>();
            row += 1;
            col = 0;
            res
        })
        .collect::<Vec<_>>();

    let m = map.len() as i8;
    let n = map[0].len() as i8;

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    seen.insert((start.0, start.1));

    while let Some(pos) = queue.pop_front() {
        let (i, j, steps) = pos;

        if pos.0 == end.0 && pos.1 == end.1 {
            return steps.to_string();
        }

        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let newi = i + di;
            let newj = j + dj;

            if !(newi >= 0 && newi < m && newj >= 0 && newj < n) {
                continue;
            }

            if seen.contains(&(newi, newj)) {
                continue;
            }

            if map[newi as usize][newj as usize] - map[i as usize][j as usize] > 1 {
                continue;
            }

            queue.push_back((newi, newj, steps + 1));
            seen.insert((newi, newj));
        }
    }

    "Should not reach here!".to_string()
}
