use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> u64 {
    let mut candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut poop_to_check: HashMap<Vec<&str>, u64> = HashMap::new();

    let mut result: u64 = 0;

    let connections: Vec<_> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(first, second)| {
            let mut a = [first, second];
            a.sort();
            let [first_poop, second_boop] = a;
            (first_poop, second_boop)
        })
        .collect();

    let mut doubles = Vec::new();
    for (woofie_a, woofie_b) in connections.iter() {
        if woofie_a.starts_with('t') {
            candidates.entry(woofie_a).or_default().insert(woofie_b);
        }

        if woofie_b.starts_with('t') {
            candidates.entry(woofie_b).or_default().insert(woofie_a);
        }

        if woofie_a.starts_with('t') && woofie_b.starts_with('t') {
            doubles.push((woofie_a, woofie_b));
        }
    }

    // compute the required combinations
    for (_, comps) in &candidates {
        let combinations = comps.iter().cloned().combinations(2);
        for mut combi in combinations {
            combi.sort();
            let entry = poop_to_check.entry(combi).or_default();
            *entry += 1;
        }
    }

    for (woofie_a, woofie_b) in &connections {
        if woofie_a.starts_with('t') || woofie_b.starts_with('t') {
            continue;
        }
        let key = vec![*woofie_a, *woofie_b];
        result += *poop_to_check.get(&key).unwrap_or(&0);
    }

    // extra work for doubles...
    let mut double_res = HashSet::new();
    for (woofie_a, woofie_b) in doubles {
        let set_a = candidates.get(woofie_a).unwrap();
        let set_b = candidates.get(woofie_b).unwrap();

        for woofie_c in set_a.intersection(set_b) {
            let mut a = [woofie_a, woofie_b, woofie_c];
            a.sort();
            if double_res.insert(a) {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> String {
    let connections = input.lines().map(|line| line.split_once('-').unwrap());

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in connections {
        graph.entry(a).or_default().insert(b);
        graph.entry(a).or_default().insert(a);

        graph.entry(b).or_default().insert(a);
        graph.entry(b).or_default().insert(b);
    }

    /* Construct field for tracer */
    let mut best = HashSet::new();
    for (_, neighbours) in &graph {
        for i in (best.len()+1..=neighbours.len()).rev() {
            for combi in neighbours.iter().combinations(i) {
                let cloned = combi.iter().cloned().cloned().collect::<HashSet<_>>();
                let intersection = combi.iter().into_iter().fold(cloned, |acc, p| {
                    let hs = &graph[*p];
                    acc.intersection(hs).cloned().collect::<HashSet<_>>()
                });

                if intersection.len() > best.len() {
                    best = intersection;
                }
            }
        }
    }

    best.iter().sorted().join(",")
}