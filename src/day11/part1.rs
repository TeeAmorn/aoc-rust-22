#[derive(Debug)]
enum Op {
    Multiply = 0,
    Add = 1,
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(u32),
}

#[derive(Debug)]
struct Monkey {
    old_items: Vec<u32>,
    new_items: Vec<u32>,
    operation: Op,
    operand: Operand,
    divisible: u32,
    true_monkey: u32,
    false_monkey: u32,
}

pub fn solve(input: &Vec<String>) -> String {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut input_iter = input.iter();
    while let Some(_) = input_iter.next() {
        let mut line = input_iter.next().unwrap();
        let items = line.trim()[16..]
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        line = input_iter.next().unwrap();
        let operation;
        if &line.trim()[21..22] == "*" {
            operation = Op::Multiply;
        } else {
            operation = Op::Add;
        }
        let operand = match line.trim()[23..].parse::<u32>() {
            Ok(n) => Operand::Number(n),
            Err(_) => Operand::Old,
        };

        line = input_iter.next().unwrap();
        let divisible = line.trim()[19..].parse::<u32>().unwrap();

        line = input_iter.next().unwrap();
        let true_monkey = line.trim()[25..].parse::<u32>().unwrap();

        line = input_iter.next().unwrap();
        let false_monkey = line.trim()[26..].parse::<u32>().unwrap();

        monkeys.push(Monkey {
            old_items: vec![],
            new_items: items,
            operation,
            operand,
            divisible,
            true_monkey,
            false_monkey,
        });

        input_iter.next();
    }

    let mut counters = vec![0u32; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            counters[i] += monkeys[i].new_items.len() as u32;
            monkeys[i].old_items = monkeys[i].new_items.clone();
            monkeys[i].new_items = vec![];

            for item in monkeys[i].old_items.clone() {
                let operand = match monkeys[i].operand {
                    Operand::Old => item,
                    Operand::Number(n) => n,
                };
                let new_item = match monkeys[i].operation {
                    Op::Multiply => item * operand,
                    Op::Add => item + operand,
                };
                if (new_item / 3) % monkeys[i].divisible == 0 {
                    let true_monkey = monkeys[i].true_monkey as usize;
                    monkeys[true_monkey].new_items.push(new_item / 3);
                } else {
                    let false_monkey = monkeys[i].false_monkey as usize;
                    monkeys[false_monkey].new_items.push(new_item / 3);
                }
            }
        }
    }

    let mut first = 0;
    let mut second = 0;
    for n in &counters {
        if *n > first {
            second = first;
            first = *n;
        } else if *n > second {
            second = *n;
        }
    }

    (first * second).to_string()
}
