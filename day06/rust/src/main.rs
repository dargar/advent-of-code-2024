use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

type Grid = HashMap<(isize, isize), char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    let path = run(&grid).unwrap();
    let first_answer = path.len();
    println!("First answer: {}", first_answer);

    let second_answer = path
        .into_iter()
        .filter(|pos| grid.get(&pos).is_some_and(|d| *d != '^'))
        .map(|pos| {
            let mut g = grid.clone();
            g.insert(pos, '#');
            g
        })
        .filter(|g| run(&g).is_none())
        .count();
    println!("Second answer: {}", second_answer);
}

fn run(grid: &Grid) -> Option<HashSet<(isize, isize)>> {
    let ((mut x, mut y), mut d): ((isize, isize), char) = grid
        .iter()
        .find(|(_, c)| **c == '^')
        .map(|(pos, d)| (pos.clone(), d.clone()))
        .unwrap()
        .clone();
    let mut seen = HashSet::new();
    let mut loop_detection = HashSet::new();
    loop {
        if !loop_detection.insert((x, y, d)) {
            return None;
        }

        seen.insert((x, y));

        let (nx, ny) = match d {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };
        if let Some(c) = grid.get(&(nx, ny)) {
            if *c == '#' {
                d = match d {
                    '^' => '>',
                    'v' => '<',
                    '<' => '^',
                    '>' => 'v',
                    _ => unreachable!(),
                };
            } else {
                x = nx;
                y = ny;
            }
        } else {
            return Some(seen);
        }
    }
}
