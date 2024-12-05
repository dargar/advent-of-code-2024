use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (raw_rules, raw_updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = raw_rules
        .lines()
        .flat_map(|line| line.split_once("|"))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let updates: Vec<Vec<u32>> = raw_updates
        .lines()
        .map(|line| line.split(",").flat_map(|n| n.parse()).collect())
        .collect();

    let (correct, incorrect): (Vec<Vec<u32>>, Vec<Vec<u32>>) = updates
        .into_iter()
        .partition(|update| is_right_order(&rules, update).is_none());

    let first_answer: u32 = correct
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum();
    println!("First answer: {}", first_answer);

    let second_answer: u32 = incorrect
        .into_iter()
        .map(|update| make_correct(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Second answer: {}", second_answer);
}

fn is_right_order<'a>(rules: &'a [(u32, u32)], update: &'a [u32]) -> Option<Vec<&'a (u32, u32)>> {
    let mut previous = HashSet::new();
    for n in update {
        let rule_broken: Vec<&(u32, u32)> = rules
            .iter()
            .filter(|(a, _)| a == n)
            .filter(|(_, b)| previous.contains(b))
            .collect();
        if !rule_broken.is_empty() {
            return Some(rule_broken);
        }
        previous.insert(n);
    }
    None
}

fn make_correct(rules: &[(u32, u32)], mut update: Vec<u32>) -> Vec<u32> {
    while let Some(broken_rules) = is_right_order(rules, &update.clone()) {
        for (a, b) in broken_rules {
            let i = update.iter().position(|u| u == a).unwrap();
            let j = update.iter().position(|u| u == b).unwrap();
            update.swap(i, j);
        }
    }
    update
}
