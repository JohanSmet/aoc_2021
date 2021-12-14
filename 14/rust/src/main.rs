use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Key {
    pair: String,
    steps: i32,
}

type CharCounts = HashMap<char, i64>;
type OperationCache = HashMap<Key, CharCounts>;
type Rules = HashMap<String, char>;

impl Key {
    fn new(pair: &str, steps: i32) -> Self {
        Key { pair: pair.to_string(), steps }
    }
}

fn min_max_difference(counts: &CharCounts) -> i64 {

    let min: &i64 = counts.values().min().unwrap_or(&0);
    let max: &i64 = counts.values().max().unwrap_or(&0);

    max - min
}

fn merge_counts(counts: &mut CharCounts, other: &CharCounts) {

    for (c, count) in other {
        *counts.entry(*c).or_insert(0) += count;
    }
}

fn process_pair(cache: &mut OperationCache, rules: &Rules, pair: &str, steps: i32) -> CharCounts {

    let key = Key::new(pair, steps);

    // check cache
    if let Some(cached) = cache.get(&key) {
        return cached.clone();
    }

    let mut counts : CharCounts = CharCounts::new();

    if let Some(extra) = rules.get(pair) {
        counts.insert(*extra, 1);

        if steps > 1 {
            let mut char_iter = pair.chars();
            let pair_1 = char_iter.next().unwrap().to_string() + &extra.to_string();
            let pair_2 = extra.to_string() + &char_iter.next().unwrap().to_string();

            merge_counts(&mut counts, &process_pair(cache, rules, &pair_1, steps - 1));
            merge_counts(&mut counts, &process_pair(cache, rules, &pair_2, steps - 1));
        }
    }

    cache.insert(key, counts.clone());
    counts
}

fn process_polymer(cache: &mut OperationCache, template: &str, rules: &Rules, steps: i32) -> CharCounts {

    let mut template_iter = template.chars();
    let mut pair: String = template_iter.next().unwrap().to_string();
    let mut counts = CharCounts::new();

    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for next_char in template_iter {
        pair.push(next_char);
        merge_counts(&mut counts, &process_pair(cache, rules, &pair, steps));
        pair = next_char.to_string();
    }

    counts
}

fn main() {
    let mut input_iter = include_str!("../../input.txt").lines();

    // parse input data
    // >> first line is the template
    let template = input_iter.next().expect("Invalid input");
    assert!(!template.is_empty());

    // >> read the pair insertion rules
    let mut rules : HashMap<String, char> = HashMap::new();

    input_iter.next();      // skip empty line
    for line in input_iter {
        let mut parts = line.split_whitespace();
        rules.insert(parts.next().unwrap().to_string(), parts.nth(1).unwrap().chars().next().unwrap());
    }

    // solve the problems
    let mut cache = OperationCache::new();

    let counts = process_polymer(&mut cache, template, &rules, 10);
    println!("Part 1: {}", min_max_difference(&counts));

    let counts = process_polymer(&mut cache, template, &rules, 40);
    println!("Part 2: {}", min_max_difference(&counts));
}
