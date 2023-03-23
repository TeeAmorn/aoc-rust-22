use std::collections::HashSet;

// Use of "array_chunks()" method requires nightly
// To run this method, do "cargo +nightly run -- 3 2"
pub fn solve(input: &Vec<String>) -> String {
    input
        .into_iter()
        .array_chunks::<3>()
        .fold(0_u32, |mut acc: u32, [a, b, c]| {
            let b_chars: HashSet<u32> = b
                .chars()
                .map(|ch| {
                    if ch.is_uppercase() {
                        ch as u32 - 38
                    } else {
                        ch as u32 - 96
                    }
                })
                .collect::<HashSet<u32>>();

            let c_chars: HashSet<u32> = c
                .chars()
                .map(|ch| {
                    if ch.is_uppercase() {
                        ch as u32 - 38
                    } else {
                        ch as u32 - 96
                    }
                })
                .collect::<HashSet<u32>>();

            let common_char = a
                .chars()
                .map(|ch| {
                    if ch.is_uppercase() {
                        ch as u32 - 38
                    } else {
                        ch as u32 - 96
                    }
                })
                .find(|a_char| b_chars.contains(a_char) && c_chars.contains(a_char))
                .unwrap();
            acc += common_char;
            acc
        })
        .to_string()
}
