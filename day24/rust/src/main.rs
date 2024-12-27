use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::stdin;
use std::io::Read;
use std::str::FromStr;

type Wires = BTreeMap<String, u64>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Gate {
    a: String,
    b: String,
    op: String,
    out: String,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_parts = s.split_whitespace();
        let lhs = line_parts.next().ok_or("lhs")?;
        let op = line_parts.next().ok_or("op")?;
        let rhs = line_parts.next().ok_or("rhs")?;
        let _ = line_parts.next().ok_or("->")?;
        let output = line_parts.next().ok_or("out")?;
        Ok(Gate {
            a: lhs.to_string(),
            b: rhs.to_string(),
            op: op.to_string(),
            out: output.to_string(),
        })
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input_sections = input.split("\n\n");
    let wires: Wires = input_sections
        .next()
        .into_iter()
        .flat_map(|section| {
            section
                .lines()
                .flat_map(|line| line.split_once(": "))
                .map(|(name, value)| (name.to_string(), value.parse().unwrap()))
        })
        .collect();
    let gates: Vec<Gate> = input_sections
        .next()
        .into_iter()
        .flat_map(|section| section.lines().flat_map(|line| line.parse()))
        .collect();

    println!("First answer: {}", first_answer(&gates, wires.clone()));
    // Actually, this will probably not produce the answer...
    println!("Second answer: {}", second_answer(&gates));
}

fn first_answer(gates: &[Gate], wires: Wires) -> u64 {
    to_num("z", &try_sim(gates, wires).unwrap())
}

fn second_answer(gates: &[Gate]) -> String {
    let mut stack: Vec<(Vec<Gate>, Vec<String>)> = vec![(gates.to_vec(), Vec::new())];
    while let Some((gates, mut swaps)) = stack.pop() {
        println!("swaps={swaps:?}");
        if swaps.len() > 8 {
            continue;
        }
        match test_bits(&gates) {
            Ok(()) => {
                swaps.sort();
                println!("Second answer: {} (maybe)", swaps.join(","));
            }
            Err(mut error_bits) => {
                error_bits.sort_by_key(|bit| reachable(*bit, &gates).len());
                let noks = reachable(error_bits[0], &gates);
                println!("{error_bits:?} ({}/{})", noks.len(), gates.len());
                for nok in noks {
                    if swaps.contains(&nok.out) {
                        continue;
                    }
                    for gate in &gates {
                        if swaps.contains(&gate.out) {
                            continue;
                        }
                        if nok == *gate {
                            continue;
                        }
                        let nok_pos = gates.iter().position(|g| *g == nok).unwrap();
                        let gate_pos = gates.iter().position(|g| g == gate).unwrap();
                        let mut try_gates = gates.clone();
                        try_gates[nok_pos].out = gates[gate_pos].out.clone();
                        try_gates[gate_pos].out = gates[nok_pos].out.clone();
                        let next_error_bits: Vec<usize> = match test_bits(&try_gates) {
                            Ok(_) => Vec::new(),
                            Err(xs) => xs,
                        };
                        if next_error_bits.len() < error_bits.len()
                            && !next_error_bits.contains(&error_bits[0])
                            && next_error_bits
                                .iter()
                                .filter(|b| !error_bits.contains(b))
                                .count()
                                == 0
                        {
                            let mut try_swaps = swaps.clone();
                            try_swaps.push(nok.out.clone());
                            try_swaps.push(gate.out.clone());
                            stack.push((try_gates, try_swaps));
                        }
                    }
                }
            }
        }
    }
    "".to_string()
}

fn test_bits(gates: &[Gate]) -> Result<(), Vec<usize>> {
    let output_bits = gates
        .iter()
        .filter(|gate| gate.out.starts_with("z"))
        .count();
    let mut error_bits = Vec::new();
    'bits: for bit in 0..(output_bits - 1) {
        let input_values = vec![0, 1 << bit];
        for x in &input_values {
            for y in &input_values {
                let wires = to_wires(*x, *y);
                let expected = x + y;
                match try_sim(gates, wires).map(|wires| to_num("z", &wires)) {
                    Some(actual) => {
                        let mut error = false;
                        let mask1 = 1 << bit;
                        if (actual & mask1) != (expected & mask1) {
                            error_bits.push(bit);
                            error = true;
                        }
                        let mask2 = 1 << (bit + 1);
                        if (actual & mask2) != (expected & mask2) {
                            error_bits.push(bit + 1);
                            error = true;
                        }
                        if error {
                            continue 'bits;
                        }
                    }
                    None => {
                        error_bits.push(bit);
                        continue 'bits;
                    }
                }
            }
        }
    }
    if error_bits.is_empty() {
        Ok(())
    } else {
        error_bits.dedup();
        Err(error_bits)
    }
}

fn try_sim(gates: &[Gate], mut wires: Wires) -> Option<Wires> {
    let mut unprocessed: Vec<Gate> = gates.to_vec();
    while !unprocessed.is_empty() {
        let unprocessed_count = unprocessed.len();
        unprocessed = unprocessed
            .drain(..)
            .filter_map(|gate| match (wires.get(&gate.a), wires.get(&gate.b)) {
                (Some(a), Some(b)) => {
                    let value = match gate.op.as_ref() {
                        "AND" => a & b,
                        "XOR" => a ^ b,
                        "OR" => a | b,
                        _ => unreachable!(),
                    };
                    wires.insert(gate.out, value);
                    None
                }
                _ => Some(gate),
            })
            .collect();
        if unprocessed_count == unprocessed.len() {
            return None;
        }
    }
    Some(wires)
}

fn to_num(prefix: &str, wires: &Wires) -> u64 {
    wires
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .map(|(_, value)| value)
        .rev()
        .fold(0, |bits, bit| (bits << 1) | bit)
}

fn to_wires(x: u64, y: u64) -> Wires {
    let mut result = BTreeMap::new();
    for b in 0..64 {
        let mask = 1 << b;
        {
            let name = format!("x{:02}", b);
            let value = std::cmp::min(x & mask, 1);
            result.insert(name, value);
        }
        {
            let name = format!("y{:02}", b);
            let value = std::cmp::min(y & mask, 1);
            result.insert(name, value);
        }
    }
    result
}

fn reachable(bit: usize, gates: &[Gate]) -> BTreeSet<Gate> {
    let mut stack = vec![format!("z{bit:02}")];
    let mut reachable = BTreeSet::new();
    while let Some(output) = stack.pop() {
        for gate in gates {
            if gate.out == *output && !reachable.contains(gate) {
                reachable.insert(gate.clone());
                stack.push(gate.a.clone());
                stack.push(gate.b.clone());
            }
        }
    }
    reachable
}
