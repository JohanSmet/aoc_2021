#[derive(Clone)]
struct Board {
    numbers : [[i32;5];5],
    marks: [[bool;5];5]
}

impl Board {
    fn draw_number(&mut self, n: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == n {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            let mut row = true;
            let mut col = true;
            for j in 0..5 {
                row = row && self.marks[i][j];
                col = col && self.marks[j][i];
            }

            if row || col {
                return true;
            }
        }

        return false;
    }

    fn sum_of_unmarked(&self) -> i32 {
        let mut result = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    result += self.numbers[i][j];
                }
            }
        }

        return result;
    }
}

#[derive(Clone)]
struct Data {
    moves : Vec<i32>,
    boards: Vec<Board>
}

impl Data {
    fn new() -> Self {
        Data {
            moves: Vec::new(),
            boards: Vec::new()
        }
    }
}

fn part1(data: &mut Data) {
    for m in &data.moves {
        for board in &mut data.boards {
            board.draw_number(*m);

            if board.has_bingo() {
                println!("Part 1: bingo with {} => result = {}", m, board.sum_of_unmarked() * m);
                return;
            }
        }
    }
}

fn part2(data: &mut Data) {

    let mut result = 0;

    for m in &data.moves {
        for board in &mut data.boards {
            board.draw_number(*m);

            if board.has_bingo() {
                result = board.sum_of_unmarked() * m;
            }
        }

        data.boards.retain(|b| !b.has_bingo());
    }

    println!("Part 2: last bingo = {}", result);
}

fn main() {
    let input_data = include_str!("../../input.txt");

    // parse input data
    let mut input_iter = input_data.lines();
    let mut data = Data::new();

    // >> first line are the moves
    data.moves = input_iter.next()
                    .expect("Invalid input")
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap_or(0))
                    .collect();
    assert!(data.moves.len() > 1);

    // >> read the boards
    let mut line = input_iter.next();
    while line.is_some() {
        assert!(line == Some(""));

        let mut numbers : [[i32; 5]; 5] = [[0; 5]; 5];

        for r in 0..5 {
            for (i, num) in input_iter.next()
                                .expect("Input incomplete")
                                .split_ascii_whitespace()
                                .map(|x| x.parse::<i32>().unwrap_or(0))
                                .enumerate() {
                numbers[r][i] = num;
            }
        }

        data.boards.push(Board {
            numbers,
            marks: [[false; 5]; 5],
        });

        line = input_iter.next();
    }

    part1(&mut data.clone());
    part2(&mut data.clone());
}
