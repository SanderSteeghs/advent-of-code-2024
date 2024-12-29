use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::str::FromStr;

#[derive(Debug)]
struct Item(i32, i32);

#[derive(Debug)]
enum ParseItemError {
    InvalidFormat(String),
}

impl std::fmt::Display for ParseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseItemError::InvalidFormat(line) => {
                write!(f, "Failed to parse line: '{}'", line)
            }
        }
    }
}

impl std::error::Error for ParseItemError {}

impl FromStr for Item {
    type Err = ParseItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_whitespace().map(|n| n.parse::<i32>());
        match (numbers.next(), numbers.next()) {
            (Some(Ok(a)), Some(Ok(b))) => Ok(Item(a, b)),
            _ => Err(ParseItemError::InvalidFormat(s.to_string())),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), ParseItemError> {
    let items: Result<Vec<_>, _> = input
        .lines()
        .map(|line| line.parse::<Item>())
        .collect(); // Collect all results, stopping at the first error

    items.map(|parsed_items| parsed_items.into_iter().map(|Item(a, b)| (a, b)).unzip())
}

fn calculate_distance(left: &[i32], right: &[i32]) -> i32 {
    let mut left_sorted = left.to_vec();
    let mut right_sorted = right.to_vec();
    left_sorted.sort_unstable();
    right_sorted.sort_unstable();
    zip(left_sorted, right_sorted).map(|(a, b)| (a - b).abs()).sum()
}

fn calculate_similarity(left: &[i32], right: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();
    for &num in right {
        *frequency_map.entry(num).or_default() += 1;
    }
    left.iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum()
}

fn run(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    // Parse input
    let (left, right) = parse_input(&contents)?;

    // Answer first question
    println!("Q1: What is the total distance between your lists?");
    let distance = calculate_distance(&left, &right);
    println!("A1: The total distance between the lists is {}", distance);

    // Answer second question
    println!("Q2: What is their similarity score?");
    let similarity = calculate_similarity(&left, &right);
    println!("A2: The similarity score is {}", similarity);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file path from CLI args
    let file_path = std::env::args()
        .nth(1)
        .ok_or("Usage: day1 <file_to_txt>")?;

    run(&file_path)
}

#[test]
fn test() {
    run("../input/dummy.txt");
}