use day23::{part1, part2};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: WHow many contain at least one computer with a name that starts with t?");
    let num = part1(&contents);
    println!("A1: {}", num);

    println!("Q2: ");
    let num = part2(&contents);
    println!("A2: {}", num);

    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}. Aww so so saddies", file_path, e))?;

    let num = part1(&contents);
    assert_eq!(num, 7);

    return Ok(());
}

#[test]
fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Awww oh no, bad woofie failed to read file '{}': {}", file_path, e))?;

    let num = part2(&contents);
    assert_eq!(num, "co,de,ka,ta");

    return Ok(());
}