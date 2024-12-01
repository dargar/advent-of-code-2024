use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut ids = line.split_whitespace().flat_map(str::parse);
        insert_sorted(&mut lhs, ids.next().unwrap());
        insert_sorted(&mut rhs, ids.next().unwrap());
    }
    let first_answer: i32 = lhs.iter().zip(rhs.iter()).map(|(l, r)| (l - r).abs()).sum();
    println!("First answer: {}", first_answer);

    let mut rhs_freqs = HashMap::new();
    for r in rhs {
        *rhs_freqs.entry(r).or_insert(0) += 1;
    }
    let second_answer: i32 = lhs.iter().map(|l| l * rhs_freqs.get(l).unwrap_or(&0)).sum();
    println!("Second answer: {}", second_answer);
}

fn insert_sorted<T: PartialOrd>(xs: &mut Vec<T>, y: T) {
    let index = xs.partition_point(|x| *x <= y);
    xs.insert(index, y);
}
