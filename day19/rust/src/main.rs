use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (raw_patterns, raw_designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = raw_patterns.split(", ").collect();
    let designs: Vec<&str> = raw_designs.lines().collect();

    let first_answer = designs
        .iter()
        .filter(|design| is_possible(&patterns, design))
        .count();
    println!("First answer: {first_answer}");

    let mut cache: HashMap<&str, usize> = HashMap::new();
    let second_answer: usize = designs
        .iter()
        .map(|design| arrangements(&patterns, design, &mut cache))
        .sum();
    println!("Second answer: {second_answer}");
}

fn is_possible(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    patterns.iter().any(|pattern| {
        if design.starts_with(pattern) {
            let (_, rest) = design.split_at(pattern.len());
            is_possible(patterns, rest)
        } else {
            false
        }
    })
}

fn arrangements<'a>(
    patterns: &[&str],
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(result) = cache.get(design) {
        return *result;
    }

    if design.is_empty() {
        return 1;
    }

    let result = patterns
        .iter()
        .map(|pattern| {
            if design.starts_with(pattern) {
                let (_, rest) = design.split_at(pattern.len());
                arrangements(patterns, rest, cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design, result);
    result
}
