use day17::{part1, part2};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).ok_or("Usage: day1 <file_to_txt>")?;
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    println!(
        "Q1: What do you get if you use commas to join the values it output into a single string?"
    );
    let num = part1(&contents);
    println!("A1: {:?}", num);

    println!("Q2: How many tiles are part of at least one of the best paths through the maze?");
    let num = part2(&contents);
    println!("A2: {:?}", num);

    Ok(())
}

#[test]
fn test_part_1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input/dummy.txt";

    // Read file contents
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let num = part1(&contents);
    assert_eq!(num, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);

    return Ok(());
}

// #[test]
// fn test_part_2() -> Result<(), Box<dyn std::error::Error>> {
//     let file_path = "input/dummy2.txt";

//     // Read file contents
//     let contents = fs::read_to_string(&file_path)
//         .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

//     let num = part2(&contents);
//     assert_eq!(num, 117440);

//     return Ok(());
// }

// #[test]
// fn test_my_test() {
//     let mut a = 0;
//     let mut b = 0;
//     let mut c = 0;
//     let mut o = 0;

//     for k in 0..100000000 {
//         let a =  1 + 4*k;
//         b =  2 + 8*k;

//         b = a % 8;
//         b = b ^ 2;

//         c = a / 2u32.pow(b);
//         b = b ^ c;

//         b = b ^ 3;
//         o = b % 8;

//         if o == 4  {
//             println!("k={}", k);
//         }
//     }
// }
