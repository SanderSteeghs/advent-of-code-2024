use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn step(&self, pos: &mut Pos) {
        match self {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(Error {}),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Guard {
    pos: Pos,
    direction: Direction,
}

impl FromStr for Guard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, chr)| (x, y, chr)));

        for (x, y, c) in iter {
            let Ok(dir) = c.to_string().parse::<Direction>() else {
                continue;
            };

            return Ok(Guard {
                pos: Pos {
                    x: x as i32,
                    y: y as i32,
                },
                direction: dir,
            });
        }

        Err(Error {})
    }
}

#[derive(Debug)]
struct Room {
    width: usize,
    height: usize,
    obstacles: HashSet<Pos>,
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, chr)| (x, y, chr)))
            .collect();

        Ok(Room {
            width: *items.iter().map(|(x, _, _)| x).max().unwrap() + 1,
            height: *items.iter().map(|(_, y, _)| y).max().unwrap() + 1,
            obstacles: items
                .iter()
                .filter(|(_, _, c)| *c == '#')
                .map(|(x, y, _)| Pos { x: *x as i32, y: *y as i32})
                .collect(),
        })
    }
}

impl Room {
    fn is_safe(&self, pos: &Pos) -> bool {
        !self.obstacles.contains(pos)
    }

    fn contains(&self, pos: &Pos) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.width && pos.y >= 0 && (pos.y as usize) < self.height
    }
}

pub fn part1(input: &str) -> u32 {
    let room = input.parse::<Room>().unwrap();
    let mut guard = input.parse::<Guard>().unwrap();

    let mut visited = HashSet::new();
    visited.insert(guard.pos);

    loop {
        let mut next_pos = guard.pos;
        guard.direction.step(&mut next_pos);

        if !room.is_safe(&next_pos) {
            guard.direction = guard.direction.turn_right();
        } else {
            guard.pos = next_pos;
        }

        if !room.contains(&guard.pos) {
            break;
        }

        visited.insert(guard.pos);
    }

    visited.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut room = input.parse::<Room>().unwrap();
    let initial_guard = input.parse::<Guard>().unwrap();
    let mut guard = initial_guard;

    let mut visited = HashSet::new();
    loop {
        let mut next_pos = guard.pos;
        guard.direction.step(&mut next_pos);

        if !room.is_safe(&next_pos) {
            guard.direction = guard.direction.turn_right();
        } else {
            guard.pos = next_pos;
        }

        if !room.contains(&guard.pos) {
            break;
        }

        visited.insert(guard.pos);
    }

    let mut result = 0;

    for new_obstacle in visited.iter() {
        let mut guard = initial_guard;
        let mut local_visited = HashSet::new();
        local_visited.insert(guard);
        let must_delete = room.obstacles.insert(*new_obstacle);

        loop {
            let mut next_pos = guard.pos;
            guard.direction.step(&mut next_pos);

            if !room.is_safe(&next_pos) {
                guard.direction = guard.direction.turn_right();
            } else {
                guard.pos = next_pos;
            }

            if !local_visited.insert(guard) {
                result += 1;
                break;
            }

            if !room.contains(&guard.pos) {
                break;
            }
        }

        if must_delete {
            room.obstacles.remove(new_obstacle);
        }
    }

    result
}
