use std::{collections::HashMap, collections::HashSet, hash::Hash, str::FromStr};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn as_idx(&self, width: usize) -> isize {
        (self.x + self.y * (width as i32)) as isize
    }
}

impl Pos {
    fn get_neighbour(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Up => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ItemType {
    BoxLeft,
    BoxRight,
    Box,
    Wall,
}

impl ItemType {
    fn parse(c: char) -> Option<Self> {
        match c {
            '#' => Some(ItemType::Wall),
            'O' => Some(ItemType::Box),
            '[' => Some(ItemType::BoxLeft),
            ']' => Some(ItemType::BoxRight),
            _ => None,
        }
    }

    fn get_other_box(&self, pos: &Pos) -> Option<Pos> {
        match self {
            ItemType::BoxLeft => Some(pos.get_neighbour(&Direction::Right)),
            ItemType::BoxRight => Some(pos.get_neighbour(&Direction::Left)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    width: usize,
    robot: Pos,
    pos_to_item: Vec<Option<ItemType>>,
}

impl Warehouse {
    fn new(robot: Pos, items: Vec<Option<ItemType>>) -> Self {
        let width = 100;

        Warehouse {
            width,
            robot,
            pos_to_item: items,
        }
    }

    fn walk_robot(&mut self, direction: Direction) {
        /*
        let desired = self.robot.get_neighbour(&direction);

        // check if robot can move into a free space
        let Some(occupied) = self.pos_to_item.get(&desired) else {
            self.robot = desired;
            return;
        };
        // check if the robot would move into a wall
        if matches!(occupied, ItemType::Wall) {
            return;
        }

        let mut current = desired;
        let mut desired = current.get_neighbour(&direction);
        let to_remove = current;
        let mut to_add = (desired, *occupied);
        // let mut to_update = vec![(current, *occupied, desired)];

        while let Some(occupied) = self.pos_to_item.get(&desired) {
            if matches!(occupied, ItemType::Wall) {
                return;
            }

            current = desired;
            desired = current.get_neighbour(&direction);
            to_add = (desired, *occupied);
        }

        self.robot = self.robot.get_neighbour(&direction);
        self.pos_to_item.remove_entry(&to_remove);
        self.pos_to_item.insert(to_add.0, to_add.1);
        */
    }

    fn walk_robot2(&mut self, direction: Direction) {
        let desired = self.robot.get_neighbour(&direction);

        // check if robot can move into a free space
        let Some(occupied) = self.pos_to_item[desired.as_idx(self.width) as usize] else {
            self.robot = desired;
            return;
        };
        // check if the robot would move into a wall
        if matches!(occupied, ItemType::Wall) {
            return;
        }

        let mut to_update = Vec::new();

        let mut to_move = vec![(Some(desired), occupied.get_other_box(&desired))];
        let mut moving = HashSet::new();
        while let Some((p1, p2)) = to_move.pop() {
            // we could have processed this box already..
            if !moving.insert((p1, p2)) || !moving.insert((p2, p1)) {
               continue;
            }

            let pos: Vec<_> = vec![p1, p2].iter().filter_map(|p| *p).collect();

            // get desired places for the box(es)
            let desired: Vec<_> = pos.iter().map(|p| p.get_neighbour(&direction)).collect();

            // get the occupied elements for the box(es)
            let occupied: Vec<_> = desired.iter().map(|d| self.pos_to_item.get(d)).collect();

            // check if any box pushes into a wall, if so we can no longer move
            if occupied.iter().any(|o| matches!(o, Some(ItemType::Wall))) {
                return;
            }

            // if not, we should be able to move these boxes
            let targets: Vec<_> =  pos.iter().map(|p| *self.pos_to_item.get(p).unwrap()).collect();

            for i in 0..pos.len() {
                to_update.push((pos[i], targets[i], desired[i]));

                if let Some(o) = occupied[i] {
                    to_move.push((Some(desired[i]), o.get_other_box(&desired[i])));
                }
            }
        }

        self.robot = self.robot.get_neighbour(&direction);
        for (old_pos, _, _) in &to_update {
            self.pos_to_item[old_pos.as_idx(self.width) as usize] = None;
        }
        for (_, old_type, new_pos) in &to_update {
            self.pos_to_item[new_pos.as_idx(self.width) as usize] = *old_type;
        }

    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
            .map(|(x, y, c)| ItemType::parse(c))
            .collect();

        let robot = items.iter().find(|(_, c)| *c == '@').map(|(pos, _)| pos).unwrap();
        let items = items.iter().filter_map(|(pos, c)| ItemType::parse(*c).map(|c| (*pos, c)));

        Ok(Warehouse::new(*robot, items))
    }
}

pub fn part1(input: &str) -> u32 {
    let (map, directions) = input.split_once("\n\n").expect("invalid input");
    let mut map = map.parse::<Warehouse>().expect("invalid warehouse");

    let directions = directions.replace('\n', "");
    let dirs = directions.chars().map(Direction::parse);

    for direction in dirs {
        map.walk_robot(direction);
    }

    map.pos_to_item
        .iter()
        .enumerate()
        .filter(|(_, item)| matches!(item, Some(ItemType::Box)))
        .map(|(i, _)| i)
        .map(|pos| pos % map.width + pos / map.width * 100)
        .map(|gps| gps as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (map, directions) = input.split_once("\n\n").expect("invalid input");
    let map = map.replace('#', "##");
    let map = map.replace('O', "[]");
    let map = map.replace('.', "..");
    let map = map.replace('@', "@.");
    let mut map = map.parse::<Warehouse>().expect("invalid warehouse");

    let directions = directions.replace('\n', "");
    let dirs = directions.chars().map(Direction::parse);

    for direction in dirs {
        map.walk_robot2(direction);
    }

    map.pos_to_item
        .iter()
        .enumerate()
        .filter(|(_, item)| matches!(item, Some(ItemType::BoxLeft)))
        .map(|(i, _)| i)
        .map(|pos| pos % map.width + pos / map.width * 100)
        .map(|gps| gps as u32)
        .sum()
}
