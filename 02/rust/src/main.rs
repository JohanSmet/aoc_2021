enum Direction {
    Forward,
    Down,
    Up
}

struct InputData {
    direction : Direction,
    value: i32
}

fn direction_from_string(input: &str) -> Direction {
    if input == "forward" {
        Direction::Forward
    } else if input == "down" {
        Direction::Down
    } else if input == "up" {
        Direction::Up
    } else {
        panic!("Unknown direction")
    }
}

fn part1(course: &[InputData]) {

    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    for cmd in course {
        match cmd.direction {
            Direction::Forward  => pos += cmd.value,
            Direction::Up       => depth -= cmd.value,
            Direction::Down     => depth += cmd.value,
        }
    }

    println!("Part 1: position = {}, depth = {} => result = {}", pos, depth, pos * depth)
}

fn part2(course: &[InputData]) {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for cmd in course {
        match cmd.direction {
            Direction::Forward  => { pos += cmd.value; depth += aim * cmd.value; },
            Direction::Up       => aim -= cmd.value,
            Direction::Down     => aim += cmd.value,
        }
    }

    println!("Part 2: position = {}, depth = {} => result = {}", pos, depth, pos * depth)
}

fn main() {
    let input_data = include_str!("../../input.txt");

    // convert input to an vector of InputData
    let values : Vec<InputData> = input_data
                                        .lines()
                                        .map(|l| {
                                                let parts: Vec<&str> = l.split(' ').collect();
                                                InputData{
                                                    direction: direction_from_string(parts[0]),
                                                    value: parts[1].parse::<i32>().unwrap_or(0)
                                                }
                                            })
                                        .collect();

    // solve
    part1(&values);
    part2(&values);
}
