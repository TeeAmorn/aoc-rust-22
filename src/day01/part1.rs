pub fn solve(input: &Vec<String>) -> String {
    input
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
        .max()
        .unwrap()
        .to_string()
}
