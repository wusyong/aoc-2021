use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let input = contents.lines().map(|line| {
        let mut stack = vec![];
        for c in line.chars() {
            if let Some(i) = "([{<".chars().position(|j| j == c) {
                stack.push(")]}>".chars().nth(i).unwrap());
            } else if stack.pop().unwrap() != c {
                return Err(match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                });
            }
        }
        Ok(stack
            .iter()
            .rev()
            .map(|c| ")]}>".chars().position(|x| x == *c).unwrap() + 1)
            .fold(0, |a, b| a * 5 + b))
    });

    let part1: usize = input.clone().filter_map(|r| r.err()).sum();

    let mut v = input.filter_map(|r| r.ok()).collect::<Vec<_>>();
    v.sort();
    let part2 = v[v.len() / 2];

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
