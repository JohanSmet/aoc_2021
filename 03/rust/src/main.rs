fn part1(values: &[&str]) {

    let mut bit_counts = vec![0; values[0].len()];

    // count values per bit
    for value in values {
        for (i, c) in value.chars().enumerate() {
            bit_counts[i] += if c == '1' {1} else {0};
        }
    }

    // compute gamma rate
    let mut gamma_rate = 0;
    let limit = values.len() / 2;
    for count in &bit_counts {
        gamma_rate = (gamma_rate << 1) | if *count > limit {1} else {0};
    }

    // epsilon rate is the complement of the gamma rate
    let epsilon_rate = gamma_rate ^ ((1 << bit_counts.len()) - 1);

    println!("Part1: gamma rate = {} / epsilon rate = {} / result = {}",
                gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn part2(values: &[&str]) {

    let mut oxygen_values = values.to_vec();
    let mut oxygen_index = 0;

    while oxygen_values.len() > 1 {
        let count_ones: usize = oxygen_values.iter().map(|&v| if v.chars().nth(oxygen_index) == Some('1') {1} else {0}).sum();
        let retain_value = if count_ones >= oxygen_values.len() - count_ones  {'1'} else {'0'};

        oxygen_values.retain(|&v| v.chars().nth(oxygen_index) == Some(retain_value));
        oxygen_index += 1;
    }

    let mut co2_values = values.to_vec();
    let mut co2_index = 0;

    while co2_values.len() > 1 {
        let count_ones: usize = co2_values.iter().map(|&v| if v.chars().nth(co2_index) == Some('1') {1} else {0}).sum();
        let retain_value = if count_ones >= co2_values.len() - count_ones  {'0'} else {'1'};

        co2_values.retain(|&v| v.chars().nth(co2_index) == Some(retain_value));
        co2_index += 1;
    }

    let oxygen_rating = i32::from_str_radix(oxygen_values[0], 2).unwrap_or(0);
    let co2_rating = i32::from_str_radix(co2_values[0], 2).unwrap_or(0);

    println!("Part2: oxygen generator rating = {} / co2 scrubber rating = {} / life support rating = {}",
                oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}

fn main() {
    let input_data = include_str!("../../input.txt");

    // convert input to an vector of string slices
    let values : Vec<&str> = input_data.lines().collect();
    assert!(values.len() > 1);

    part1(&values);
    part2(&values);
}
