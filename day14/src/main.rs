use day14::{part1, part2};
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!("Q1: What will the safety factor be after exactly 100 seconds have elapsed?");
    let num = part1(&contents, 101, 103);
    println!("A1: {}", num);

    println!("Q2: What is the fewest number of seconds that must elapse for the robots to display the Easter egg?");
    let num = part2(&contents, 101, 103);
    println!("A2: {}", num);

    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents, 11, 7);
    assert_eq!(num, 12);

    return Ok(());
}

// #[test]
// fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
//     let file_path = "input/dummy.txt";

//     // Read file contents
//     let contents = fs::read_to_string(&file_path)
//         .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

//     let num = part2(&contents, 11, 7);
//     assert_eq!(num, 0);

//     return Ok(());
// }