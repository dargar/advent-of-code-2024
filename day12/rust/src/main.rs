use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

type Grid = HashMap<(i64, i64), char>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c))
        })
        .collect();
    let (max_x, max_y) = *grid.keys().max().unwrap();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if visited.contains(&(x, y)) {
                continue;
            }

            let id = *grid.get(&(x, y)).unwrap();
            let mut region = HashSet::new();
            let mut queue = VecDeque::from([(x, y)]);
            while let Some((x, y)) = queue.pop_front() {
                if !region.insert((x, y)) {
                    continue;
                }
                queue.extend(
                    vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                        .into_iter()
                        .filter(|p| !region.contains(p))
                        .filter(|p| grid.get(p).is_some_and(|c| *c == id))
                        .collect::<Vec<_>>(),
                )
            }
            visited.extend(region.clone());
            regions.push(region);
        }
    }

    let first_answer: usize = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter = {
                let mut p = 0;
                for r @ (x, y) in region {
                    let id = *grid.get(r).unwrap();
                    p += vec![(*x, *y - 1), (*x, *y + 1), (*x - 1, *y), (*x + 1, *y)]
                        .into_iter()
                        .filter(|p| grid.get(p).is_none_or(|c| *c != id))
                        .count();
                }
                p
            };
            area * perimeter
        })
        .sum();
    println!("First answer: {first_answer}");

    let second_answer: usize = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let sides = {
                let subdivided_region: HashSet<(i64, i64)> = region
                    .iter()
                    .cloned()
                    .flat_map(|(x, y)| {
                        let xx = x * 2;
                        let yy = y * 2;
                        vec![(xx, yy), (xx + 1, yy), (xx, yy + 1), (xx + 1, yy + 1)]
                    })
                    .collect();
                subdivided_region
                    .iter()
                    .map(|(x, y)| {
                        let mut ns = 0;
                        for yy in (*y - 1)..=(*y + 1) {
                            for xx in (*x - 1)..=(*x + 1) {
                                if !subdivided_region.contains(&(xx, yy)) {
                                    ns += 1;
                                }
                            }
                        }
                        ns
                    })
                    .filter(|ns| *ns == 5 || *ns == 4 || *ns == 1)
                    .count()
            };
            area * sides
        })
        .sum();
    println!("Second answer: {second_answer}");
}
