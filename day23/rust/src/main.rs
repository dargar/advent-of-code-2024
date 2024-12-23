use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

type Computers<'a> = HashMap<&'a str, BTreeSet<&'a str>>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let computers: Computers = input.lines().flat_map(|line| line.split_once("-")).fold(
        HashMap::new(),
        |mut map, (a, b)| {
            map.entry(a)
                .and_modify(|set| {
                    set.insert(a);
                    set.insert(b);
                })
                .or_insert(BTreeSet::from([a, b]));
            map.entry(b)
                .and_modify(|set| {
                    set.insert(a);
                    set.insert(b);
                })
                .or_insert(BTreeSet::from([a, b]));
            map
        },
    );

    let mut peer_group_frequencies = HashMap::new();
    for peers in computers.values() {
        let peer_groups: BTreeSet<BTreeSet<&str>> = peers
            .iter()
            .flat_map(|peer| computers.get(peer))
            .map(|peer_peers| peers & peer_peers)
            .collect();
        for computers in peer_groups {
            peer_group_frequencies
                .entry(computers.clone())
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }

    let first_answer: BTreeSet<BTreeSet<&str>> = peer_group_frequencies
        .iter()
        .filter(|(computers, n)| computers.len() == **n)
        .flat_map(|(computers, _)| triples(computers))
        .filter(|computers| computers.iter().any(|computer| computer.starts_with("t")))
        .collect();
    println!("First answer: {}", first_answer.len());

    let second_answer = peer_group_frequencies
        .iter()
        .filter(|(computers, n)| computers.len() == **n)
        .max_by_key(|(_, n)| **n)
        .map(|(computers, _)| {
            let names: Vec<&str> = computers.iter().cloned().collect();
            names.join(",")
        })
        .unwrap();
    println!("Second answer: {second_answer}");
}

fn triples<T: Clone + Ord>(xs: &BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
    if xs.len() < 3 {
        return BTreeSet::new();
    }

    if xs.len() == 3 {
        return BTreeSet::from([xs.clone()]);
    }

    let mut result = BTreeSet::new();
    for a in xs {
        for b in xs {
            for c in xs {
                if a != b && b != c && a != c {
                    result.insert(BTreeSet::from([a.clone(), b.clone(), c.clone()]));
                }
            }
        }
    }
    result
}
