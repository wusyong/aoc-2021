use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ops: Vec<(char, i32)> = contents
        .lines()
        .map(|s| {
            let mut s = s.split_whitespace();
            (
                s.next().unwrap().chars().next().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let p1 = part1(&ops);
    let p2 = part2(&ops);
    println!("part1: {}\npart2: {}", p1, p2);
    Ok(())
}

fn part1(ops: &[(char, i32)]) -> i32 {
    let (x, y) = ops.iter().fold((0, 0), |(x, y), (op, i)| match op {
        'f' => (x + i, y),
        'd' => (x, y + i),
        'u' => (x, y - i),
        _ => unreachable!(),
    });

    x * y
}

fn part2(ops: &[(char, i32)]) -> i32 {
    let (x, y, _aim) = ops.iter().fold((0, 0, 0), |(x, y, aim), (op, i)| match op {
        'f' => (x + i, y + aim * i, aim),
        'd' => (x, y, aim + i),
        'u' => (x, y, aim - i),
        _ => unreachable!(),
    });

    x * y
}
