pub fn solve(input: &Vec<String>) -> String {
    let mut x: i64 = 1;
    let mut cycle: usize = 0;
    let mut display = [["."; 40]; 6];
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
        for _ in 0..dcycle {
            let position = (cycle as i64) % 40;
            if (position >= x - 1) && (position <= x + 1) {
                display[cycle / 40][cycle % 40] = "#";
            }
            cycle += 1;
        }
        x += dx;
    });

    let mut res = "\n".to_string();
    for i in 0..6 {
        for j in 0..40 {
            res.push_str(display[i][j]);
        }
        res.push_str("\n");
    }
    res
}
