use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers: HashMap<&str, i64> = registers
        .lines()
        .map(|line| {
            let (r, n) = line.split_once(": ").unwrap();
            (r.split(" ").last().unwrap(), n.parse().unwrap())
        })
        .collect();
    let program: Vec<i64> = program
        .split(": ")
        .last()
        .unwrap()
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let first_answer = run(&program, registers.clone())
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("First answer: {first_answer}");
    assert_eq!(first_answer, "2,1,4,7,6,0,3,1,4");

    let second_answer = part2(&program, &registers, 0).unwrap();
    println!("Second answer: {second_answer}");
}

fn part2(program: &[i64], registers: &HashMap<&str, i64>, i: i64) -> Option<i64> {
    for n in (i..).take(8) {
        let mut rs = registers.clone();
        rs.insert("A", n);
        let output = run(program, rs);
        if program == output {
            return Some(n);
        } else if program[(program.len() - output.len())..] == output {
            if let Some(r) = part2(program, registers, n * 8) {
                return Some(r);
            }
        }
    }
    None
}

fn run(program: &[i64], mut registers: HashMap<&str, i64>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let mut ip = 0;
    while ip < program.len() - 1 {
        let opcode = program[ip];
        let operand = program[ip + 1];
        match opcode {
            // adv - division
            0 => {
                let numerator = *registers.get(&"A").unwrap();
                let combo_operand = combo(operand, &registers);
                let result = numerator / 2i64.pow(combo_operand as u32);
                registers.insert("A", result);
                ip += 2;
            }
            // bxl - bitwise XOR
            1 => {
                let a = *registers.get(&"B").unwrap();
                let b = operand;
                let result = a ^ b;
                registers.insert("B", result);
                ip += 2;
            }
            // bst - modulo
            2 => {
                let combo_operand = combo(operand, &registers);
                let result = combo_operand % 8;
                registers.insert("B", result);
                ip += 2;
            }
            // jnz - jump not zero
            3 => {
                let a = *registers.get(&"A").unwrap();
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            // bxc - bitwise XOR
            4 => {
                let b = *registers.get(&"B").unwrap();
                let c = *registers.get(&"C").unwrap();
                let result = b ^ c;
                registers.insert("B", result);
                ip += 2;
            }
            // out - output
            5 => {
                let combo_operand = combo(operand, &registers);
                let result = combo_operand % 8;
                output.push(result);
                ip += 2;
            }
            // bdv - division
            6 => {
                let numerator = *registers.get(&"A").unwrap();
                let combo_operand = combo(operand, &registers);
                let result = numerator / 2i64.pow(combo_operand as u32);
                registers.insert("B", result);
                ip += 2;
            }
            // cdv - division
            7 => {
                let numerator = *registers.get(&"A").unwrap();
                let combo_operand = combo(operand, &registers);
                let result = numerator / 2i64.pow(combo_operand as u32);
                registers.insert("C", result);
                ip += 2;
            }
            _ => unreachable!(),
        }
    }
    output
}

fn combo(operand: i64, registers: &HashMap<&str, i64>) -> i64 {
    match operand {
        0..=3 => operand,
        4 => *registers.get(&"A").unwrap(),
        5 => *registers.get(&"B").unwrap(),
        6 => *registers.get(&"C").unwrap(),
        _ => unreachable!(),
    }
}
