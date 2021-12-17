use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input = HashMap::new();
    contents.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| {
            input.insert((x as isize, y as isize), c - b'0');
        });
    });

    let mut input2 = input.clone();
    let part1: usize = (0..100).map(|_| step(&mut input).len()).sum();

    let mut part2 = 1;
    while step(&mut input2).len() != input2.len() {
        part2 += 1
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

const DIRECTION: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn step(map: &mut HashMap<(isize, isize), u8>) -> HashSet<(isize, isize)> {
    map.values_mut().for_each(|v| *v += 1);
    let mut queue: VecDeque<_> = map
        .iter()
        .filter_map(|(k, v)| if v > &9 { Some(*k) } else { None })
        .collect();

    let mut result = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        if !result.contains(&pos) {
            result.insert(pos);
            DIRECTION.iter().for_each(|dir| {
                let next = (pos.0 + dir.0, pos.1 + dir.1);
                if let Some(v) = map.get_mut(&next) {
                    *v += 1;
                    if *v > 9 {
                        queue.push_back(next)
                    }
                }
            });
        }
    }

    map.values_mut().for_each(|v| {
        if *v > 9 {
            *v = 0;
        }
    });
    result
}
