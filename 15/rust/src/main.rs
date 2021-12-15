use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug)]
struct Grid {
    grid: Vec<usize>,
    width : usize,
    height : usize,
}

type Point = [i32; 2];

impl Grid {
    fn new() -> Self {
        Grid {grid: vec![], width: 0, height: 0}
    }

    fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn cell_index(&self, p: &Point) -> usize {
        (p[1] as usize * self.width) + p[0] as usize
    }

    fn valid_cell(&self, p: &Point) -> bool {
        p[0] >= 0 && p[1] >= 0 && p[0] < self.width as i32 && p[1] < self.height as i32
    }

    fn cell_value(&self, p: &Point) -> usize {
        if self.valid_cell(p) {
            self.grid[self.cell_index(p)]
        } else {
            usize::MAX
        }
    }

    fn set_cell_value(&mut self, p: &Point, v: usize) {
        if self.valid_cell(p) {
            let i = self.cell_index(p);
            self.grid[i] = v;
        }
    }
}

// Dijkstra's algorithm
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // comparison of cost is flipped to obtain a min-heap
        other.cost.cmp(&self.cost)
             .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &Grid, start: &Point, target: &Point) -> Option<usize> {
    // dist[node] = current shortest distance from 'start' to that node
    let mut dist: Vec<usize> = vec![usize::MAX; grid.size()];
    dist[grid.cell_index(start)] = 0;

    // initialize the priority queue
    let mut to_visit = BinaryHeap::new();
    to_visit.push(State {cost: 0, position: *start});

    // examine the next step with the lowest associated cost
    while let Some(State { cost, position }) = to_visit.pop() {

        // stop if arrived at target
        if position.cmp(target) == Ordering::Equal {
            return Some(cost);
        }

        // skip path with higher cost than previously discovered
        if cost > dist[grid.cell_index(&position)] {
            continue;
        }

        // check if the path through this node is a shorter way to reach our neighbours
        let neighbours: [Point; 4] = [
            [position[0] - 1, position[1]],
            [position[0] + 1, position[1]],
            [position[0], position[1] - 1],
            [position[0], position[1] + 1],
        ];

        for next_pos in neighbours.iter().filter(|p| grid.valid_cell(p)) {
            let next_visit = State {cost: cost + grid.cell_value(next_pos), position: *next_pos};

            if next_visit.cost < dist[grid.cell_index(next_pos)] {
                to_visit.push(next_visit);
                dist[grid.cell_index(next_pos)] = next_visit.cost;
            }
        }
    }


    // target not reachable
    None
}

fn part1(grid: &Grid) {
    let cost = shortest_path(grid, &[0, 0],  &[grid.width as i32 - 1, grid.height as i32 - 1]);
    println!("Part 1: cost of shortest path = {:?}", cost);
}

fn part2(grid: &Grid) {
    // construct the full grid
    let mut full_grid = Grid {
        grid: vec![0; grid.width * grid.height * 25],
        width: grid.width * 5,
        height: grid.height * 5
    };

    for big_y in 0..full_grid.height as i32 {

        let y = big_y % grid.height as i32;
        let delta_y = big_y as usize / grid.height;

        for big_x in 0..full_grid.width as i32 {

            let x = big_x % grid.width as i32;
            let delta_x = big_x as usize / grid.width;

            let value = grid.cell_value(&[x,y]) + delta_x + delta_y;
            full_grid.set_cell_value(&[big_x, big_y], if value > 9 {value - 9} else {value});
        }
    }

    let cost = shortest_path(&full_grid, &[0, 0],  &[full_grid.width as i32 - 1, full_grid.height as i32 - 1]);
    println!("Part 2: cost of shortest path = {:?}", cost);

}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let mut grid = Grid::new();
    for l in input_data.lines() {
        grid.grid.extend(l.bytes().map(|b| (b as usize) - 48));
        grid.height += 1;
        grid.width = l.len();
    }

    // solve the problems
    part1(&grid);
    part2(&grid);
}
