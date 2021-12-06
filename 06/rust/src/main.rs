fn solve(population: &mut [i64; 9], days: usize) {
// The problem statement reduces to adding the 0th element to the 7th element and rotating the array to the left.
// This can further be optimized by shifting the indices instead of copying the array each step.

    for step in 0..days {
        population[(step+7)%9] += population[step%9];
    }

    println!("Population after {} days = {}", days, population.iter().sum::<i64>());
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read initial population
    let mut population : [i64; 9] = [0; 9];
    input_data.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| population[x] += 1);

    // solve problem
    solve(&mut population.clone(), 18);
    solve(&mut population.clone(), 80);
    solve(&mut population.clone(), 256);
}
