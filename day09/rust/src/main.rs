use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let map: Vec<u32> = input.trim().chars().flat_map(|c| c.to_digit(10)).collect();

    println!("First answer: {}", first_answer(&map));
    println!("Second answer: {}", second_answer(&map));
}

fn first_answer(map: &[u32]) -> usize {
    let mut m = Vec::new();
    for (i, x) in map.chunks(2).enumerate() {
        m.extend(vec![Some(i); x[0] as usize]);
        if x.len() > 1 {
            m.extend(vec![None; x[1] as usize]);
        }
    }

    let mut l = 0;
    let mut r = m.len() - 1;
    while l < r {
        if m[l].is_some() {
            l += 1;
        } else if m[r].is_none() {
            r -= 1;
        } else {
            m.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    m.into_iter()
        .enumerate()
        .flat_map(|(position, file_id)| file_id.map(|f| position * f))
        .sum()
}

fn second_answer(map: &[u32]) -> usize {
    let mut m = Vec::new();
    for (i, x) in map.chunks(2).enumerate() {
        m.extend(vec![Some(i); x[0] as usize]);
        if x.len() > 1 {
            m.extend(vec![None; x[1] as usize]);
        }
    }

    let mut r = m.len() - 1;
    let mut considered = HashSet::new();
    while r > 0 {
        if m[r].is_none_or(|x| considered.contains(&x)) {
            r -= 1;
        } else {
            considered.insert(m[r].unwrap());

            let mut rr = r;
            while m[rr] == m[r] && rr > 0 {
                rr -= 1;
            }

            let mut l = 0;
            while l < r {
                if m[l].is_some() {
                    l += 1;
                } else {
                    let mut ll = l;
                    while m[ll] == m[l] {
                        ll += 1;
                    }
                    if (ll - l) >= (r - rr) {
                        for _ in rr..r {
                            m.swap(l, r);
                            l += 1;
                            r -= 1;
                        }
                        break;
                    } else {
                        l = ll;
                    }
                }
            }
        }
    }

    m.into_iter()
        .enumerate()
        .flat_map(|(position, file_id)| file_id.map(|f| position * f))
        .sum()
}
