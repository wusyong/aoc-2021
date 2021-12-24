use std::collections::HashMap;

fn main() {
    let p1 = part1([8, 1]);
    let p2 = part2(
        &mut HashMap::new(),
        State {
            players: [0, 0],
            pos: [8, 1],
            turn: 0,
        },
    )
    .into_iter()
    .max();

    println!("part1: {}", p1);
    println!("part2: {:?}", p2);
}

fn part1(mut pos: [usize; 2]) -> usize {
    let mut die = (1..=100).cycle();
    let mut scores = [0, 0];
    let mut nrolls = 0;
    let mut turn = 0;
    loop {
        let roll: usize = (0..3).map(|_| die.next().unwrap()).sum();
        nrolls += 3;
        pos[turn] = (pos[turn] - 1 + roll) % 10 + 1;
        scores[turn] += pos[turn];
        if scores[turn] >= 1000 {
            break;
        }
        turn = (turn + 1) % 2;
    }
    nrolls * scores[(turn + 1) % 2]
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct State {
    players: [usize; 2],
    pos: [usize; 2],
    turn: usize,
}

fn part2(state: &mut HashMap<State, [usize; 2]>, next: State) -> [usize; 2] {
    let mut score = [0, 0];
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let die = i + j + k;
                let s = {
                    let mut new = next.clone();
                    let t = new.turn;
                    let p = (new.pos[t] - 1 + die) % 10 + 1;
                    new.players[t] += p;
                    new.pos[t] = p;
                    new.turn = (new.turn + 1) % 2;

                    if new.players[0] >= 21 {
                        [1, 0]
                    } else if new.players[1] >= 21 {
                        [0, 1]
                    } else if let Some(&score) = state.get(&new) {
                        score
                    } else if let Some(&score) = state.get(&{
                        let mut rev = new.clone();
                        rev.players.reverse();
                        rev.pos.reverse();
                        rev.turn = next.turn;
                        rev
                    }) {
                        [score[1], score[0]]
                    } else {
                        part2(state, new)
                    }
                };
                score[0] += s[0];
                score[1] += s[1];
            }
        }
    }
    state.insert(next, score);
    score
}
