use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut inputs = contents.split("\n\n");
    let numbers: Vec<usize> = inputs.next().unwrap()
        .split(",")
        .filter_map(|i| i.parse::<usize>().ok())
        .collect();
    let boards: Vec<Vec<Vec<usize>>> = inputs
        .map(|board| {
            board.lines().map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse().ok())
                    .collect()
            }).collect()
        }).collect();

    let p1 = part1(&numbers, &boards);
    let p2 = part2(&numbers, boards);
    println!("part1: {:?}\npart2: {:?}", p1, p2);
    Ok(())
}

fn sum(numbers: &[usize], b: &[Vec<usize>]) -> usize {
  b.iter().flatten().filter(|x| !numbers.contains(x)).sum()
}

fn check(numbers: &[usize], board: &[Vec<usize>]) -> Option<usize> {
    (0..5).find_map(|i| {
        if (0..5).all(|j| numbers.contains(&board[i][j])) {
            Some(sum(numbers, board) * numbers.last()?)
        } else if (0..5).all(|j| numbers.contains(&board[j][i])) {
            Some(sum(numbers, board) * numbers.last()?)
        } else {
            None
        }
    })
}

fn part1(numbers: &[usize], boards: &[Vec<Vec<usize>>]) -> Option<usize> {
    (5..numbers.len()).find_map(|i| {
        boards.iter().find_map(|b| {
            check(&numbers[..i], b)
        })
    })
}

fn part2(numbers: &[usize], mut boards: Vec<Vec<Vec<usize>>>) -> Option<usize> {
    let mut last = None;
    (5..numbers.len()).for_each(|i| {
        boards.retain(|b| {
            check(&numbers[..i], b)
                .map(|n| last = Some(n))
                .is_none()
        })
    });
    last
}
