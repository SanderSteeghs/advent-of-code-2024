use day18::{part1, part2};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: What is the minimum number of steps needed to reach the exit?");
    let num = part1(&contents, 1024, 71, 71);
    println!("A1: {}", num);

    println!("Q2: What are the coordinates of the first byte that will prevent the exit from being reachable from your starting position?");
    let (x, y) = part2(&contents, 71, 71);
    println!("A2: {},{}", x, y);

    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents, 12, 7, 7);
    assert_eq!(num, 22);

    return Ok(());
}

#[test]
fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part2(&contents, 7, 7);
    assert_eq!(num, (6, 1));

    return Ok(());
}
