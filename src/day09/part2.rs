use std::collections::HashSet;

pub fn solve(input: &Vec<String>) -> String {
    let mut operations: Vec<(i64, i64)> = vec![];
    input.into_iter().for_each(|line| {
        let instruction = line.split(" ").collect::<Vec<_>>();
        if instruction[0] == "U" {
            operations.push((0, -1 * instruction[1].parse::<i64>().unwrap()));
        } else if instruction[0] == "R" {
            operations.push((instruction[1].parse::<i64>().unwrap(), 0));
        } else if instruction[0] == "D" {
            operations.push((0, instruction[1].parse::<i64>().unwrap()));
        } else {
            operations.push((-1 * instruction[1].parse::<i64>().unwrap(), 0));
        }
    });

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert((0, 0));
    let mut knots = [[0i64; 2]; 10];

    for (dx, dy) in operations {
        for _ in 1..=dx.abs() {
            knots[0][0] += dx.signum();
            for i in 1..10usize {
                find_tail_position(&mut knots, i)
            }
            visited.insert((knots[9][0], knots[9][1]));
        }
        for _ in 1..=dy.abs() {
            knots[0][1] += dy.signum();
            for i in 1..10usize {
                find_tail_position(&mut knots, i)
            }
            visited.insert((knots[9][0], knots[9][1]));
        }
    }

    visited.len().to_string()
}

fn find_tail_position(knots: &mut [[i64; 2]; 10], i: usize) {
    let x_diff = knots[i - 1][0] - knots[i][0];
    let y_diff = knots[i - 1][1] - knots[i][1];
    if x_diff == 0 && y_diff.abs() > 1 {
        knots[i][1] += y_diff.signum();
    } else if x_diff.abs() > 1 && y_diff.abs() == 0 {
        knots[i][0] += x_diff.signum();
    } else if (x_diff.abs() > 1) || (y_diff.abs() > 1) {
        knots[i][0] += x_diff.signum();
        knots[i][1] += y_diff.signum();
    }
}
