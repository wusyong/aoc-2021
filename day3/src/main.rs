use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let numbers: Vec<_> = contents
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();

    let p1 = part1(&numbers);
    let p2 = part2(&numbers);
    println!("part1: {}\npart2: {}", p1, p2);
    Ok(())
}

fn part1(numbers: &[usize]) -> usize {
    let gamma: usize = (0..12)
        .map(|bit| most_common_bit(numbers, bit) << bit)
        .sum();
    gamma * (!gamma & 0xfff)
}

fn part2(numbers: &[usize]) -> usize {
    let mut o2 = numbers.to_vec();
    (0..12).rev().for_each(|bit| {
        if o2.len() > 1 {
            let o2_bit = most_common_bit(&o2, bit) ^ 1;
            o2.retain(|n| (n >> bit) & 1 == o2_bit);
        }
    });

    let mut co2 = numbers.to_vec();
    (0..12).rev().for_each(|bit| {
        if co2.len() > 1 {
            let co2_bit = most_common_bit(&co2, bit) ^ 0;
            co2.retain(|n| (n >> bit) & 1 == co2_bit);
        }
    });

    o2[0] * co2[0]
}

fn most_common_bit(numbers: &[usize], bit: usize) -> usize {
    let mut count = [0, 0];
    numbers.iter().for_each(|n| count[(n >> bit) & 1] += 1);
    (count[1] >= count[0]) as usize
}
