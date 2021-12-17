use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut input = HashMap::new();
    contents.lines().for_each(|l| {
        if let Some((a, b)) = l.split_once('-') {
            let a = a.chars().next().unwrap();
            let b = b.chars().next().unwrap();
            input.entry(a).or_insert(Vec::new()).push(b);
            input.entry(b).or_insert(Vec::new()).push(a);
        }
    });
    let part1 = count(&input, &mut vec![], 's', Some(' '));
    let part2 = count(&input, &mut vec![], 's', None);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn count(
    graph: &HashMap<char, Vec<char>>,
    path: &mut Vec<char>,
    from: char,
    mut seen: Option<char>,
) -> usize {
    if from == 'e' {
        return 1;
    }
    if from.is_lowercase() && path.contains(&from) {
        if seen.is_some() || from == 's' {
            return 0;
        }
        seen = Some(from);
    }

    path.push(from);
    let ans = graph[&from]
        .iter()
        .map(|n| count(graph, path, *n, seen))
        .sum();
    path.pop();
    ans
}
