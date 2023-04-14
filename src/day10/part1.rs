pub fn solve(input: &Vec<String>) -> String {
    let mut threshold = 20;
    let mut cycle = 0;
    let mut x = 1;
    let mut res = 0;
    input.into_iter().for_each(|line| {
        let command = line.split(" ").collect::<Vec<_>>();
        let dcycle;
        let dx;
        if command[0] == "addx" {
            dcycle = 2;
            dx = command[1].parse::<i64>().unwrap();
        } else {
            dcycle = 1;
            dx = 0;
        }

        if cycle + dcycle >= threshold {
            res += x * threshold;
            threshold += 40;
        }
        x += dx;
        cycle += dcycle;
    });

    res.to_string()
}
