use std::{cmp::Ordering, process, str::Chars};

pub fn solve(input: &Vec<String>) -> String {
    let mut input_iter = input.chunks(3);
    let mut items = vec![];
    while let Some(lines) = input_iter.next() {
        let mut left_iter = lines[0].chars();
        left_iter.next();
        let left = parse(&mut left_iter);
        items.push(left);
        let mut right_iter = lines[1].chars();
        right_iter.next();
        let right = parse(&mut right_iter);
        items.push(right);
    }

    let divider_a = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_b = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    items.push(divider_a);
    items.push(divider_b);
    items.sort_by(compare);

    let divider_a = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_b = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    let mut ans = 1;
    for i in 0..items.len() {
        if items[i] == divider_a || items[i] == divider_b {
            ans *= i + 1;
        }
    }
    ans.to_string()
}

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => {
                return a == b;
            }
            (Packet::List(a), Packet::List(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (x, y) in a.iter().zip(b.iter()) {
                    if !x.eq(y) {
                        return false;
                    }
                }
                return true;
            }
            _ => return false,
        }
    }
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

fn compare(left: &Packet, right: &Packet) -> Ordering {
    if let (Packet::Integer(n), Packet::Integer(m)) = (left, right) {
        if n < m {
            return Ordering::Less;
        } else if m < n {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    } else if let (Packet::List(n), Packet::List(m)) = (left, right) {
        let mut n_iter = n.iter();
        let mut m_iter = m.iter();
        loop {
            match (n_iter.next(), m_iter.next()) {
                (Some(a), Some(b)) => match compare(a, b) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => continue,
                },
                (Some(_), None) => {
                    return Ordering::Greater;
                }
                (None, Some(_)) => {
                    return Ordering::Less;
                }
                (None, None) => {
                    return Ordering::Equal;
                }
            }
        }
    } else if let (Packet::Integer(n), Packet::List(_)) = (left, right) {
        return compare(&Packet::List(vec![Packet::Integer(n.clone())]), right);
    } else if let (Packet::List(_), Packet::Integer(m)) = (left, right) {
        return compare(left, &Packet::List(vec![Packet::Integer(m.clone())]));
    }

    process::exit(-1);
}
