use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let values: Vec<Value> = contents
        .lines()
        .map(|l| l.parse::<Value>().unwrap())
        .collect();

    println!("part1: {}", part1(&values));
    println!("part2: {}", part2(&values));
    Ok(())
}

fn part1(values: &[Value]) -> usize {
    let check = [2, 3, 4, 7];
    values
        .iter()
        .map(|s| s.output.iter().filter(|o| check.contains(&o.len())).count())
        .sum()
}

fn part2(values: &[Value]) -> usize {
    values
        .iter()
        .map(|s| {
            let mut num_to_segments = [None; 10];

            s.input.iter().for_each(|segments| {
                match segments.len() {
                    2 => num_to_segments[1] = Some(segments),
                    3 => num_to_segments[7] = Some(segments),
                    4 => num_to_segments[4] = Some(segments),
                    7 => num_to_segments[8] = Some(segments),
                    _ => (),
                };
            });

            s.input.iter().for_each(|segments| {
                let segment_intersection: Vec<usize> = [4, 7, 8]
                    .into_iter()
                    .map(|common| {
                        segments
                            .intersection(&num_to_segments[common].unwrap())
                            .count()
                    })
                    .collect();

                match &segment_intersection[..] {
                    &[2, 2, 5] => num_to_segments[2] = Some(segments),
                    &[3, 2, 5] => num_to_segments[5] = Some(segments),
                    &[3, 2, 6] => num_to_segments[6] = Some(segments),
                    &[3, 3, 5] => num_to_segments[3] = Some(segments),
                    &[3, 3, 6] => num_to_segments[0] = Some(segments),
                    &[4, 3, 6] => num_to_segments[9] = Some(segments),
                    &[_, _, 2] | &[_, _, 3] | &[_, _, 4] | &[_, _, 7] => (),
                    _ => unreachable!(),
                };
            });

            s.output
                .iter()
                .map(|o| {
                    num_to_segments
                        .iter()
                        .map(|i| i.unwrap())
                        .position(|i| o.len() == i.len() && o.intersection(&i).count() == i.len())
                        .unwrap()
                })
                .rev()
                .enumerate()
                .map(|(i, num)| num * 10_usize.pow(i as u32))
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug)]
struct Value {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl FromStr for Value {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split('|').map(|l| {
            l.split_whitespace()
                .map(|signal| signal.chars().collect::<HashSet<_>>())
                .collect()
        });
        Ok(Self {
            input: s.next().unwrap(),
            output: s.next().unwrap(),
        })
    }
}
