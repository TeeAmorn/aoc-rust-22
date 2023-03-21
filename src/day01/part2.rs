use std::collections::BinaryHeap;

pub fn solve(input: &Vec<String>) -> String {
    (input
        .clone()
        .into_iter()
        .fold(Vec::new(), |mut acc, c| {
            if acc.is_empty() || c.is_empty() {
                acc.push(0_u32);
            } else {
                *acc.last_mut().unwrap() += c.parse::<u32>().unwrap();
            }
            acc
        })
        .into_iter()
        .fold(BinaryHeap::new(), |mut acc, c| {
            acc.push(i64::from(c) * -1);
            if acc.len() == 4 {
                acc.pop();
            }
            acc
        })
        .into_vec()
        .into_iter()
        .sum::<i64>()
        * -1)
        .to_string()
}
