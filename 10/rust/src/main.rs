use std::collections::HashMap;

fn solve(chunks: &[&str]) {

    let delimiters = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let error_scores = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let completion_scores = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    let mut error_score = 0;
    let mut completion_score: Vec<i64> = vec![];

    for chunk in chunks {

        let mut expected : Vec<char> = vec![];
        let mut error = false;

        for c in chunk.chars() {

            match delimiters.get(&c) {
                Some(delim) => expected.push(*delim),
                None => {
                    let expected_c = *expected.last().unwrap_or(&' ');
                    if c != expected_c{
                        // println!("Expected {}, but found {} instead.", expected_c, c);
                        error_score += error_scores.get(&c).unwrap_or(&0);
                        error = true;
                        break;
                    }
                    expected.pop();
                }
            }
        }

        if !error {
            let mut score : i64 = 0;
            for s in expected.iter().rev() {
                score = score * 5 + completion_scores.get(s).unwrap_or(&0);
            }
            completion_score.push(score);
        }
    }

    completion_score.sort_unstable();

    println!("Part 1: score = {}", error_score);
    println!("Part 2: score = {}", completion_score[completion_score.len() / 2]);
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read input data
    let chunks: Vec<&str> = input_data.lines().collect();

    // solve the problems
    solve(&chunks);
}
