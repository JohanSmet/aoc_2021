const TEST_RUN: bool = false;
const P1_START: u8 = if TEST_RUN {4} else {7};
const P2_START: u8 = if TEST_RUN {8} else {5};

fn part1() -> i32 {

    let mut pos: [i32; 2] = [P1_START as i32, P2_START as i32];
    let mut score: [i32; 2] = [0, 0];

    let mut dice_count = 0;
    let mut dice_value = 0;

    let mut dice = || -> i32 {
        dice_value += 1;
        if dice_value > 100 {
            dice_value = 1
        };
        dice_value
    };

    loop {
        for p in 0..2 {
            pos[p] += dice() + dice() + dice();
            while pos[p] > 10 {
                pos[p] -= 10;
            }
            score[p] += pos[p] as i32;
            dice_count += 3;

            if score[p] >= 1000 {
                return dice_count * score[(p+1)%2];
            }
        }
    }
}

#[derive(Clone)]
struct State {
    pos: [u8; 2],
    score: [u8; 2],
    player: u8
}

fn p2_recurse(state: &State) -> [usize; 2] {

    const THROW_COUNTS : [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    let mut result = [0; 2];

    for throw in 3..10 {

        let mut next_state = state.clone();
        let p = state.player as usize;

        next_state.pos[p] += throw;
        while next_state.pos[p] > 10 {
            next_state.pos[p] -= 10;
        }
        next_state.score[p] += next_state.pos[p];
        next_state.player = (next_state.player + 1) % 2;

        if next_state.score[p] >= 21 {
            result[p] += THROW_COUNTS[throw as usize];
        } else {
            let wins = p2_recurse(&next_state);
            result[0] += wins[0] * THROW_COUNTS[throw as usize];
            result[1] += wins[1] * THROW_COUNTS[throw as usize];
        }
    }

    result
}

fn part2() -> usize {

    let result = p2_recurse(&State{
        pos: [P1_START, P2_START],
        score: [0; 2],
        player: 0
    });

    std::cmp::max(result[0], result[1])
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
