use std::{process, str::Chars};

pub fn solve(input: &Vec<String>) -> String {
    let mut input_iter = input.chunks(3);
    let mut counter: u32 = 0;
    let mut answer: u32 = 0;
    while let Some(lines) = input_iter.next() {
        counter += 1;
        let mut left_iter = lines[0].chars();
        left_iter.next();
        let left = parse(&mut left_iter);
        let mut right_iter = lines[1].chars();
        right_iter.next();
        let right = parse(&mut right_iter);
        println!("{counter}");
        println!("{:?}", left);
        println!("{:?}", right);
        if let PacketCmp::Less = compare(&left, &right) {
            println!("Right: {}", counter);
            answer += counter;
        }
    }
    answer.to_string()
}

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

fn parse(input: &mut Chars) -> Packet {
    let mut list = vec![];
    let mut curr = "".to_string();

    loop {
        let c = input.next().unwrap();
        match c {
            '[' => list.push(parse(input)),
            ']' | ',' => {
                if !curr.is_empty() {
                    list.push(Packet::Integer(curr.parse::<u32>().unwrap()));
                    curr = "".to_string();
                }
                if c == ']' {
                    break;
                }
            }
            _ => curr.push(c),
        }
    }

    Packet::List(list)
}

#[derive(Debug)]
enum PacketCmp {
    Less,
    Equal,
    Greater,
    Undefined,
}

fn compare(left: &Packet, right: &Packet) -> PacketCmp {
    if let (Packet::Integer(n), Packet::Integer(m)) = (left, right) {
        if n < m {
            return PacketCmp::Less;
        } else if m < n {
            return PacketCmp::Greater;
        } else {
            return PacketCmp::Equal;
        }
    } else if let (Packet::List(n), Packet::List(m)) = (left, right) {
        let mut n_iter = n.iter();
        let mut m_iter = m.iter();
        loop {
            match (n_iter.next(), m_iter.next()) {
                (Some(a), Some(b)) => match compare(a, b) {
                    PacketCmp::Less => return PacketCmp::Less,
                    PacketCmp::Greater => return PacketCmp::Greater,
                    PacketCmp::Equal => continue,
                    PacketCmp::Undefined => {
                        process::exit(1);
                    }
                },
                (Some(_), None) => {
                    return PacketCmp::Greater;
                }
                (None, Some(_)) => {
                    return PacketCmp::Less;
                }
                (None, None) => {
                    return PacketCmp::Equal;
                }
            }
        }
    } else if let (Packet::Integer(n), Packet::List(_)) = (left, right) {
        return compare(&Packet::List(vec![Packet::Integer(n.clone())]), right);
    } else if let (Packet::List(_), Packet::Integer(m)) = (left, right) {
        return compare(left, &Packet::List(vec![Packet::Integer(m.clone())]));
    }

    PacketCmp::Undefined
}
