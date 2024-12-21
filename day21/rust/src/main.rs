use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

type Grid = HashMap<(i64, i64), char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let num_pad: Grid = "
        789
        456
        123
         0A
        "
    .lines()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(move |(x, c)| ((x as i64, y as i64), c))
    })
    .collect();

    let dir_pad: Grid = "
         ^A
        <v>
        "
    .lines()
    .enumerate()
    .flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(move |(x, c)| ((x as i64, y as i64), c))
    })
    .collect();

    let first_answer = solve(&num_pad, &dir_pad, &input, 2);
    println!("First answer: {first_answer}");

    let second_answer = solve(&num_pad, &dir_pad, &input, 25);
    println!("Second answer: {second_answer}");
}

fn solve(num_pad: &Grid, dir_pad: &Grid, codes: &str, num_robots: usize) -> usize {
    let mut cache = HashMap::new();
    let mut results = Vec::new();
    for line in codes.lines() {
        let ns = button_sequences(num_pad, line);
        let shortest_sequence = ns
            .iter()
            .map(|n| shortest_sequence(dir_pad, n, num_robots, &mut cache))
            .min()
            .unwrap();
        let numeric_code = line
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        results.push(shortest_sequence * numeric_code);
    }
    results.into_iter().sum()
}

fn shortest_sequence(
    grid: &Grid,
    code: &str,
    n: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if n == 0 {
        return code.len();
    }

    if let Some(result) = cache.get(&(code.to_string(), n)) {
        return *result;
    }

    let result = code
        .split_inclusive('A')
        .flat_map(|sub| {
            button_sequences(grid, sub)
                .into_iter()
                .map(|button_sequence| shortest_sequence(grid, &button_sequence, n - 1, cache))
                .min()
        })
        .sum();
    cache.insert((code.to_string(), n), result);
    result
}

fn button_sequences(grid: &Grid, code: &str) -> Vec<String> {
    let mut current_button = 'A';
    let mut result = Vec::new();
    for target_button in code.chars() {
        let paths = shortest_paths(grid, current_button, target_button);
        if result.is_empty() {
            result = paths;
        } else {
            result = result
                .drain(..)
                .flat_map(|prefix| paths.iter().map(move |suffix| prefix.clone() + suffix))
                .collect();
        }
        current_button = target_button;
    }
    result
}

fn shortest_paths(grid: &Grid, start: char, end: char) -> Vec<String> {
    let (start_pos, _) = grid.iter().find(|(_, c)| **c == start).unwrap();
    let (end_pos, _) = grid.iter().find(|(_, c)| **c == end).unwrap();
    let max_steps = (start_pos.0 - end_pos.0).abs() + (start_pos.1 - end_pos.1).abs();

    let mut queue = VecDeque::from([(*start_pos, String::new())]);
    let mut result = Vec::new();
    while let Some((curr, mut dirs)) = queue.pop_front() {
        if curr == *end_pos {
            dirs.push('A');
            result.push(dirs);
            continue;
        }

        queue.extend(
            vec!['^', '<', 'v', '>']
                .into_iter()
                .map(|d| {
                    let (dx, dy) = match d {
                        '^' => (0, -1),
                        '<' => (-1, 0),
                        'v' => (0, 1),
                        '>' => (1, 0),
                        _ => unreachable!(),
                    };
                    let next_pos = (curr.0 + dx, curr.1 + dy);
                    let mut next_dirs = dirs.clone();
                    next_dirs.push(d);
                    (next_pos, next_dirs)
                })
                .filter(|(next_pos, _)| grid.contains_key(next_pos))
                .filter(|(_, ds)| ds.len() <= max_steps as usize),
        );
    }
    result
}
