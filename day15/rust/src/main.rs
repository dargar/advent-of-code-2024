use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

type Warehouse = HashMap<(i64, i64), char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (raw_warehouse, raw_moves) = input.split_once("\n\n").unwrap();
    let warehouse: Warehouse = raw_warehouse
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c))
        })
        .collect();

    println!(
        "First answer: {}",
        first_answer(warehouse.clone(), raw_moves)
    );
    println!(
        "Second answer: {}",
        second_answer(warehouse.clone(), raw_moves)
    );
}

fn first_answer(mut warehouse: Warehouse, raw_moves: &str) -> i64 {
    for (dx, dy) in raw_moves.chars().filter_map(delta) {
        let (x, y) = *warehouse
            .iter()
            .filter(|(_, c)| **c == '@')
            .map(|(p, _)| p)
            .next()
            .unwrap();

        let mut boxes_ahead = 0;
        while warehouse
            .get(&(x + dx * (boxes_ahead + 1), y + dy * (boxes_ahead + 1)))
            .is_some_and(|c| *c == 'O')
        {
            boxes_ahead += 1;
        }

        let nx = x + dx * (boxes_ahead + 1);
        let ny = y + dy * (boxes_ahead + 1);
        if warehouse.get(&(nx, ny)).is_some_and(|c| *c != '#') {
            warehouse.insert((x, y), '.');
            for n in 1..=boxes_ahead {
                warehouse.insert((x + dx * n, y + dy * n), '.');
            }
            for n in 1..=boxes_ahead {
                warehouse.insert((x + dx * (n + 1), y + dy * (n + 1)), 'O');
            }
            warehouse.insert((x + dx, y + dy), '@');
        }
    }

    warehouse
        .iter()
        .filter(|(_, c)| **c == 'O')
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

fn second_answer(warehouse: Warehouse, raw_moves: &str) -> i64 {
    let mut warehouse: Warehouse = warehouse
        .into_iter()
        .flat_map(|((x, y), c)| {
            let cc = match c {
                '#' => vec!['#', '#'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                'O' => vec!['[', ']'],
                _ => unreachable!(),
            };
            cc.into_iter()
                .enumerate()
                .map(move |(i, c)| ((2 * x + i as i64, y), c))
        })
        .collect();
    for (dx, dy) in raw_moves.chars().filter_map(delta) {
        let (x, y) = *warehouse
            .iter()
            .filter(|(_, c)| **c == '@')
            .map(|(p, _)| p)
            .next()
            .unwrap();
        if let Some(ms) = moves(&warehouse, (x, y), (dx, dy)) {
            let mut bs = HashMap::new();
            for m in ms {
                let c = warehouse.insert(m, '.').unwrap();
                bs.insert(m, c);
            }
            warehouse.extend(bs.into_iter().map(|((x, y), c)| ((x + dx, y + dy), c)));
        }
    }

    warehouse
        .iter()
        .filter(|(_, c)| **c == '[')
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

fn delta(c: char) -> Option<(i64, i64)> {
    match c {
        '^' => Some((0, -1)),
        'v' => Some((0, 1)),
        '<' => Some((-1, 0)),
        '>' => Some((1, 0)),
        _ => None,
    }
}

fn moves(
    warehouse: &Warehouse,
    (x, y): (i64, i64),
    (dx, dy): (i64, i64),
) -> Option<HashSet<(i64, i64)>> {
    let nx = x + dx;
    let ny = y + dy;
    match warehouse.get(&(nx, ny)) {
        Some('#') => None,
        Some('.') => Some(HashSet::from([(x, y)])),
        Some('[') => {
            if dy == 0 {
                let ms = moves(warehouse, (nx, ny), (dx, dy))?;
                let mut result = HashSet::from([(x, y)]);
                result.extend(ms);
                Some(result)
            } else {
                let lhs = moves(warehouse, (nx, ny), (dx, dy))?;
                let rhs = moves(warehouse, (nx + 1, ny), (dx, dy))?;
                let mut result = HashSet::from([(x, y)]);
                result.extend(lhs);
                result.extend(rhs);
                Some(result)
            }
        }
        Some(']') => {
            if dy == 0 {
                let ms = moves(warehouse, (nx, ny), (dx, dy))?;
                let mut result = HashSet::from([(x, y)]);
                result.extend(ms);
                Some(result)
            } else {
                let lhs = moves(warehouse, (nx - 1, ny), (dx, dy))?;
                let rhs = moves(warehouse, (nx, ny), (dx, dy))?;
                let mut result = HashSet::from([(x, y)]);
                result.extend(lhs);
                result.extend(rhs);
                Some(result)
            }
        }
        _ => unreachable!(),
    }
}
