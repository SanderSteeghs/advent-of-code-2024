use day5::{part1, part2};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: What do you get if you add up the middle page number from those correctly-ordered updates?");
    let num = part1(&contents);
    println!("A1: {}", num);

    println!("Q2: What do you get if you add up the middle page numbers after correctly ordering just those updates?");
    let num = part2(&contents);
    println!("A2: {}", num);
    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents);
    assert_eq!(num, 143);

    return Ok(());
}

#[test]
fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part2(&contents);
    assert_eq!(num, 123);

    return Ok(());
}