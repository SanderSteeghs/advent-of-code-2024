use day3::parse_muls;
use day3::parse_muls2;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: What do you get if you add up all of the results of the multiplications?");
    let sum = parse_muls(&contents, false);
    println!("A1: The sum of all results is {}", sum);

    println!(
        "Q2: What do you get if you add up all of the results of just the enabled multiplications?"
    );
    let sum = parse_muls(&contents, true);
    println!("A2: The sum of all results is {}", sum);

    Ok(())
}

#[test]
fn test_all_muls() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let sum = parse_muls2(&contents, false);
    assert_eq!(sum, 161);

    return Ok(());
}

#[test]
fn test_enabled_muls() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy2.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let sum = parse_muls2(&contents, true);
    assert_eq!(sum, 48);

    return Ok(());
}
