use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lists: Vec<SnailFish> = contents.lines().map(|x| SnailFish::new(x)).collect();

    println!("part1: {}", part1(lists.clone()));
    println!("part2: {:?}", part2(lists));
    Ok(())
}

fn part1(lists: Vec<SnailFish>) -> usize {
    let mut lists = lists.into_iter();
    let acc = lists.next().unwrap();
    lists.fold(acc, |acc, i| acc + i).magnitude()
}

fn part2(lists: Vec<SnailFish>) -> Option<usize> {
    (0..lists.len())
        .map(|i| {
            (0..lists.len())
                .filter(|&j| j != i)
                .map(|j| (lists[i].clone() + lists[j].clone()).magnitude())
                .max()
        })
        .flatten()
        .max()
}

#[derive(Clone)]
enum SnailFish {
    Pair(Box<SnailFish>, Box<SnailFish>),
    Number(usize),
}

impl SnailFish {
    fn new(list: &str) -> SnailFish {
        if let Some(a) = list.parse::<usize>().ok() {
            SnailFish::Number(a)
        } else {
            let mut pos = 0;
            let mut pair = 0;
            for (i, c) in list.chars().enumerate() {
                match c {
                    '[' => pair += 1,
                    ']' => pair -= 1,
                    ',' if pair == 1 => pos = i,
                    _ => (),
                }
            }
            SnailFish::new(&list[1..pos]) + SnailFish::new(&list[pos + 1..list.chars().count() - 1])
        }
    }

    fn reduce(mut self) -> Self {
        while self.split() {}
        self
    }

    fn split(&mut self) -> bool {
        while let Some(_) = self.explode(0) {}
        match self {
            SnailFish::Pair(i, j) => {
                if !i.split() {
                    j.split()
                } else {
                    true
                }
            }
            SnailFish::Number(i) => {
                if *i >= 10 {
                    *self = SnailFish::Pair(
                        Box::new(SnailFish::Number(*i / 2)),
                        Box::new(SnailFish::Number(*i / 2 + *i % 2)),
                    );
                    true
                } else {
                    false
                }
            }
        }
    }

    fn add_value(&mut self, from: bool, value: usize) {
        match (self, from) {
            (SnailFish::Pair(i, _), false) => i.add_value(from, value),
            (SnailFish::Pair(_, j), true) => j.add_value(from, value),
            (SnailFish::Number(a), _) => *a += value,
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            SnailFish::Pair(i, j) => {
                if depth == 4 {
                    let left = i.explode(depth + 1)?.0;
                    let right = j.explode(depth + 1)?.0;
                    if let SnailFish::Pair(..) = self {
                        *self = SnailFish::Number(0);
                    }
                    Some((left, right))
                } else {
                    if let Some(i) = i.explode(depth + 1) {
                        j.add_value(false, i.1);
                        Some((i.0, 0))
                    } else if let Some(j) = j.explode(depth + 1) {
                        i.add_value(true, j.0);
                        Some((0, j.1))
                    } else {
                        None
                    }
                }
            }
            SnailFish::Number(i) => {
                if depth > 4 {
                    Some((*i, *i))
                } else {
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailFish::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            SnailFish::Number(a) => *a,
        }
    }
}

impl Add for SnailFish {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        SnailFish::Pair(Box::new(self), Box::new(rhs)).reduce()
    }
}
