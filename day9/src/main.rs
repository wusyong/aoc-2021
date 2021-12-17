use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let grid: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| l.chars().map(|s| s.to_digit(10).unwrap()).collect())
        .collect();

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
    Ok(())
}

pub fn part1(grid: &[Vec<u32>]) -> u32 {
    let mut result = 0;
    (0..grid.len()).for_each(|x| {
        (0..grid[0].len()).for_each(|y| {
            let current = grid[x][y];
            let up = *(grid[x].get(y + 1).unwrap_or(&10));
            let down = *(grid[x].get(y - 1).unwrap_or(&10));
            let left = *(grid.get(x - 1).unwrap_or(&vec![]).get(y).unwrap_or(&10));
            let right = *(grid.get(x + 1).unwrap_or(&vec![]).get(y).unwrap_or(&10));
            if [up, down, left, right].iter().min().unwrap() > &current {
                result += current + 1;
            }
        });
    });
    result
}

pub fn part2(grid: &[Vec<u32>]) -> u32 {
    let mut grid: Vec<Vec<bool>> = grid
        .iter()
        .map(|n| n.iter().map(|i| *i == 9).collect())
        .collect();
    let mut basins = vec![];
    (0..grid.len()).for_each(|x| {
        (0..grid[0].len()).for_each(|y| {
            basins.push(basin(x, y, &mut grid));
        });
    });
    basins.sort();
    basins.iter().rev().take(3).product()
}

fn basin(x: usize, y: usize, grid: &mut [Vec<bool>]) -> u32 {
    let (mut empty_x, mut empty_y) = (vec![], true);
    let current = grid
        .get_mut(x)
        .unwrap_or(&mut empty_x)
        .get_mut(y)
        .unwrap_or(&mut empty_y);
    if *current {
        0
    } else {
        *current = true;
        1 + basin(x + 1, y, grid)
            + basin(x - 1, y, grid)
            + basin(x, y + 1, grid)
            + basin(x, y - 1, grid)
    }
}
