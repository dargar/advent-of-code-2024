use std::io::stdin;
use std::io::Read;

type Schema = Vec<Vec<char>>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (keys, locks): (Vec<Schema>, Vec<Schema>) = input
        .split("\n\n")
        .map(|schema| schema.lines().map(|line| line.chars().collect()).collect())
        .map(transpose)
        .partition(is_key);

    let first_answer = keys
        .iter()
        .flat_map(|key| locks.iter().filter(|lock| !overlaps(key, lock)))
        .count();
    println!("First answer: {}", first_answer);
}

fn transpose(vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![vec!['.'; vec.len()]; vec[0].len()];
    vec.iter()
        .enumerate()
        .flat_map(|(y, xs)| xs.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .for_each(|((x, y), c)| transposed[x][y] = c);
    transposed
}

fn is_key(schema: &Vec<Vec<char>>) -> bool {
    schema.iter().flat_map(|row| row.first()).all(|c| *c == '#')
}

fn overlaps(key: &Schema, lock: &Schema) -> bool {
    let space = key[0].len();
    let key_pins = pin_heights(key);
    let lock_pins = pin_heights(lock);
    key_pins
        .iter()
        .zip(lock_pins.iter())
        .any(|(k, l)| *k + *l > space)
}

fn pin_heights(schema: &Schema) -> Vec<usize> {
    schema
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .collect()
}
