use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

type Position = (i64, i64);

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let height_map: HashMap<Position, i32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c.to_digit(10).unwrap() as i32))
        })
        .collect();

    let trail_heads: Vec<Position> = height_map
        .iter()
        .filter(|(_, h)| **h == 0)
        .map(|(p, _)| p)
        .cloned()
        .collect();

    let first_answer: usize = trail_heads
        .iter()
        .map(|trail_head| score(&height_map, trail_head))
        .sum();
    println!("First answer: {first_answer}");

    let second_answer: usize = trail_heads
        .iter()
        .map(|trail_head| rating(&height_map, trail_head))
        .sum();
    println!("Second answer: {second_answer}");
}

fn score(height_map: &HashMap<Position, i32>, trail_head: &Position) -> usize {
    let mut frontier: VecDeque<Position> = VecDeque::from([*trail_head]);
    let mut visited = HashSet::new();
    let mut heights = HashSet::new();
    while let Some(pos) = frontier.pop_front() {
        if !visited.insert(pos) {
            continue;
        }

        let current_height = height_map.get(&pos).unwrap();
        if *current_height == 9 {
            heights.insert(pos);
        }

        frontier.extend(
            neighbours(pos)
                .into_iter()
                .filter(|p| height_map.get(p).is_some_and(|ph| ph - current_height == 1)),
        );
    }
    heights.len()
}

fn rating(height_map: &HashMap<Position, i32>, trail_head: &Position) -> usize {
    let mut frontier: VecDeque<(Position, Vec<Position>)> =
        VecDeque::from([(*trail_head, Vec::new())]);
    let mut trails = HashSet::new();
    while let Some((pos, mut path)) = frontier.pop_front() {
        let current_height = height_map.get(&pos).unwrap();
        path.push(pos);
        if *current_height == 9 {
            trails.insert(path.clone());
        }

        frontier.extend(
            neighbours(pos)
                .into_iter()
                .filter(|p| height_map.get(p).is_some_and(|ph| ph - current_height == 1))
                .map(|p| (p, path.clone())),
        );
    }
    trails.len()
}

fn neighbours((x, y): Position) -> Vec<Position> {
    vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
}
