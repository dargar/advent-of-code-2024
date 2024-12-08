use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<((isize, isize), char)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    println!("First answer: {}", first_answer(&grid));
    println!("Second answer: {}", second_answer(&grid));
}

fn first_answer(grid: &[((isize, isize), char)]) -> usize {
    let ((max_x, max_y), _) = grid.iter().max_by_key(|(p, _)| p).unwrap();

    let mut antinodes = HashSet::new();
    for a @ ((ax, ay), ac) in grid {
        for b @ ((bx, by), bc) in grid {
            if a == b {
                continue;
            }

            if ac != bc {
                continue;
            }

            if *ac == '.' || *bc == '.' {
                continue;
            }

            let dx = ax - bx;
            let dy = ay - by;
            antinodes.insert((bx + (-dx), by + (-dy)));
        }
    }

    antinodes
        .iter()
        .filter(|(x, y)| (0 <= *x && *x <= *max_x) && (0 <= *y && *y <= *max_y))
        .count()
}

fn second_answer(grid: &[((isize, isize), char)]) -> usize {
    let (max_x, max_y) = grid.iter().map(|(p, _)| p).max().unwrap();

    let mut antinodes = HashSet::new();
    for a @ ((ax, ay), ac) in grid {
        for b @ ((bx, by), bc) in grid {
            if a == b {
                continue;
            }

            if ac != bc {
                continue;
            }

            if *ac == '.' || *bc == '.' {
                continue;
            }

            let dx = ax - bx;
            let dy = ay - by;
            for n in 0.. {
                let nx = bx + n * (-dx);
                let ny = by + n * (-dy);
                if (0 <= nx && nx <= *max_x) && (0 <= ny && ny <= *max_y) {
                    antinodes.insert((nx, ny));
                } else {
                    break;
                }
            }
        }
    }

    antinodes
        .iter()
        .filter(|(x, y)| (0 <= *x && *x <= *max_x) && (0 <= *y && *y <= *max_y))
        .count()
}
