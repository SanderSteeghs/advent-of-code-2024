use day15::{part1, part2};
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: what is the sum of all boxes' GPS coordinates?");
    let num = part1(&contents);
    println!("A1: {}", num);

    println!("Q2: What is the sum of all boxes' final GPS coordinates?");
    let num = part2(&contents);
    println!("A2: {}", num);

    Ok(())
}

#[test]
fn test_part_1_small() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy_small.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents);
    assert_eq!(num, 2028);

    return Ok(());
}

#[test]
fn test_part_1_big() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy_big.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents);
    assert_eq!(num, 10092);

    return Ok(());
}


#[test]
fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy_big.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part2(&contents);
    assert_eq!(num, 9021);

    return Ok(());
}