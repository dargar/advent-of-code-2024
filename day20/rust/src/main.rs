use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

type Pos = (i64, i64);
type Grid = HashMap<Pos, char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
                .map(move |(x, c)| ((x as i64, y as i64), c))
        })
        .collect();

    let first_answer = first_answer(&grid);
    println!("First answer: {first_answer}");

    let second_answer = second_answer(&grid);
    println!("Second answer: {second_answer}");
}

fn first_answer(grid: &Grid) -> usize {
    let path = path(grid);
    let two_step_cheats: Vec<(Pos, Pos, i64)> = path
        .iter()
        .flat_map(|((x, y), steps)| {
            let no_clip = vec![
                vec![(*x + 1, *y), (*x + 2, *y)],
                vec![(*x - 1, *y), (*x - 2, *y)],
                vec![(*x, *y + 1), (*x, *y + 2)],
                vec![(*x, *y - 1), (*x, *y - 2)],
            ];
            no_clip
                .into_iter()
                .filter(|ps| !grid.contains_key(&ps[0]))
                .filter(|ps| grid.contains_key(&ps[1]))
                .map(|ps| {
                    let s = path.get(&ps[1]).unwrap();
                    ((*x, *y), ps[1], s - *steps - 2)
                })
        })
        .filter(|(_, _, s)| *s > 0)
        .collect();
    two_step_cheats.iter().filter(|(_, _, s)| *s >= 100).count()
}

fn second_answer(grid: &Grid) -> usize {
    let path = path(grid);
    let cheats: Vec<(Pos, Pos, i64)> = path
        .iter()
        .flat_map(|(p0, steps)| {
            no_clip_reachable(grid, *p0, 20)
                .into_iter()
                .filter(|(p1, _)| grid.contains_key(p1))
                .map(|(p1, steps_taken)| {
                    let s1 = *path.get(&p1).unwrap();
                    (*p0, p1, s1 - *steps - steps_taken)
                })
        })
        .filter(|(_, _, s)| *s > 0)
        .collect();
    cheats.iter().filter(|(_, _, s)| *s >= 100).count()
}

fn path(grid: &Grid) -> HashMap<Pos, i64> {
    let start: Pos = *grid
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .unwrap();
    let end: Pos = *grid
        .iter()
        .find(|(_, c)| **c == 'E')
        .map(|(p, _)| p)
        .unwrap();

    let mut path: HashMap<Pos, i64> = HashMap::new();
    let mut pos = start;
    for steps in 0.. {
        path.insert(pos, steps);

        if pos == end {
            break;
        }

        pos = vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ]
        .into_iter()
        .find(|p| grid.contains_key(p) && !path.contains_key(p))
        .unwrap();
    }
    path
}

fn no_clip_reachable(grid: &Grid, pos: Pos, steps: i64) -> HashMap<Pos, i64> {
    let max_x = 1 + *grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = 1 + *grid.keys().map(|(_, y)| y).max().unwrap();

    let mut visited = HashMap::new();
    let mut queue = VecDeque::from([(pos, steps)]);
    while let Some((p, s)) = queue.pop_front() {
        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, steps - s);

        let dirs = vec![
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ];
        if s > 0 {
            queue.extend(
                dirs.into_iter()
                    .filter(|(x, _)| 0 <= *x && *x <= max_x)
                    .filter(|(_, y)| 0 <= *y && *y <= max_y)
                    .filter(|p| !visited.contains_key(p))
                    .map(|p| (p, s - 1)),
            );
        }
    }
    visited
}
