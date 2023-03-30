use std::collections::HashSet;

pub fn solve(input: &Vec<String>) -> String {
    let chars = input[0].chars().collect::<Vec<_>>();
    for (pos, window) in chars.windows(14).enumerate() {
        let seen: HashSet<char> = Vec::from(window).into_iter().collect();
        if seen.len() == 14 {
            return (pos + 14).to_string();
        }
    }
    "".to_string()
}
