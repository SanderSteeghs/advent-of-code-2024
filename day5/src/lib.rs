use std::collections::{HashMap, HashSet};

struct State<'a> {
    updates: Vec<&'a str>,
    transitions: HashMap<usize, HashSet<usize>>
}

impl State<'_> {
    fn is_valid_update(&self, chain: &Vec<usize>) -> bool {
        for idx in 0..chain.len() - 1 {
            let head = chain[idx];
            let tail = &chain[idx + 1..];

            // check if there isn't any unallowed transition
            let Some(err_transition) = self.transitions.get(&head) else {
                continue;
            };
            let valid = !tail.iter().any(|item| err_transition.contains(item));

            if !valid {
                return false;
            }
        }

        return true;
    }
}

fn parse(input: &str) -> State {
    let mut iter = input.lines();
    let rules: Vec<_> = iter.by_ref().take_while(|l| !l.is_empty()).collect();

    let mut transitions: HashMap<usize, HashSet<usize>> = HashMap::new();
    for rule in rules {
        let mut rule = rule.split('|');
        let (before, after) = (
            rule.next().unwrap().parse::<usize>().unwrap(),
            rule.next().unwrap().parse::<usize>().unwrap(),
        );

        let items = transitions.entry(after).or_default();
        items.insert(before);
    }

    State { updates: iter.collect(), transitions: transitions }
}

pub fn part1(input: &str) -> u64 {

    let state = parse(input);
    let mut sum = 0;

    for update in &state.updates {
        let chain: Vec<_> = update
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

            if state.is_valid_update(&chain) {
                let mid_idx: usize = chain.len() / 2;
                sum += chain[mid_idx] as u64;
            }
    }

    sum
}

pub fn part2(input: &str) -> u64 {
    let state = parse(input);
    let mut sum = 0;

    for update in &state.updates {
        let chain: Vec<_> = update
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if state.is_valid_update(&chain) {
            continue;
        }

        // let's fix the chain
        let mut new_chain = chain.clone();
        for item in &chain {
            let count = chain
                .iter()
                .filter(|i| {
                    state.transitions
                        .get(i)
                        .map(|t| t.contains(item))
                        .unwrap_or(false)
                })
                .count();
            new_chain[count] = *item;
        }

        let mid_idx: usize = new_chain.len() / 2;
        sum += new_chain[mid_idx] as u64;
    }

    sum
}
