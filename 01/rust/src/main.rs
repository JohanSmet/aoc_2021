fn part1(values: &[i32]) {
    let mut result = 0;
    let mut last_value = values[0];

    for value in &values[1 ..] {
        if *value > last_value {
            result += 1;
        }
        last_value = *value;
    }

    println!("Part 1 result = {}", result);
}

fn part2(values: &[i32]) {

    let mut windows = values.windows(3);
    let mut result = 0;

    let first_window = windows.next().expect("Non-empty window");
    let mut last_sum: i32 = first_window.iter().sum();

    for window in windows {
        let sum: i32 = window.iter().sum();
        if sum > last_sum {
            result += 1;
        }
        last_sum = sum;
    }

    println!("Part 2 result = {}", result);
}

fn main() {
    let input_data = include_str!("../../input.txt");

    // convert input to an vector of integer
    let mut values : Vec<i32> = Vec::new();
    for line in input_data.lines() {
        values.push(line.parse().unwrap_or(0));
    }
    assert!(values.len() > 1);

    part1(&values);
    part2(&values);
}
