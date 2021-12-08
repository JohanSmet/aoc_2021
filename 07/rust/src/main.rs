fn solve<F>(positions: &Vec<i32>, fuel_func : F) -> (i32, i32)
        where F: Fn(i32) -> i32 {
    // hmm, not the cleanest or most efficient approach but it runs fast enough ...
    let min_pos = positions[0];
    let max_pos = *positions.last().unwrap();

    let mut best_dist = fuel_func(min_pos);
    let mut best_pos = min_pos;

    for pos in min_pos..max_pos {
        let dist = fuel_func(pos + 1);
        if dist < best_dist {
            best_dist = dist;
            best_pos  = pos + 1;
        } else {
            // not decreasing anymore, reach lowest point
            break;
        }
    }

    return (best_pos, best_dist)
}

fn part1(positions: &Vec<i32>) {
    let (pos, fuel) = solve(positions, |pos: i32| positions.iter().fold(0, |sum, x| sum + (x - pos).abs()));
    println!("Part 1: best position = {} with used fuel = {}", pos, fuel);
}

fn part2(positions: &Vec<i32>) {

    // pre-compute the fuel costs
    let mut fuel_costs = vec!(0);
    for i in 1..*positions.last().unwrap()+1 {
        fuel_costs.push(fuel_costs.last().unwrap() + i);
    }

    let (pos, fuel) = solve(positions, |pos: i32| positions.iter().fold(0, |sum, x| {
        sum + fuel_costs[(x - pos).abs() as usize]
    }));

    println!("Part 2: best position = {} with used fuel = {}", pos, fuel);
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read initial positions
    let mut positions : Vec<i32> = input_data.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    positions.sort();

    // solve the problems
    part1(&positions);
    part2(&positions);
}
