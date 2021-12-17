use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut input = contents
        .chars()
        .filter_map(|c| c.to_digit(16))
        .flat_map(|x| [x & 8 != 0, x & 4 != 0, x & 2 != 0, x & 1 != 0]);
    let packet = parse(&mut input).unwrap();
    println!("part1: {:?}", part1(&packet));
    println!("part2: {:?}", part2(&packet));
    Ok(())
}

enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        op: usize,
        children: Vec<Packet>,
    },
}

fn parse(iter: &mut impl Iterator<Item = bool>) -> Option<Packet> {
    let version = parse_int(iter, 3)?;
    let op = parse_int(iter, 3)?;
    if op == 4 {
        let mut value = 0;
        loop {
            let bit = iter.next()?;
            value = 16 * value + parse_int(iter, 4)?;
            if !bit {
                break;
            }
        }
        Some(Packet::Literal { version, value })
    } else {
        let children = if iter.next()? {
            (0..parse_int(iter, 11)?)
                .map(|_| parse(iter))
                .collect::<Option<_>>()
        } else {
            let n = parse_int(iter, 15)?;
            let slice: Vec<bool> = iter.take(n).collect();
            if slice.len() == n {
                let mut iter = slice.into_iter();
                let children = std::iter::from_fn(|| parse(&mut iter)).collect();
                if iter.next().is_none() {
                    Some(children)
                } else {
                    None
                }
            } else {
                None
            }
        }?;
        Some(Packet::Operator {
            version,
            op,
            children,
        })
    }
}

fn parse_int(iter: &mut impl Iterator<Item = bool>, n: usize) -> Option<usize> {
    let mut i = 0;
    for _ in 0..n {
        i = 2 * i + iter.next()? as usize;
    }
    Some(i)
}

fn part1(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Operator {
            version, children, ..
        } => *version + children.iter().map(part1).sum::<usize>(),
    }
}

fn part2(packet: &Packet) -> Option<usize> {
    match packet {
        Packet::Literal { value, .. } => Some(*value),
        Packet::Operator { op, children, .. } => match op {
            0 => Some(children.iter().filter_map(part2).sum()),
            1 => Some(children.iter().filter_map(part2).product()),
            2 => children.iter().filter_map(part2).min(),
            3 => children.iter().filter_map(part2).max(),
            5 => Some((part2(&children[0])? > part2(&children[1])?) as usize),
            6 => Some((part2(&children[0])? < part2(&children[1])?) as usize),
            7 => Some((part2(&children[0])? == part2(&children[1])?) as usize),
            _ => None,
        },
    }
}
