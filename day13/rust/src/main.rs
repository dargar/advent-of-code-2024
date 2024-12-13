use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut first_answer = 0;
    let mut second_answer = 0;
    for machine in input.split("\n\n") {
        let ns: Vec<Vec<i64>> = machine
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
                    .flat_map(|s| s.parse())
                    .collect()
            })
            .collect();

        let p1 = part1(&ns);
        let p2 = part2(&ns);
        first_answer += p1;
        second_answer += p2;
    }
    println!("First answer: {first_answer}");
    println!("Second answer: {second_answer}");
}

fn part1(ns: &[Vec<i64>]) -> i64 {
    let ax = ns[0][0];
    let ay = ns[0][1];
    let bx = ns[1][0];
    let by = ns[1][1];
    let px = ns[2][0];
    let py = ns[2][1];
    let mut tokens = 0;
    for a in 0.. {
        if ax * a > px || ay * a > py {
            break;
        }
        for b in 0.. {
            let x = ax * a + bx * b;
            let y = ay * a + by * b;
            if x == px && y == py {
                tokens += a * 3 + b;
                break;
            } else if x > px || y > py {
                break;
            }
        }
    }
    tokens
}

fn part2(ns: &[Vec<i64>]) -> i64 {
    let ax = ns[0][0];
    let ay = ns[0][1];
    let bx = ns[1][0];
    let by = ns[1][1];
    let px = ns[2][0] + 10000000000000;
    let py = ns[2][1] + 10000000000000;
    let m = if ax >= ay { lcm(ax, ay) } else { lcm(ay, ax) };
    let m1 = m / ax;
    let m2 = m / ay;
    let b = (m1 * px + (-m2) * py) / (m1 * bx + (-m2) * by);
    let p2 = px - bx * b;
    let p3 = py - by * b;
    if p2 % ax != 0 {
        return 0;
    }
    if p3 % ay != 0 {
        return 0;
    }
    let a1 = p2 / ax;
    let a2 = p3 / ay;
    if a1 == a2 {
        a1 * 3 + b
    } else {
        0
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a > b {
        (a * b).abs() / gcd(a, b)
    } else {
        (a * b).abs() / gcd(b, a)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[test]
fn lcm_test() {
    assert_eq!(lcm(21, 6), 42);
    assert_eq!(lcm(6, 21), 42);
    assert_eq!(lcm(94, 34), 1598)
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(48, 18), 6);
}
