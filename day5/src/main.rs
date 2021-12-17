use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let numbers = contents.lines()
        .map(|l| l.split(" -> ")
           .map(|xy| xy.split(","))
           .flatten()
           .map(|i| i.parse().unwrap())
           .collect::<Points>())
        .collect::<Vec<_>>();

    println!("part1: {}", overlap(numbers.clone().into_iter().filter(|Points { x1, y1, x2, y2 }| x1 == x2 || y1 == y2)));
    println!("part2: {}", overlap(numbers.clone().into_iter()));
    Ok(())
}

#[derive(Clone)]
struct Points {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl FromIterator<isize> for Points {
    fn from_iter<I: IntoIterator<Item=isize>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self {
            x1: iter.next().unwrap(),
            y1: iter.next().unwrap(),
            x2: iter.next().unwrap(),
            y2: iter.next().unwrap(),
        }
    }
}

fn overlap(points: impl Iterator<Item=Points>) -> usize {
  let mut count = HashMap::new();
  points.for_each(|Points { mut x1, mut y1, x2, y2 }| {
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    while (x1, y1) != (x2 + dx, y2 + dy) {
      count.entry((x1, y1)).and_modify(|e| { *e += 1 }).or_insert(1);
      x1 += dx;
      y1 += dy;
    }
  });
  count.values().filter(|&&n| n > 1).count()
}

