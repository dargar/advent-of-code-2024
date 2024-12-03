use regex::Regex;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    let first_answer: i64 = re
        .captures_iter(&input)
        .map(|capture| {
            let a: i64 = capture.name("a").unwrap().as_str().parse().unwrap();
            let b: i64 = capture.name("b").unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum();
    println!("First answer: {}", first_answer);

    let re = Regex::new(r"((?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))")
        .unwrap();
    let (second_answer, _): (i64, bool) =
        re.captures_iter(&input)
            .fold((0, true), |(sum, accept), capture| {
                if capture.name("do").is_some() {
                    (sum, true)
                } else if capture.name("dont").is_some() {
                    (sum, false)
                } else if accept {
                    let a: i64 = capture.name("a").unwrap().as_str().parse().unwrap();
                    let b: i64 = capture.name("b").unwrap().as_str().parse().unwrap();
                    (sum + a * b, accept)
                } else {
                    (sum, accept)
                }
            });
    println!("Second answer: {}", second_answer);
}
