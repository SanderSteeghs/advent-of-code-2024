use std::cmp::min;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Report(Vec<i32>);

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Result<Vec<_>, _> = s.split_whitespace().map(|n| n.parse::<i32>()).collect();

        numbers.map(Report)
    }
}

impl Report {
    fn is_safe(&self) -> Result<(), usize> {
        let data = &self.0;

        let mut incr = false;
        let mut decr = false;
        let idx = data
            .iter()
            .zip(data.iter().skip(1))
            .map(|(a, b)| {
                let step = a - b;
                incr |= step > 0;
                decr |= step < 0;

                step == 0 || step.abs() > 3 || (incr && decr)
            })
            .position(|t| t);

        match idx {
            Some(idx) => Err(idx),
            None => Ok(()),
        }
    }

    fn is_safe_gracefully(&self) -> bool {
        let data = &self.0;

        let idx = self.is_safe();
        if idx.is_ok() {
            return true;
        }

        // remove indices around the error and re-try
        let idx = idx.unwrap_err();
        let lower = idx - min(idx, 2);
        let upper = min(idx + 2, data.len());

        for i in lower..upper {
            let report = data[..i]
                .iter()
                .chain(data[i + 1..].iter())
                .cloned()
                .collect::<Vec<_>>();
            if Report(report).is_safe().is_ok() {
                return true;
            }
        }

        false
    }
}

fn calculate_num_safe(reports: &[Report]) -> i32 {
    reports.iter().filter(|l| l.is_safe().is_ok()).count() as i32
}

fn calculate_num_safe_gracefully(reports: &[Report]) -> i32 {
    reports.iter().filter(|l| l.is_safe_gracefully()).count() as i32
}

fn parse_input(input: &str) -> Result<Vec<Report>, ParseIntError> {
    input.lines().map(|line| line.parse::<Report>()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file path from CLI args
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    // Parse input
    let reports: Vec<Report> = parse_input(&contents)?;

    // Answer first question
    println!("Q1: How many reports are safe?");
    let num_safe = calculate_num_safe(&reports);
    println!("A1: The total number of safe reports {}", num_safe);

    // Answer second question
    println!("Q2: How many reports are safe, if one is removed?");
    let num_safe: i32 = calculate_num_safe_gracefully(&reports);
    println!("A2: The total number of safe reports {}", num_safe);

    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let levels: Vec<Report> = parse_input(&contents)?;
    assert_eq!(calculate_num_safe(&levels), 2);
    assert_eq!(calculate_num_safe_gracefully(&levels), 4);

    return Ok(());
}
