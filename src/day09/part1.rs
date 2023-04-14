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
    let mut head = [0, 0];
    let mut tail = [0, 0];

    for (dx, dy) in operations {
        for _ in 1..=dx.abs() {
            head[0] += dx.signum() * 1;
            find_tail_position(&mut head, &mut tail);
            visited.insert((tail[0], tail[1]));
        }
        for _ in 1..=dy.abs() {
            head[1] += dy.signum() * 1;
            find_tail_position(&mut head, &mut tail);
            visited.insert((tail[0], tail[1]));
        }
    }

    visited.len().to_string()
}

fn find_tail_position(head: &mut [i64], tail: &mut [i64]) {
    let x_diff = head[0] - tail[0];
    let y_diff = head[1] - tail[1];
    if x_diff == 0 && y_diff.abs() > 1 {
        tail[1] += y_diff.signum();
    } else if x_diff.abs() > 1 && y_diff.abs() == 0 {
        tail[0] += x_diff.signum();
    } else if (x_diff.abs() > 1) || (y_diff.abs() > 1) {
        tail[0] += x_diff.signum();
        tail[1] += y_diff.signum();
    }
}
