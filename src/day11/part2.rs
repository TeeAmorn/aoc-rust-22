#[derive(Debug)]
enum Op {
    Multiply = 0,
    Add = 1,
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(u64),
}

#[derive(Debug)]
struct Monkey {
    old_items: Vec<u64>,
    new_items: Vec<u64>,
    operation: Op,
    operand: Operand,
    divisible: u64,
    true_monkey: u64,
    false_monkey: u64,
}

pub fn solve(input: &Vec<String>) -> String {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut input_iter = input.iter();
    while let Some(_) = input_iter.next() {
        let mut line = input_iter.next().unwrap();
        let items = line.trim()[16..]
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        line = input_iter.next().unwrap();
        let operation;
        if &line.trim()[21..22] == "*" {
            operation = Op::Multiply;
        } else {
            operation = Op::Add;
        }
        let operand = match line.trim()[23..].parse::<u64>() {
            Ok(n) => Operand::Number(n),
            Err(_) => Operand::Old,
        };

        line = input_iter.next().unwrap();
        let divisible = line.trim()[19..].parse::<u64>().unwrap();

        line = input_iter.next().unwrap();
        let true_monkey = line.trim()[25..].parse::<u64>().unwrap();

        line = input_iter.next().unwrap();
        let false_monkey = line.trim()[26..].parse::<u64>().unwrap();

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

    let mut gcd = 1;
    for monkey in monkeys.iter() {
        gcd *= monkey.divisible;
    }

    println!("{gcd}");

    let mut counters  = vec![0u64; monkeys.len()];
    
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            counters[i] += monkeys[i].new_items.len() as u64;
            monkeys[i].old_items = monkeys[i].new_items.clone();
            monkeys[i].new_items = vec![];

            for item in monkeys[i].old_items.clone() {
                let operand = match monkeys[i].operand {
                    Operand::Old => item,
                    Operand::Number(n) => n,
                };
                let new_item = match monkeys[i].operation {
                    Op::Multiply => ((item % gcd) * (operand % gcd)) % gcd,
                    Op::Add => ((item % gcd) + (operand % gcd)) % gcd,
                };
                if (new_item) % monkeys[i].divisible == 0 {
                    let true_monkey = monkeys[i].true_monkey as usize;
                    monkeys[true_monkey].new_items.push(new_item);
                } else {
                    let false_monkey = monkeys[i].false_monkey as usize;
                    monkeys[false_monkey].new_items.push(new_item);
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
