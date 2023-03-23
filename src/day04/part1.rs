pub fn solve(input: &Vec<String>) -> String {
    input
        .into_iter()
        .map(|line| {
            let sections = line
                .split(",")
                .map(|section| {
                    section
                        .split("-")
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();
            let mut first = sections[0].clone();
            let mut second = sections[1].clone();
            first.append(&mut second);
            first
        })
        .fold(0_u32, |mut acc: u32, range| {
            acc += ((range[0] <= range[2] && range[1] >= range[3])
                || (range[2] <= range[0] && range[3] >= range[1])) as u32;
            acc
        })
        .to_string()
}
