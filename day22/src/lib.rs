use std::collections::{HashMap, HashSet};
use std::iter::zip;

fn generate_secret(secret: u64) -> u64 {
    let mut secret = secret;

    let x = secret * 64;
    secret = x ^ secret;
    secret = secret % 16777216;

    let x = secret / 32;
    secret = x ^ secret;
    secret = secret % 16777216;

    let x = secret * 2048;
    secret = x ^ secret;
    secret = secret % 16777216;

    secret
}

fn generate_secrets(secret: u64, num: usize) -> u64 {
    let mut secret = secret;
    for _ in 0..num {
        secret = generate_secret(secret);
    }
    secret
}

fn get_prices_and_diffs(secret: u64, num: usize) -> (Vec<u8>, Vec<i8>) {
    let mut secret = secret;
    let mut prices = vec![];
    let mut diffs = vec![];

    let mut prev = (secret % 10) as u8;
    for _ in 0..num {
        secret = generate_secret(secret);
        let price = (secret % 10) as u8;

        prices.push(price);
        diffs.push(price as i8 - prev as i8);

        prev = price;
    }

    (prices, diffs)
}

pub fn part1(input: &str) -> u64 {
    let num_secrets = 2000;
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| generate_secrets(secret, num_secrets))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let num_prices = 2000;
    let buyers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| get_prices_and_diffs(secret, num_prices))
        .collect();

    let mut buyer_to_sequences = HashSet::new();
    let mut sequence_to_bananas = HashMap::new();

    let buyers = buyers
        .iter()
        .map(|(prices, diffs)| (&prices[3..], diffs))
        .map(|(prices, diffs)| (prices, diffs.windows(4)));

    for (i, (prices, diffs)) in buyers.enumerate() {
        for (price, diff) in zip(prices, diffs) {
            if !buyer_to_sequences.insert((i, diff)) {
                continue;
            }

            *sequence_to_bananas.entry(diff).or_default() += *price as u64;
        }
    }

    *sequence_to_bananas.iter().max_by_key(|x| x.1).unwrap().1
}
