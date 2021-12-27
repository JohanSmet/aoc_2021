#[derive(Debug)]
struct Grid {
    grid: Vec<char>,
    width : usize,
    height : usize,
}

impl Grid {
    fn new() -> Self {
        Grid {grid: vec![], width: 0, height: 0}
    }

    fn cell_index(&self, x: usize, y: usize) -> usize {
        ((y % self.height) * self.width) + (x % self.width)
    }

    fn cell_value(&self, x: usize, y: usize) -> char {
        self.grid[self.cell_index(x, y)]
    }

    fn perform_step(&mut self) -> usize {
        let mut count_moves = 0;

        // perform moves to right
        let mut next_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell_value(x, y) == '>' && self.cell_value(x + 1, y) == '.' {
                    next_grid[self.cell_index(x, y)] = '.';
                    next_grid[self.cell_index(x+1, y)] = '>';
                    count_moves += 1;
                }
            }
        }

        self.grid = next_grid;

        // perform moves downwards
        let mut next_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell_value(x, y) == 'v' && self.cell_value(x, y+1) == '.' {
                    next_grid[self.cell_index(x, y)] = '.';
                    next_grid[self.cell_index(x, y+1)] = 'v';
                    count_moves += 1;
                }
            }
        }

        self.grid = next_grid;

        count_moves
    }
}

fn part1(grid : &mut Grid) {
    let mut steps = 0;

    while grid.perform_step() != 0 {
        steps += 1;
    }

    println!("Part1: {} steps", steps + 1);
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let mut grid = Grid::new();
    for l in input_data.lines() {
        grid.grid.extend(l.chars());
        grid.height += 1;
        grid.width = l.len();
    }

    // solve the problems
    part1(&mut grid);
}
