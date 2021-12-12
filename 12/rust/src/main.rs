use std::collections::HashMap;

struct Cave {
    connections: Vec<usize>,
    visit_once: bool
}

struct CaveSystem {
    caves: Vec<Cave>,
    cave_name_map: HashMap<String, usize>,
    start: usize,
    end: usize
}

impl Cave {
    fn new(name: &str) -> Self {
        Cave {
            connections: vec![],
            visit_once: name.to_lowercase() == *name
        }
    }

    fn add_connection(&mut self, index: usize) {
        if !self.connections.contains(&index) {
            self.connections.push(index);
        }
    }
}

impl CaveSystem {
    fn new() -> Self {
        Self { caves: vec![], cave_name_map: HashMap::new(), start: 0, end: 0}
    }

    fn fetch_or_create_cave(&mut self, cave_name: &str) -> usize {
        match self.cave_name_map.get(cave_name) {
            Some(index) => *index,
            None => {
                let index = self.caves.len();
                self.caves.push(Cave::new(cave_name));
                self.cave_name_map.insert(cave_name.to_string(), index);
                if cave_name == "start" {
                    self.start = index;
                } else if cave_name == "end" {
                    self.end = index;
                }
                index
            }
        }
    }

    fn connect_nodes(&mut self, a: usize, b: usize) {
        self.caves[a].add_connection(b);
        self.caves[b].add_connection(a);
    }
}

fn r_traverse_1(cave_system: &CaveSystem, cur_cave: usize, path: &mut Vec<usize>) -> usize {

    if cur_cave == cave_system.end {
        return 1;
    }

    path.push(cur_cave);
    let mut count = 0;

    for next in &cave_system.caves[cur_cave].connections {
        if cave_system.caves[*next].visit_once && path.contains(next) {
            continue;
        }

        count += r_traverse_1(cave_system, *next, path);
    }

    assert_eq!(path.pop().expect("current cave"), cur_cave);
    count
}

fn part1(cave_system: &CaveSystem) {
    let count = r_traverse_1(cave_system, cave_system.start, &mut vec![]);
    println!("Part 1: {} paths", count);
}

fn r_traverse_2(cave_system: &CaveSystem, cur_cave: usize, path: &mut Vec<usize>, allow_double: bool) -> usize {

    if cur_cave == cave_system.end {
        return 1;
    }

    path.push(cur_cave);
    let mut count = 0;

    for next in &cave_system.caves[cur_cave].connections {
        if cave_system.caves[*next].visit_once && path.contains(next) {
            if allow_double && *next != cave_system.start {
                count += r_traverse_2(cave_system, *next, path, false);
            } else {
                continue;
            }
        } else {
            count += r_traverse_2(cave_system, *next, path, allow_double);
        }
    }

    assert_eq!(path.pop().expect("current cave"), cur_cave);
    count
}

fn part2(cave_system: &CaveSystem) {
    let count = r_traverse_2(cave_system, cave_system.start, &mut vec![], true);
    println!("Part 2: {} paths", count);
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let mut cave_system = CaveSystem::new();

    for line in input_data.lines() {
        let nodes : Vec<usize> = line.split('-').map(|n| cave_system.fetch_or_create_cave(n)).collect();
        assert_eq!(nodes.len(), 2);
        cave_system.connect_nodes(nodes[0], nodes[1]);
    }

    // solve the problems
    part1(&cave_system);
    part2(&cave_system);
}
