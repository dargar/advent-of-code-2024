use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let first_answer = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(str::parse::<i32>)
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let diffs = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();
            let all_increasing = diffs.iter().all(|d| *d < 0);
            let all_decreasing = diffs.iter().all(|d| *d > 0);
            let level_diff = diffs.iter().map(|d| d.abs()).all(|d| 1 <= d && d <= 3);
            (all_increasing || all_decreasing) && level_diff
        })
        .count();
    println!("First answer: {}", first_answer);

    let second_answer = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(str::parse::<i32>)
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let mut ok = false;
            for i in 0..report.len() {
                let report = report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, l)| l)
                    .collect::<Vec<_>>();
                let diffs = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();
                let all_increasing = diffs.iter().all(|d| *d < 0);
                let all_decreasing = diffs.iter().all(|d| *d > 0);
                let level_diff = diffs.iter().map(|d| d.abs()).all(|d| 1 <= d && d <= 3);
                ok |= (all_increasing || all_decreasing) && level_diff;
            }
            ok
        })
        .count();
    println!("Second answer: {}", second_answer);
}
