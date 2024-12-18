use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid_size = 71;
    let iterations = 1024;
    let bytes: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(",").flat_map(|s| s.parse()).collect())
        .collect();
    let mut grid: HashSet<&Vec<i64>> = bytes.iter().take(iterations).collect();

    let mut path = find_path(&grid, grid_size);
    println!("First answer: {}", path.clone().unwrap().len() - 1);

    for b in bytes.iter().skip(iterations) {
        grid.insert(b);
        if path.clone().is_some_and(|p| !p.contains(b)) {
            continue;
        }
        path = find_path(&grid, grid_size);
        if path.is_none() {
            println!("Second answer: {},{}", b[0], b[1]);
            break;
        }
    }
}

fn find_path(grid: &HashSet<&Vec<i64>>, grid_size: i64) -> Option<Vec<Vec<i64>>> {
    let mut queue = VecDeque::from(vec![(vec![0, 0], Vec::new())]);
    let mut visited = HashSet::new();
    while let Some((pos, mut path)) = queue.pop_front() {
        path.push(pos.clone());

        if pos == vec![grid_size - 1, grid_size - 1] {
            return Some(path);
        }

        if !visited.insert(pos.clone()) {
            continue;
        }

        queue.extend(
            vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]]
                .into_iter()
                .map(|dir| {
                    dir.into_iter()
                        .zip(pos.iter())
                        .map(|(d, p)| d + p)
                        .collect::<Vec<_>>()
                })
                .filter(|p| p.iter().all(|n| 0 <= *n && *n < grid_size))
                .filter(|p| !grid.contains(p))
                .filter(|p| !visited.contains(p))
                .map(|p| (p, path.clone())),
        );
    }
    None
}
