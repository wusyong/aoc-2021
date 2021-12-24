use std::ops::RangeInclusive;

static X: RangeInclusive<i32> = 119..=176;
static Y: RangeInclusive<i32> = -141..=-84;

fn calc(mut vx: i32, mut vy: i32) -> Option<i32> {
    let (mut x, mut y, mut maxy) = (0, 0, 0);
    loop {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        maxy = maxy.max(y);
        match (X.contains(&x), Y.contains(&y)) {
            (true, true) => return Some(maxy),
            (false, _) if vx == 0 => return None,
            (_, false) if vy < 0 && y < -141 => return None,
            _ => ()
        }
    }
}

fn main() {
    let mut maxy = Vec::new();
    for x in 0..=176 {
        for y in -141..200 { // Add to 300 if fail lol
            if let Some(i) = calc(x, y) {
                maxy.push(i);
            }
        }
    }
    println!("part1: {:?}", maxy.iter().max());
    println!("part2: {}", maxy.len());
}
