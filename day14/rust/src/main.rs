use std::io::stdin;
use std::io::Read;

type Robot = Vec<Vec<i64>>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s[2..].split(",").flat_map(|n| n.parse()).collect())
                .collect()
        })
        .collect();

    let width = 101;
    let height = 103;
    for n in 1.. {
        for robot in &mut robots {
            robot[0][0] = (robot[0][0] + robot[1][0]).rem_euclid(width);
            robot[0][1] = (robot[0][1] + robot[1][1]).rem_euclid(height);
        }

        if n == 100 {
            let mut quadrants = vec![0; 4];
            for robot in &robots {
                let x = robot[0][0];
                let y = robot[0][1];
                if x != width / 2 && y != height / 2 {
                    let ix = x / ((width + 1) / 2);
                    let iy = y / ((height + 1) / 2);
                    let quadrant = ix + iy * 2;
                    quadrants[quadrant as usize] += 1;
                }
            }
            let first_answer: usize = quadrants.into_iter().product();
            println!("First answer: {}", first_answer);
        }

        {
            let mut grid = vec![vec![0; width as usize]; height as usize];
            for robot in &robots {
                let x = robot[0][0];
                let y = robot[0][1];
                grid[y as usize][x as usize] += 1;
            }
            if grid
                .iter()
                .any(|row| row.windows(30).any(|w| w.iter().all(|n| *n != 0)))
            {
                println!("Second answer: {n}");
                for y in 0..height {
                    for x in 0..width {
                        if grid[y as usize][x as usize] > 0 {
                            print!("{}", grid[y as usize][x as usize]);
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                break;
            }
        }
    }
}
