use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let n = 2_000;
    let secrets: Vec<Vec<u64>> = input
        .lines()
        .flat_map(|line| line.parse())
        .map(|initial_secret| secrets(initial_secret, n))
        .collect();
    let first_answer: u64 = secrets.iter().map(|xs| *xs.last().unwrap()).sum();
    println!("First answer: {first_answer}");

    let prices: Vec<Vec<u64>> = secrets
        .iter()
        .map(|ss| ss.iter().map(|s| *s % 10).collect())
        .collect();
    let changes: Vec<Vec<i64>> = prices
        .iter()
        .map(|ss| {
            ss.windows(2)
                .map(|ns| ns[1] as i64 - ns[0] as i64)
                .collect()
        })
        .collect();
    let best_sequences: Vec<HashMap<Vec<i64>, u64>> = changes
        .iter()
        .enumerate()
        .map(|(i, cs)| {
            cs.windows(4)
                .enumerate()
                .map(|(j, xs)| {
                    let price = prices[i][j + 4];
                    (xs, price)
                })
                .fold(HashMap::new(), |mut map, (seq, price)| {
                    map.entry(seq.to_vec()).or_insert(price);
                    map
                })
        })
        .collect();
    let mut overall_best_sequences: HashMap<Vec<i64>, u64> = HashMap::new();
    for map in best_sequences {
        for (seq, price) in map {
            overall_best_sequences
                .entry(seq)
                .and_modify(|p| *p += price)
                .or_insert(price);
        }
    }
    let second_answer = overall_best_sequences.values().max().unwrap();
    println!("Second answer: {second_answer}");
}

fn secrets(secret: u64, n: usize) -> Vec<u64> {
    let mut secrets = vec![secret];
    for _ in 0..n {
        let previous_secret = *secrets.last().unwrap();
        secrets.push(next_secret(previous_secret));
    }
    secrets
}

fn next_secret(mut secret: u64) -> u64 {
    secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}
