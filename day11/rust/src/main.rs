use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

type Cache = HashMap<(u64, u64), u64>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let stones: Vec<u64> = input.split_whitespace().flat_map(str::parse).collect();
    println!("First answer: {}", answer(&stones, 25));
    println!("Second answer: {}", answer(&stones, 75));
}

fn answer(stones: &[u64], blinks: u64) -> u64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| answer_one(&mut cache, *stone, blinks))
        .sum()
}

fn answer_one(cache: &mut Cache, stone: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, blinks)) {
        return *result;
    }

    let result = change(stone)
        .into_iter()
        .map(|s| answer_one(cache, s, blinks - 1))
        .sum();
    cache.insert((stone, blinks), result);
    result
}

fn change(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let digits = digits(stone);
    if digits.len() % 2 == 0 {
        let lhs = &digits[0..digits.len() / 2];
        let rhs = &digits[digits.len() / 2..];
        return vec![undigits(lhs), undigits(rhs)];
    }

    vec![stone * 2024]
}

fn digits(mut n: u64) -> Vec<u8> {
    let mut ds = Vec::new();
    while n > 0 {
        ds.push((n % 10) as u8);
        n /= 10;
    }
    ds.into_iter().rev().collect()
}

fn undigits(ds: &[u8]) -> u64 {
    ds.iter().fold(0, |n, d| n * 10 + *d as u64)
}
