use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let numbers: Vec<usize> = contents.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    let p1 = numbers.windows(2).filter(|i| i[0] < i[1]).count();
    let p2 = numbers.windows(4).filter(|i| i[0] < i[3]).count();
    println!("part1: {}\npart2: {}", p1, p2);
    Ok(())
}
