use std::collections::HashSet;

pub fn solve(input: &Vec<String>) -> String {
    let chars = input[0].chars().collect::<Vec<_>>();
    for (pos, window) in chars.windows(4).enumerate() {
        let seen: HashSet<char> = Vec::from(window).into_iter().collect();
        if seen.len() == 4 {
            return (pos + 4).to_string();
        }
    }
    "".to_string()
}