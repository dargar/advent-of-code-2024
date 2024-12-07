use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let equations: Vec<(isize, Vec<isize>)> = input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let result = lhs.parse().unwrap();
            let numbers = rhs.split_whitespace().flat_map(str::parse).collect();
            (result, numbers)
        })
        .collect();

    let first_answer: isize = equations
        .iter()
        .filter(|(result, numbers)| part1(numbers).into_iter().any(|n| n == *result))
        .map(|(result, _)| result)
        .sum();
    println!("First answer: {}", first_answer);

    let second_answer: isize = equations
        .iter()
        .filter(|(result, numbers)| part2(numbers).into_iter().any(|n| n == *result))
        .map(|(result, _)| result)
        .sum();
    println!("Second answer: {}", second_answer);
}

fn part1(numbers: &[isize]) -> Vec<isize> {
    if numbers.is_empty() {
        return Vec::new();
    }

    part1_ltr(numbers[0], &numbers[1..])
}

fn part1_ltr(n: isize, ns: &[isize]) -> Vec<isize> {
    if ns.is_empty() {
        return vec![n];
    }

    let (head, rest) = ns.split_at(1);
    let mut result = Vec::new();
    result.extend(part1_ltr(n + head[0], rest));
    result.extend(part1_ltr(n * head[0], rest));
    result
}

fn part2(numbers: &[isize]) -> Vec<isize> {
    if numbers.is_empty() {
        return Vec::new();
    }

    part2_ltr(numbers[0], &numbers[1..])
}

fn part2_ltr(n: isize, ns: &[isize]) -> Vec<isize> {
    if ns.is_empty() {
        return vec![n];
    }

    let (head, rest) = ns.split_at(1);
    let mut result = Vec::new();
    result.extend(part2_ltr(n + head[0], rest));
    result.extend(part2_ltr(n * head[0], rest));
    result.extend(part2_ltr(concat(n, head[0]), rest));
    result
}

fn concat(a: isize, b: isize) -> isize {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    let mut c = b;
    let mut digits = Vec::new();
    while c > 0 {
        digits.push(c % 10);
        c /= 10;
    }
    digits.into_iter().rev().fold(a, |acc, n| acc * 10 + n)
}
