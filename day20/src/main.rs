use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (key, input) = contents.split_once("\n\n").unwrap();

    println!("part1: {}", solve(key, input, 2));
    println!("part2: {}", solve(key, input, 50));
    Ok(())
}

fn solve(key: &str, input: &str, n: usize) -> usize {
    let mut image = Image(
        input.lines().map(|line| line.bytes().collect()).collect(),
        key.as_bytes(),
    );
    image.expand(b'.');
    (0..n).for_each(|_| image.enhance());
    image.0.iter()
        .map(|row| row.iter().filter(|&v| *v == b'#').count())
        .sum()
}

struct Image<'a>(Vec<Vec<u8>>, &'a [u8]);
impl Image<'_> {
    fn enhance(&mut self) {
        self.expand(self.0[0][0]);
        let mut grid = self.0.clone();
        (1..self.0.len() - 1).for_each(|r| {
            (1..self.0[r].len() - 1).for_each(|c| {
                let idx = [
                    self.0[r - 1][c - 1],
                    self.0[r - 1][c],
                    self.0[r - 1][c + 1],
                    self.0[r][c - 1],
                    self.0[r][c],
                    self.0[r][c + 1],
                    self.0[r + 1][c - 1],
                    self.0[r + 1][c],
                    self.0[r + 1][c + 1],
                ]
                .iter()
                .fold(0, |a, b| a << 1 | (*b == b'#') as usize);
                grid[r][c] = self.1[idx];
            });
        });

        let ch = self.1[[grid[0][0]; 9]
            .iter()
            .fold(0, |a, b| a << 1 | (*b == b'#') as usize)];
        let last = grid.len() - 1;
        for (i, row) in grid.iter_mut().enumerate() {
            row[0] = ch;
            *row.last_mut().unwrap() = ch;
            if i == 0 || i == last {
                row.iter_mut().for_each(|v| *v = ch);
            }
        }
        self.0 = grid;
    }

    fn expand(&mut self, c: u8) {
        self.0.insert(0, vec![c; self.0[0].len()]);
        self.0.push(vec![c; self.0[0].len()]);
        for i in self.0.iter_mut() {
            i.insert(0, c);
            i.push(c);
        }
    }
}
