use std::collections::HashMap;

const TEST_RUN: bool = false;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Cell {
    neighbours: Vec<usize>,
    value: char,
    target_value: char,
    fixed: bool
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Maze {
    cells: Vec<Cell>
}

type Cache = HashMap<Maze, usize>;
type Move = (usize, usize, usize);

impl Cell {
    pub fn new(target_value: char, neighbours: &[usize]) -> Self {
        Cell {neighbours: neighbours.to_vec(), value: ' ', target_value, fixed: false}
    }

    pub fn is_corridor(&self) -> bool {
        self.target_value == ' '
    }

    pub fn is_room(&self) -> bool {
        self.target_value != ' '
    }
}

impl Maze {

    pub fn new(part: i32) -> Self {
        let mut cells = vec![
            Cell::new(' ', &[1]),           // corridor cell 0
            Cell::new(' ', &[0, 2]),        // corridor cell 1
            Cell::new(' ', &[1, 3]),        // corridor cell 2
            Cell::new(' ', &[2, 4]),        // corridor cell 3
            Cell::new(' ', &[3, 5]),        // corridor cell 4
            Cell::new(' ', &[4, 6]),        // corridor cell 5
            Cell::new(' ', &[5, 7]),        // corridor cell 6
            Cell::new(' ', &[6, 8]),        // corridor cell 7
            Cell::new(' ', &[7, 9]),        // corridor cell 8
            Cell::new(' ', &[8, 10]),       // corridor cell 9
            Cell::new(' ', &[9]),           // corridor cell 10
            Cell::new('A', &[2, 12]),       // cell 11
            Cell::new('A', &[11]),          // cell 12
            Cell::new('B', &[4, 14]),       // cell 13
            Cell::new('B', &[13]),          // cell 14
            Cell::new('C', &[6, 16]),       // cell 15
            Cell::new('C', &[15]),          // cell 16
            Cell::new('D', &[8, 18]),       // cell 17
            Cell::new('D', &[17]),          // cell 18
        ];

        if part == 2 {
            cells.push(Cell::new('A', &[12, 20]));      // cell 19
            cells.push(Cell::new('A', &[19]));          // cell 20
            cells.push(Cell::new('B', &[14, 22]));      // cell 21
            cells.push(Cell::new('B', &[21]));          // cell 22
            cells.push(Cell::new('C', &[16, 24]));      // cell 23
            cells.push(Cell::new('C', &[23]));          // cell 24
            cells.push(Cell::new('D', &[18, 26]));      // cell 25
            cells.push(Cell::new('D', &[25]));          // cell 26

            cells[12].neighbours.push(19);
            cells[14].neighbours.push(21);
            cells[16].neighbours.push(23);
            cells[18].neighbours.push(26);
        }

        if TEST_RUN && part == 1 {
            cells[11].value = 'B';
            cells[12].value = 'A';
            cells[13].value = 'C';
            cells[14].value = 'D';
            cells[15].value = 'B';
            cells[16].value = 'C';
            cells[17].value = 'D';
            cells[18].value = 'A';
        } else if TEST_RUN && part == 2 {
            cells[11].value = 'B';
            cells[12].value = 'D';
            cells[19].value = 'D';
            cells[20].value = 'A';
            cells[13].value = 'C';
            cells[14].value = 'C';
            cells[21].value = 'B';
            cells[22].value = 'D';
            cells[15].value = 'B';
            cells[16].value = 'B';
            cells[23].value = 'A';
            cells[24].value = 'C';
            cells[17].value = 'D';
            cells[18].value = 'A';
            cells[25].value = 'C';
            cells[26].value = 'A';
        } else if part == 1 {
            cells[11].value = 'B';
            cells[12].value = 'D';
            cells[13].value = 'B';
            cells[14].value = 'C';
            cells[15].value = 'D';
            cells[16].value = 'A';
            cells[17].value = 'A';
            cells[18].value = 'C';
        } else if part == 2 {
            cells[11].value = 'B';
            cells[12].value = 'D';
            cells[19].value = 'D';
            cells[20].value = 'D';
            cells[13].value = 'B';
            cells[14].value = 'C';
            cells[21].value = 'B';
            cells[22].value = 'C';
            cells[15].value = 'D';
            cells[16].value = 'B';
            cells[23].value = 'A';
            cells[24].value = 'A';
            cells[17].value = 'A';
            cells[18].value = 'A';
            cells[25].value = 'C';
            cells[26].value = 'C';
        }

        Maze { cells }
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().map(|x| if x.value != x.target_value {1} else {0}).sum::<usize>() == 0
    }
}

fn amphipod_cost(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0
    }
}

fn amphipod_target(maze: &Maze, amphipod: char) -> Option<usize> {
    // find the highest free room cell
    for cell in (0..maze.cells.len()).rev() {
        if maze.cells[cell].target_value == amphipod && maze.cells[cell].value == ' ' {
            return Some(cell);
        }
    }

    None
}

fn maze_move_to_room(maze: &Maze, start: usize, moves: &mut Vec<Move>) {

    let target_cell = amphipod_target(maze, maze.cells[start].value);
    if target_cell.is_none() {
        return;
    }
    let target = target_cell.unwrap();

    assert_eq!(maze.cells[target].target_value, maze.cells[start].value);

    // trace path backwards from target to start
    let mut count_steps = 0;
    let mut current = target;

    if target % 2 == 0 {
        // second room, move to first room at junction with corridor
        current = target - 1;
        count_steps += 1;
    }

    // >> move into corridor
    while maze.cells[current].is_room() {
        current = maze.cells[current].neighbours[0];
        count_steps += 1;
    }

    // >> move to start
    while current != start {
        if maze.cells[current].value != ' ' {
            // occupied cell blocking the way, no solution for this state
            return;
        }

        if current < start {
            current += 1;
        } else {
            current -= 1;
        }

        count_steps += 1;
    }

    moves.push((start, target, count_steps));
}

fn maze_move_to_corridor(maze: &Maze, start: usize, moves: &mut Vec<Move>) {

    let mut stack = vec![(start, 0)];
    let mut visited = vec![false; maze.cells.len()];

    while let Some((current, step_count)) = stack.pop() {

        visited[current] = true;

        // check neighbours
        for &n in &maze.cells[current].neighbours {
            if maze.cells[n].value == ' ' && !visited[n] {
                stack.push((n, step_count + 1));
            }
        }

        // solve a maze where the amphipod moved to this cell
        if maze.cells[current].target_value == ' ' && current != 2 && current != 4 && current != 6 && current != 8 {
            moves.push((start, current, step_count));
        }
    }
}

fn maze_solve(maze: &Maze, cache: &mut Cache) -> (usize, bool) {

    if let Some(cost) = cache.get(maze) {
        return (*cost, *cost != usize::MAX);
    }

    if maze.is_solved() {
        return (0, true);
    }

    // build a list of potential moves
    let mut potential_moves = vec![];

    for cell in 0..maze.cells.len() {

        if maze.cells[cell].value == ' ' || maze.cells[cell].fixed {
            continue;
        }

        if maze.cells[cell].is_corridor() {
            maze_move_to_room(maze, cell, &mut potential_moves);
        } else {
            maze_move_to_corridor(maze, cell, &mut potential_moves);
        }
    }

    // find the move that leads to the minimal cost solution
    let mut minimal_cost = usize::MAX;

    for (start, target, steps) in potential_moves {

        // cost of this move
        let move_cost = steps * amphipod_cost(maze.cells[start].value);

        // recurse to a maze where this move has happened
        let mut next_maze = maze.clone();
        next_maze.cells[target].value = maze.cells[start].value;
        next_maze.cells[target].fixed = maze.cells[target].is_room();
        next_maze.cells[start].value = ' ';

        let (child_cost, child_solved) = maze_solve(&next_maze, cache);

        if child_solved {
            minimal_cost = std::cmp::min(minimal_cost, move_cost + child_cost);
        }
    }

    cache.insert(maze.clone(), minimal_cost);
    (minimal_cost, minimal_cost != usize::MAX)
}

fn main() {
    // part 1
    let maze = Maze::new(1);
    let mut cache = Cache::new();
    println!("Part 1: {:?}", maze_solve(&maze, &mut cache));

    // part 2
    let maze = Maze::new(2);
    let mut cache = Cache::new();
    println!("Part 2: {:?}", maze_solve(&maze, &mut cache));
}
