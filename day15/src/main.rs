use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cavern = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let larger = (0..(cavern.len() * 5))
        .map(|x| {
            (0..(cavern[0].len() * 5))
                .map(|y| {
                    let level_x = (x / cavern.len()) as i32;
                    let level_y = (y / cavern[0].len()) as i32;
                    let mut cost = cavern[x % cavern.len()][y % cavern[0].len()] + level_x + level_y;
                    if cost > 9 {
                        cost -= 9
                    }
                    cost
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("part1: {:?}", shortes_path(&cavern));
    println!("part1: {:?}", shortes_path(&larger));
    Ok(())
}

fn shortes_path(cavern: &[Vec<i32>]) -> Option<i32> {
    let goal = (cavern.len() - 1, cavern[0].len() - 1);
    let mut dist = vec![vec![i32::MAX; cavern[0].len()]; cavern.len()];
    let mut heap = BinaryHeap::new();
    heap.push((0, 0, 0));

    while let Some((cost, x, y)) = heap.pop() {
        let cost = -cost;
        if (x, y) == goal {
            return Some(cost);
        }
        if cost > dist[x][y] {
            continue;
        }
        for (i, j) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if let Some(level) = cavern.get(i).and_then(|row| row.get(j)) {
                let next = cost + level;
                if next < dist[i][j] {
                    // Storing negative value since it's max heap.
                    heap.push((-next, i, j));
                    dist[i][j] = next;
                }
            }
        }
    }
    None
}
