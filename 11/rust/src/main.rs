struct Grid {
    grid: Vec<i32>,
    width : i32,
    height : i32,
    steps_done : i32,
    first_all_flash : i32
}

impl Grid {
    fn valid_cell(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn cell_value(&self, x: i32, y: i32) -> i32 {
        if self.valid_cell(x, y) {
            self.grid[((y * self.width) + x) as usize]
        } else {
            -1
        }
    }

    fn cell_increment_if_valid(&mut self, x: i32, y: i32) {
        if self.valid_cell(x, y) {
            let idx = ((y * self.width) + x) as usize;
            if self.grid[idx] != 0 {
                self.grid[idx] += 1;
            }

        }
    }

    fn flash_grid_once(&mut self) -> i32 {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell_value(x, y) > 9 {
                    self.flash_cell(x, y);
                    count += 1;
                }
            }
        }

        count
    }

    fn flash_cell(&mut self, x: i32, y: i32) {
        self.grid[((y * self.width) + x) as usize] = 0;

        for fy in y-1..y+2 {
            for fx in x-1..x+2 {
                self.cell_increment_if_valid(fx, fy);
            }
        }
    }
}

fn solve(grid: &mut Grid, steps: i32, run_all : bool) -> i32 {

    let mut count_flashes = 0;

    for step in 0..steps {

        // increment energy level of each octopus
        for i in grid.grid.iter_mut() {
            *i += 1;
        }

        // flash any octopus with an energy level greater than 9
        loop {
            let count = grid.flash_grid_once();
            count_flashes += count;
            if count == 0 {
                break;
            }
        }

        // check if all octopuses flashed
        if grid.first_all_flash == 0 && grid.grid.iter().sum::<i32>() == 0 {
            grid.first_all_flash = grid.steps_done + step + 1;
            if !run_all {
                break;
            }
        }
    }

    grid.steps_done += steps;
    count_flashes
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let mut grid = Grid {grid: vec![], width: 0, height: 0, steps_done: 0, first_all_flash: 0};
    for l in input_data.lines() {
        grid.grid.extend(l.bytes().map(|b| (b as i32) - 48));
        grid.height += 1;
        grid.width = l.len() as i32;
    }

    // solve the problems
    let part1_flashes = solve(&mut grid, 100, true);
    println!("Part 1: number of flashes = {} after 100 steps", part1_flashes);
    while grid.first_all_flash == 0 {
        solve(&mut grid, 1000, false);
    }
    println!("Part 2: all flashed at {}", grid.first_all_flash);
}
