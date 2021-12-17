use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let numbers = contents.trim().split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

     println!("part1: {}", simulate(&numbers, 80));
     println!("part2: {}", simulate(&numbers, 256));
    Ok(())
}

fn simulate(inputs: &[usize], days: usize) -> usize {
    let mut counts = [0; 9];
    inputs.iter().for_each(|&i| counts[i] += 1);
    (0..days).for_each(|_| {
        let new_fish = counts[0];
        (0..8).for_each(|i| counts[i] = counts[i + 1]);
        counts[6] += new_fish;
        counts[8] = new_fish;
    });
    counts.into_iter().sum()
}
