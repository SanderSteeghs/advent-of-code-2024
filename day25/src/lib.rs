use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let keys_and_locks = input.split("\n\n");

    let mut keys = HashSet::new();
    let mut locks = Vec::new();

    for key_or_lock in keys_and_locks {
        let first_line = key_or_lock.lines().next().unwrap();
        let num_lines = key_or_lock.lines().count();

        let is_lock = first_line.starts_with('#');

        let mut pin_heights: Vec<u8> = Vec::new();
        pin_heights.resize(first_line.len(), 0);

        let lines: Vec<_> = if is_lock {
            key_or_lock.lines().skip(1).collect()
        } else {
            key_or_lock.lines().take(num_lines - 1).collect()
        };

        for line in lines {
            for (i, char) in line.char_indices() {
                if char == '#' {
                    pin_heights[i] += 1;
                }
            }
        }
        if is_lock {
            locks.push(pin_heights);
        } else {
            keys.insert(pin_heights);
        }
    }

    let mut idx_height_to_keys: HashMap<(usize, u8), HashSet<_>> = HashMap::new();

    for key in &keys {
        for (i, height) in key.iter().enumerate() {
            let k = (i, *height);
            idx_height_to_keys.entry(k).or_default().insert(key.clone());
        }
    }

    let mut result: u32 = 0;

    for lock in locks {
        let mut candiates = keys.clone();
        for (i, height) in lock.iter().enumerate() {
            let max = 5 - height;
            for h in max+1..=5 {
                let empty = HashSet::new();
                let to_remove = idx_height_to_keys.get(&(i, h)).unwrap_or(&empty);

                candiates = candiates.difference(to_remove).cloned().collect();
            }
        }
        result += candiates.len() as u32;
    }

    result
}

// 11631 too high...