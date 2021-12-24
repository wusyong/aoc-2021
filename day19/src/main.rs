use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input: Vec<Vec<(i32, i32 ,i32)>> = contents
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    let mut l = l.split(",");
                    (
                        l.next().unwrap().parse().unwrap(),
                        l.next().unwrap().parse().unwrap(),
                        l.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut detected = input.pop().unwrap();
    let mut dists = vec![(0, 0, 0)];
    while let Some((distance, merged_scan)) = locate(&mut input, &detected) {
        detected = merged_scan;
        dists.push(distance);
    }

    let part1 = detected.len();
    let mut part2 = 0;
    for &(x1, y1, z1) in dists.iter() {
        for &(x2, y2, z2) in dists.iter() {
            part2 = part2.max((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs());
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn locate(
    input: &mut Vec<Vec<(i32, i32, i32)>>,
    detected: &[(i32, i32, i32)],
) -> Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> {
    (0..input.len()).find_map(|i| {
        rotation(&input[i]).iter().find_map(|j| {
            merge(&detected, &j).map(|x| {
                input.swap_remove(i);
                x
            })
        })
    })
}

fn merge(i: &[(i32, i32, i32)], j: &[(i32, i32, i32)]) -> Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> {
    let mut set = i.iter().copied().collect::<HashSet<_>>();
    for (x1, y1, z1) in i.iter() {
        for (x2, y2, z2) in j.iter() {
            let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
            let translated = j.iter().map(|(x3, y3, z3)| (x3 + dx, y3 + dy, z3 + dz));
            if translated.clone().filter(|v| set.contains(v)).count() >= 12 {
                set.extend(translated);
                return Some(((dx, dy, dz), set.into_iter().collect()));
            }
        }
    }
    None
}

fn rotation(i: &[(i32, i32, i32)]) -> [Vec<(i32, i32, i32)>; 24] {
    [
        i.to_vec(),
        i.iter().map(|&(a, b, c)| (b, c, a)).collect(),
        i.iter().map(|&(a, b, c)| (c, a, b)).collect(),
        i.iter().map(|&(a, b, c)| (c, b, -a)).collect(),
        i.iter().map(|&(a, b, c)| (b, a, -c)).collect(),
        i.iter().map(|&(a, b, c)| (a, c, -b)).collect(),

        i.iter().map(|&(a, b, c)| (a, -b, -c)).collect(),
        i.iter().map(|&(a, b, c)| (b, -c, -a)).collect(),
        i.iter().map(|&(a, b, c)| (c, -a, -b)).collect(),
        i.iter().map(|&(a, b, c)| (c, -b, a)).collect(),
        i.iter().map(|&(a, b, c)| (b, -a, c)).collect(),
        i.iter().map(|&(a, b, c)| (a, -c, b)).collect(),

        i.iter().map(|&(a, b, c)| (-a, b, -c)).collect(),
        i.iter().map(|&(a, b, c)| (-b, c, -a)).collect(),
        i.iter().map(|&(a, b, c)| (-c, a, -b)).collect(),
        i.iter().map(|&(a, b, c)| (-c, b, a)).collect(),
        i.iter().map(|&(a, b, c)| (-b, a, c)).collect(),
        i.iter().map(|&(a, b, c)| (-a, c, b)).collect(),

        i.iter().map(|&(a, b, c)| (-a, -b, c)).collect(),
        i.iter().map(|&(a, b, c)| (-b, -c, a)).collect(),
        i.iter().map(|&(a, b, c)| (-c, -a, b)).collect(),
        i.iter().map(|&(a, b, c)| (-c, -b, -a)).collect(),
        i.iter().map(|&(a, b, c)| (-b, -a, -c)).collect(),
        i.iter().map(|&(a, b, c)| (-a, -c, -b)).collect(),
    ]
}
