use std::collections::HashMap;

fn is_possible(design: &str, towels: &[&str]) -> bool {
    if design.len() == 0 {
        return true;
    }

    towels
        .into_iter()
        .filter(|&&t| design.starts_with(t))
        .map(|towel| is_possible(&design[towel.len()..], towels))
        .any(|b| b)
}

fn num_is_possible<'a>(design: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(res) = cache.get(design) {
        return *res;
    }

    let count = towels
        .into_iter()
        .filter(|&&t| design.starts_with(t))
        .map(|t| num_is_possible(&design[t.len()..], towels, cache))
        .sum();

    cache.insert(design, count);
    return count;
}

pub fn part1(input: &str) -> usize {
    let towels: Vec<_> = input.lines().next().unwrap().split(", ").collect();
    input
        .lines()
        .skip(2)
        .map(|l| is_possible(l, &towels))
        .filter(|o| *o)
        .count()
}

pub fn part2(input: &str) -> u64 {
    let towels: Vec<_> = input.lines().next().unwrap().split(", ").collect();
    let mut cache = HashMap::new();

    input
        .lines()
        .skip(2)
        .map(|design| num_is_possible(design, &towels, &mut cache))
        .sum()
}
