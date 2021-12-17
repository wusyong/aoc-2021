use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut contents = contents.lines();
    let template = contents.next().unwrap();
    let mut init = HashMap::new();
    let first_char = template.chars().next().unwrap();
    template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .for_each(|i| {
            *init.entry((i[0], i[1])).or_insert(0) += 1;
        });
    contents.next();

    let mut ops = HashMap::new();
    contents.for_each(|l| {
        if let Some((a, b)) = l.split_once(" -> ") {
            let mut a = a.chars();
            ops.insert(
                (a.next().unwrap(), a.next().unwrap()),
                b.chars().next().unwrap(),
            );
        }
    });

    println!("{}", calc(init.clone(), &ops, 10, first_char));
    println!("{}", calc(init, &ops, 40, first_char));
    Ok(())
}

fn calc(
    mut count: HashMap<(char, char), usize>,
    ops: &HashMap<(char, char), char>,
    step: usize,
    first: char,
) -> usize {
    (0..step).for_each(|_| {
        let mut new_count = HashMap::new();
        count.iter().for_each(|(k, v)| {
            let i = ops[k];
            *new_count.entry((k.0, i)).or_insert(0) += v;
            *new_count.entry((i, k.1)).or_insert(0) += v;
        });
        count = new_count;
    });

    let mut freq = HashMap::new();
    count.into_iter().for_each(|((_, i), c)| {
        *freq.entry(i).or_insert(0) += c;
    });
    *freq.entry(first).or_insert(0) += 1;
    let max = freq.values().max().unwrap();
    let min = freq.values().min().unwrap();
    max - min
}
