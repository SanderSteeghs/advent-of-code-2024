use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

type Step = Pos;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        let x = x.parse::<i32>().map_err(|_| ())?;
        let y = y.parse::<i32>().map_err(|_| ())?;
        Ok(Pos { x, y })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    pos: Pos,
    step: Step,
}

impl Robot {
    fn simulate(&mut self, n: u32, width: u32, height: u32) {
        let n = n as i32;
        let (width, height) = (width as i32, height as i32);

        self.pos.x += self.step.x * n;
        self.pos.y += self.step.y * n;

        self.pos.x = self.pos.x % width;
        self.pos.y = self.pos.y % height;

        if self.pos.x < 0 {
            self.pos.x += width;
        }

        if self.pos.y < 0 {
            self.pos.y += height;
        }
    }
}

pub fn part1(input: &str, width: u32, height: u32) -> u32 {
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').expect("invalid line"))
        .map(|(pos, step)| Robot {
            pos: pos[2..].parse::<Pos>().expect("invalid pos"),
            step: step[2..].parse::<Step>().expect("invalid step"),
        })
        .collect();

    // simulate robots
    for robot in &mut robots {
        robot.simulate(100, width, height);
    }

    // tally their positions
    let map = robots
        .into_iter()
        .map(|robot| robot.pos)
        .fold(HashMap::new(), |mut acc, pos| {
            *acc.entry(pos).or_insert(0) += 1;
            acc
        });

    let mut quarters = [0; 4];
    let (width, height) = (width as i32, height as i32);

    for (pos, count) in map.iter() {
        if pos.x * 2 + 1 == width || pos.y * 2 + 1 == height {
            continue;
        }

        match (pos.x < width / 2, pos.y < height / 2) {
            (true, true) => quarters[0] += count,
            (false, true) => quarters[1] += count,
            (true, false) => quarters[2] += count,
            (false, false) => quarters[3] += count,
        }
    }

    quarters.iter().fold(1, |acc, i| acc * i)
}

pub fn part2(input: &str, width: u32, height: u32) -> u32 {
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').expect("invalid line"))
        .map(|(pos, step)| Robot {
            pos: pos[2..].parse::<Pos>().expect("invalid pos"),
            step: step[2..].parse::<Step>().expect("invalid step"),
        })
        .collect();

    let mut history = HashSet::new();

    // simulate robots
    for i in 0.. {
        for robot in &mut robots {
            robot.simulate(1, width, height);
        }

        if !history.insert(robots.clone()) {
            break;
        }

        let map =
            robots
                .iter()
                .map(|robot| robot.pos.clone())
                .fold(HashMap::new(), |mut acc, pos| {
                    *acc.entry(pos).or_insert(0) += 1;
                    acc
                });

        let mut save = false;
        let mut contents = String::new();

        for y in 0..height {
            let mut counter = 0;
            for x in 0..width {
                let pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                let char = match map.get(&pos) {
                    Some(count) => {
                        counter += 1;
                        assert!(*count <= 9);
                        count.to_string().chars().next().unwrap()
                    }
                    None => {
                        counter = 0;
                        '.'
                    }
                };
                contents.push(char);
                if counter >= 3 {
                    save = true;
                }
            }
            contents.push('\n');
        }

        if save {
            let filename: String = format!("output/{:?}.txt", i + 1);
            let mut file = File::create(filename).expect("could not create file");
            file.write_all(contents.as_bytes()).expect("could not write file");
        }
    }

    0
}
