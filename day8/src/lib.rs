use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    // pos_to_antenna: HashMap<Pos, char>,
    antenna_to_pos: HashMap<char, Vec<Pos>>,
    size: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut pos_to_antenna = HashMap::new();
        let mut antenna_to_pos = HashMap::new();
        let first = s.lines().next().ok_or(ParseError {})?;
        let width = first.len();
        let height = s.lines().count();

        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                // pos_to_antenna.insert(pos, char);

                if char == '.' {
                    continue;
                }

                let entry = antenna_to_pos.entry(char).or_insert(Vec::new());
                entry.push(pos.clone());
            }
        }

        Ok(Map {
            // pos_to_antenna,
            antenna_to_pos,
            size: (width, height),
        })
    }
}

impl Map {
    fn contains(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size.0 as i64 && pos.y < self.size.1 as i64
    }

    fn compute_antinodes(&self) -> HashSet<Pos> {
        let combinations = self
            .antenna_to_pos
            .iter()
            .flat_map(|(_, pos)| pos.into_iter().tuple_combinations())
            .flat_map(|(first, second)| {
                let x1 = 2 * second.x - first.x;
                let y1 = 2 * second.y - first.y;
                let x2 = 2 * first.x - second.x;
                let y2 = 2 * first.y - second.y;

                let first = Pos { x: x1, y: y1 };
                let second = Pos { x: x2, y: y2 };

                [first, second]
            })
            .filter(|antinode| self.contains(antinode));

        return combinations.collect();
    }

    fn compute_harmonics(&self) -> HashSet<Pos> {
        let combinations = self
            .antenna_to_pos
            .iter()
            .flat_map(|(_, pos)| pos.into_iter().tuple_combinations())
            .flat_map(|(first, second)| {
                let dx = second.x - first.x;
                let dy = second.y - first.y;

                let lefts = (0..)
                    .map(move |i| Pos {
                        x: first.x - i * dx,
                        y: first.y - i * dy,
                    })
                    .take_while(|antinode| self.contains(antinode));
                let rights = (1..)
                    .map(move |i| Pos {
                        x: first.x + i * dx,
                        y: first.y + i * dy,
                    })
                    .take_while(|antinode| self.contains(antinode));

                lefts.chain(rights)
            });

        return combinations.collect();
    }
}

pub fn part1(input: &str) -> u64 {
    let map = input.parse::<Map>().expect("could not parse map");
    map.compute_antinodes().len() as u64
}

pub fn part2(input: &str) -> u64 {
    let map = input.parse::<Map>().expect("could not parse map");
    map.compute_harmonics().len() as u64
}
