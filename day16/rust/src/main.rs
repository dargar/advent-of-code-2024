use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

type Pos = (i64, i64);
type Map = HashMap<Pos, char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
                .map(move |(x, c)| ((x as i64, y as i64), c))
        })
        .collect();
    let start = *map
        .iter()
        .filter(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .next()
        .unwrap();
    let end = *map
        .iter()
        .filter(|(_, c)| **c == 'E')
        .map(|(p, _)| p)
        .next()
        .unwrap();
    let mut queue = VecDeque::from([(start, (1, 0), 0, Vec::new())]);
    let mut visited = HashMap::new();
    let mut scores = Vec::new();
    while let Some((pos, dir, score, mut path)) = queue.pop_front() {
        path.push(pos);

        if pos == end {
            scores.push((score, path));
            continue;
        }

        if visited.get(&(pos, dir)).is_some_and(|s| score > *s) {
            continue;
        }

        visited.insert((pos, dir), score);

        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if map.contains_key(&next) {
            queue.push_back((next, dir, score + 1, path.clone()));
        }

        let dirs = HashMap::from([
            ((1, 0), vec![(0, 1), (0, -1)]),
            ((-1, 0), vec![(0, 1), (0, -1)]),
            ((0, 1), vec![(1, 0), (-1, 0)]),
            ((0, -1), vec![(1, 0), (-1, 0)]),
        ]);
        queue.extend(
            dirs.get(&dir)
                .unwrap()
                .iter()
                .filter(|d| map.contains_key(&(pos.0 + d.0, pos.1 + d.1)))
                .map(|d| (pos, *d, score + 1_000, path.clone())),
        );
    }
    let first_answer = scores.iter().map(|(score, _)| score).min().unwrap();
    let second_answer = scores
        .iter()
        .filter(|(score, _)| score == first_answer)
        .flat_map(|(_, path)| path)
        .cloned()
        .collect::<HashSet<Pos>>()
        .len();
    println!("First answer: {first_answer}");
    println!("Second answer: {second_answer}");
}
