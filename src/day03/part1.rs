use std::collections::HashSet;

pub fn solve(input: &Vec<String>) -> String {
    input
        .into_iter()
        .fold(0_u32, |mut acc: u32, line: &String| {
            let mut values = line.chars().map(|c| {
                if c.is_uppercase() {
                    c as u32 - 38
                } else {
                    c as u32 - 96
                }
            });
            let mut seen: HashSet<u32> = HashSet::new();
            let size = values.size_hint().1.unwrap();
            for _ in 1..((size / 2) + 1) {
                seen.insert(values.next().unwrap());
            }
            for _ in 1..((size / 2) + 1) {
                let value = values.next().unwrap();
                if seen.contains(&value) {
                    acc += value;
                    break;
                }
            }
            acc
        })
        .to_string()
}
