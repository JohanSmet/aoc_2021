fn cell_value(grid: &[Vec<i32>], x: i32, y: i32) -> i32 {
    if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[0].len() {
        return 10;
    }

    grid[y as usize][x as usize]
}

fn cell_is_lowpoint(grid: &[Vec<i32>], x: i32, y: i32) -> bool {
    let value = cell_value(grid, x, y);

    let mut is_low = cell_value(grid, x - 1, y) > value;
    is_low = is_low && cell_value(grid, x + 1, y) > value;
    is_low = is_low && cell_value(grid, x, y - 1) > value;
    is_low = is_low && cell_value(grid, x, y + 1) > value;
    is_low
}

fn cell_is_flooded(grid: &[Vec<bool>], x: i32, y: i32) -> bool {
    grid[y as usize][x as usize]
}

fn flood_fill(grid: &[Vec<i32>], flooded: &mut Vec<Vec<bool>>, x: i32, y: i32) -> i32 {
// straight-forward recursive floodfill. Should be plenty fast for a 1000x1000 grid.

    if cell_value(grid, x, y) == 9 {
        return 0;
    }

    flooded[y as usize][x as usize] = true;
    let mut count = 1;

    if x > 0 && !cell_is_flooded(flooded, x - 1, y) {
        count += flood_fill(grid, flooded, x - 1, y);
    }

    if x < grid[0].len() as i32 - 1 && !cell_is_flooded(flooded, x + 1, y) {
        count += flood_fill(grid, flooded, x + 1, y);
    }

    if y > 0 && !cell_is_flooded(flooded, x, y - 1) {
        count += flood_fill(grid, flooded, x, y - 1);
    }

    if y < grid.len() as i32 - 1 && !cell_is_flooded(flooded, x, y + 1) {
        count += flood_fill(grid, flooded, x, y + 1);
    }


    count
}

fn part1(grid: &[Vec<i32>]) {

    let mut low_points = vec![];

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            if cell_is_lowpoint(grid, x, y) {
                low_points.push(cell_value(grid, x, y) + 1);
            }
        }
    }

    println!("Part 1: risk level = {}", low_points.iter().sum::<i32>());
}

fn part2(grid: &[Vec<i32>]) {

    let mut flooded: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut basins: Vec<i32> = vec![];

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            if cell_is_lowpoint(grid, x, y) && !cell_is_flooded(&flooded, x, y) {
                let count = flood_fill(grid, &mut flooded, x, y);
                basins.push(count);
            }
        }
    }

    basins.sort_unstable();
    println!("Part 2: result = {}", basins.iter().rev().take(3).product::<i32>());
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let grid: Vec<Vec<i32>> =
        input_data
            .lines()
            .map(|l| l.bytes().map(|b| (b as i32) - 48).collect())
            .collect();

    // solve the problems
    part1(&grid);
    part2(&grid);
}
