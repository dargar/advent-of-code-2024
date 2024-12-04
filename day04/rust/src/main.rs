use std::io::stdin;
use std::io::Read;

type Grid = Vec<Vec<char>>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let left_to_right_grid: Grid = grid.clone();
    let right_to_left_grid: Grid = grid
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();
    let top_to_bottom_grid: Grid = {
        assert_eq!(grid.len(), grid[0].len());
        let mut g = vec![vec!['.'; grid.len()]; grid.len()];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                g[x][y] = grid[y][x];
            }
        }
        g
    };
    let bottom_to_top_grid: Grid = top_to_bottom_grid
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();
    let top_left_to_bottom_right_grid: Grid = {
        let mut g = Vec::new();
        for mut y in (0..grid.len()).rev() {
            let mut x = 0;
            let mut cs = Vec::new();
            while y < grid.len() && x < grid.len() {
                cs.push(grid[y][x]);
                y += 1;
                x += 1;
            }
            g.push(cs);
        }
        for mut x in 1..grid.len() {
            let mut y = 0;
            let mut cs = Vec::new();
            while y < grid.len() && x < grid.len() {
                cs.push(grid[y][x]);
                y += 1;
                x += 1;
            }
            g.push(cs);
        }
        g
    };
    let bottom_right_to_top_left_grid: Grid = top_left_to_bottom_right_grid
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();
    let bottom_left_to_top_right = {
        let mut g = Vec::new();
        for mut y in (0..grid.len()).rev() {
            let mut x = 0;
            let mut cs = Vec::new();
            while y < grid.len() && x < grid.len() {
                cs.push(grid[y][x]);
                if y == 0 {
                    break;
                }
                y -= 1;
                x += 1;
            }
            g.push(cs);
        }
        for mut x in 1..grid.len() {
            let mut y = grid.len() - 1;
            let mut cs = Vec::new();
            while y < grid.len() && x < grid.len() {
                cs.push(grid[y][x]);
                if y == 0 {
                    break;
                }
                y -= 1;
                x += 1;
            }
            g.push(cs);
        }
        g
    };
    let top_right_to_bottom_left_grid: Grid = bottom_left_to_top_right
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();

    let grids = vec![
        left_to_right_grid,
        right_to_left_grid,
        top_to_bottom_grid,
        bottom_to_top_grid,
        top_left_to_bottom_right_grid,
        bottom_right_to_top_left_grid,
        bottom_left_to_top_right,
        top_right_to_bottom_left_grid,
    ];
    let first_answer: usize = grids.iter().map(xmas).sum();
    println!("First answer: {}", first_answer);

    let second_answer = x_mas(&grid);
    println!("Second answer: {}", second_answer);
}

fn xmas(grid: &Grid) -> usize {
    let mut result = 0;
    for row in grid {
        for w in row.windows(4) {
            if w[0] == 'X' && w[1] == 'M' && w[2] == 'A' && w[3] == 'S' {
                result += 1;
            }
        }
    }
    result
}

fn x_mas(grid: &Grid) -> usize {
    let mut result = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid.len() - 1 {
            let mut a = String::new();
            {
                let y = y - 1;
                let x = x - 1;
                for n in 0..3 {
                    a.push(grid[y + n][x + n]);
                }
            }
            let mut b = String::new();
            {
                let mut y = y + 1;
                let mut x = x - 1;
                for _ in 0..3 {
                    b.push(grid[y][x]);
                    if y == 0 {
                        break;
                    }
                    y -= 1;
                    x += 1;
                }
            }
            if (a == "MAS" || a == "SAM") && (b == "MAS" || b == "SAM") {
                result += 1;
            }
        }
    }
    result
}
