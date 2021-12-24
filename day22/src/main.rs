use std::cmp::{max, min};
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ops = contents.lines().map(|l| {
            let (on, rest) = l.split_once(' ').unwrap();
            let coord = rest.split(',').map(|s| {
                    let n = s[2..].split_once("..").unwrap();
                    [n.0.parse().unwrap(), n.1.parse().unwrap()]
                }).collect::<Vec<_>>();
            (on == "on", coord.try_into().unwrap())
        }).collect::<Vec<_>>();
    let fifties = ops.iter()
        .filter_map(|&c| intersection(c, (true, [[-50, 50], [-50, 50], [-50, 50]])))
        .collect::<Vec<_>>();
    println!("part1: {}", solve(&fifties));
    println!("part2: {}", solve(&ops));
    Ok(())
}

type Cuboid = (bool, [[i64; 2]; 3]);
fn solve(ops: &[Cuboid]) -> i64 {
    (0..ops.len()).fold(0, |acc, i| {
        if ops[i].0 {
            acc + volume(ops[i], &ops[i + 1..])
        } else {
            acc
        }
    })
}

fn volume(c: Cuboid, others: &[Cuboid]) -> i64 {
    let inner = others.iter()
        .filter_map(|&i| intersection(i, c))
        .collect::<Vec<_>>();
    (0..inner.len()).fold(
        c.1.iter().fold(1, |v, [x1, x2]| v * (x2 - x1 + 1)),
        |acc, i| acc - volume(inner[i], &inner[i + 1..]),
    )
}

fn intersection(i: Cuboid, j: Cuboid) -> Option<Cuboid> {
    let xr = range(i.1[0], j.1[0])?;
    let yr = range(i.1[1], j.1[1])?;
    let zr = range(i.1[2], j.1[2])?;
    Some((i.0, [xr, yr, zr]))
}

fn range([i, j]: [i64; 2], [low, high]: [i64; 2]) -> Option<[i64; 2]> {
    if i > high || j < low {
        return None;
    }
    Some([min(max(i, low), high), min(max(j, low), high)])
}
