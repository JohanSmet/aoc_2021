use std::vec;
use std::iter::FromIterator;

struct Data {
    data: Vec<String>
}

impl Data {
    fn patterns(&self) -> &[String] {
        &self.data[0..10]
    }

    fn output(&self) -> &[String] {
        &self.data[11..15]
    }
}

fn part1(data : &Vec<Data>) {
    let mut count = 0;

    for d in data {
        for s in d.output() {
            match s.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part2(data: &Vec<Data>) {

    let mut result: usize = 0;

    for d in data {
        let mut digits_to_pattern : [&str; 10] = [""; 10];

        let decode_digit = |digits: &[&str], pattern : &str| -> usize {
            for digit in 0..10 {
                if pattern == digits[digit] {
                    return digit;
                }
            }
            panic!("Bad Input");
        };

        let mut top_right : char = ' ';
        let mut bottom_right : char = ' ';

        // find the digits with an unique count
        for p in d.patterns() {
            match p.len() {
                2 => digits_to_pattern[1] = p,
                3 => digits_to_pattern[7] = p,
                4 => digits_to_pattern[4] = p,
                7 => digits_to_pattern[8] = p,
                _ => {}
            }
        }

        // '6' is the only six segment digit which doesn't contain both segments from '1'
        let one_chars : Vec<char> = digits_to_pattern[1].chars().collect();

        for p in d.patterns() {
            if p.len() == 6 && !(p.contains(one_chars[0]) && p.contains(one_chars[1])) {
                digits_to_pattern[6] = p;

                if p.contains(one_chars[0]) {
                    bottom_right = one_chars[0];
                    top_right = one_chars[1];
                } else {
                    bottom_right = one_chars[1];
                    top_right = one_chars[0];
                }
                break;
            }
        }

        // now we can identify '2', '3' and '5'
        for p in d.patterns().iter().filter(|x| x.len() == 5) {
            if p.contains(top_right) && p.contains(bottom_right) {
                digits_to_pattern[3] = p;
            } else if !p.contains(top_right) && p.contains(bottom_right) {
                digits_to_pattern[5] = p;
            } else if p.contains(top_right) && !p.contains(bottom_right) {
                digits_to_pattern[2] = p;
            }
        }

        // the only segments '2' and '5' have in common are the horizontal segments, we can use this to differentiate between '0', '6', and 9
        let mut hor_segments: Vec<char> = vec![];

        for c in digits_to_pattern[2].chars() {
            if digits_to_pattern[5].contains(c) {
                hor_segments.push(c);
            }
        }
        assert_eq!(hor_segments.len(), 3);

        for p in d.patterns().iter().filter(|x| x.len() == 6) {
            if p.contains(hor_segments[0]) && p.contains(hor_segments[1]) && p.contains(hor_segments[2]) {
                if p.contains(top_right) {
                    digits_to_pattern[9] = p;
                } else {
                    digits_to_pattern[6] = p;
                }
            } else {
                digits_to_pattern[0] = p;
            }
        }

        // decode the digits
        let mut number = 0;

        for p in d.output() {
            number = (number * 10) + decode_digit(&digits_to_pattern, p);
        }

        result += number;
    }

    println!("Part 2: sum is {}", result);
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // read initial positions
    let data : Vec<Data> =
        input_data
            .lines()
            .map(|l| { Data { data : l.split_whitespace().map(|x| {
                    let mut buffer : Vec<char> = x.chars().collect();
                    buffer.sort();
                    String::from_iter(buffer)
                }).collect() } })
            .collect();

    // solve the problems
    part1(&data);
    part2(&data);
}
