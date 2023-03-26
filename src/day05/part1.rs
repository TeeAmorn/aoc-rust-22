pub fn solve(input: &Vec<String>) -> String {
    let mut input_iter = input.into_iter();

    // Stacks construction
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in &mut input_iter {
        if line.trim().chars().nth(0).unwrap() != '1' {
            let chars = line.chars().collect::<Vec<_>>();
            let items = chars.chunks(4).map(|item| item[1]);
            if stacks.len() == 0 {
                for _ in 0..items.len() {
                    stacks.push(Vec::new());
                }
            }
            for (pos, item) in items.enumerate() {
                stacks[pos].push(item);
            }
        } else {
            break;
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
        while let Some(item) = stack.last() {
            if *item == ' ' {
                stack.pop();
            } else {
                break;
            }
        }
    }

    // Moving operations
    input_iter.next();
    for line in input_iter {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let amount = tokens[1].parse::<usize>().unwrap();
        let src = tokens[3].parse::<usize>().unwrap() - 1;
        let dst = tokens[5].parse::<usize>().unwrap() - 1;
        for _ in 0..amount {
            let item = stacks[src].pop().unwrap();
            stacks[dst].push(item);
        }
    }

    stacks.into_iter().fold(String::new(), |mut acc, stack| {
        if let Some(item) = stack.last() {
            acc += &String::from(*item);
        }
        acc
    })
}
