use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let numbers = contents
        .trim()
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    println!("part1: {:?}", part1(&numbers));
    println!("part2: {:?}", part2(&numbers));
    Ok(())
}

fn part1(numbers: &[isize]) -> Option<isize> {
    numbers
        .iter()
        .map(|i| numbers.iter().map(|n| (n - i).abs()).sum())
        .min()
}

fn part2(numbers: &[isize]) -> Option<isize> {
    let min = *numbers.iter().min()?;
    let max = *numbers.iter().max()?;
    (min..=max)
        .map(|i| {
            numbers
                .iter()
                .map(|n| ((n - i).abs() + 1) * (n - i).abs() / 2)
                .sum()
        })
        .min()
}
