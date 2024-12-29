use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    x: i32,
    y: i32,
    id: char,
}

impl Node {
    fn get_neighbour_down(&self) -> Node {
        Node {
            x: self.x,
            y: self.y + 1,
            id: self.id,
        }
    }

    fn get_neighbour_up(&self) -> Node {
        Node {
            x: self.x,
            y: self.y - 1,
            id: self.id,
        }
    }

    fn get_neighbour_left(&self) -> Node {
        Node {
            x: self.x - 1,
            y: self.y,
            id: self.id,
        }
    }

    fn get_neighbour_right(&self) -> Node {
        Node {
            x: self.x + 1,
            y: self.y,
            id: self.id,
        }
    }

    fn get_neighbours(&self) -> Vec<Node> {
        vec![
            self.get_neighbour_up(),
            self.get_neighbour_down(),
            self.get_neighbour_left(),
            self.get_neighbour_right(),
        ]
    }
}

pub fn part1(input: &str) -> u32 {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| Node {
                x: x as i32,
                y: y as i32,
                id: char,
            })
        })
        .collect::<Vec<_>>();

    let nodes_set = nodes.iter().cloned().collect::<HashSet<_>>();

    let mut node_to_group: HashMap<&Node, usize> = HashMap::new();
    let mut group_to_node: HashMap<usize, Vec<&Node>> = HashMap::new();
    let mut group_to_area_permimiter: HashMap<usize, (u32, u32)> = HashMap::new();

    for (i, node) in nodes.iter().enumerate() {
        let all_neighbours = node.get_neighbours();
        let neighbours: Vec<_> = all_neighbours
            .iter()
            .filter_map(|neighbour| nodes_set.get(&neighbour))
            .collect();

        let perimiter = (all_neighbours.len() - neighbours.len()) as u32;

        let mut groups: Vec<_> = neighbours
            .into_iter()
            .filter_map(|neighbour| node_to_group.get(&neighbour))
            .map(|group| *group)
            .collect();

        // add new group
        if groups.len() == 0 {
            groups.push(i);
        }

        groups.sort();
        groups.dedup();
        let group = groups[0];
        let to_merge = &groups[1..];

        node_to_group.entry(&node).or_insert(group);
        group_to_node.entry(group).or_insert(Vec::new()).push(node);
        let (area, per) = group_to_area_permimiter.entry(group).or_insert((0, 0));
        *area += 1;
        *per += perimiter;

        for neighbour_group in to_merge {
            // copy area, perimiter to result
            let (area, per) = *group_to_area_permimiter.get(neighbour_group).unwrap();
            let (new_area, new_per): &mut (u32, u32) =
                group_to_area_permimiter.get_mut(&group).unwrap();
            *new_area += area;
            *new_per += per;

            // copy nodes to new group
            let to_copy: Vec<&Node> = group_to_node.get(&neighbour_group).unwrap().clone();
            group_to_node
                .get_mut(&group)
                .unwrap()
                .extend(to_copy.clone());

            // update node lookup
            for n in &to_copy {
                let g = node_to_group.get_mut(n).unwrap();
                *g = group;
            }

            group_to_area_permimiter.remove(&neighbour_group);
            group_to_node.remove(&neighbour_group);
        }
    }

    group_to_area_permimiter
        .into_iter()
        .map(|(_, (area, per))| area * per)
        .sum()
}

fn count_sides_horizontal(points: &mut Vec<&Node>, nodes_set: &HashSet<Node>) -> u32 {
    let mut sides = 0;

    for dir in [Node::get_neighbour_up, Node::get_neighbour_down] {
        // sort points horizontally
        points.sort_by_key(|point| (point.y, point.x));
        let mut prev: &Node = points[0];
        let mut prev_was_edge = false;

        let is_edge = !nodes_set.contains(&dir(prev));
        if is_edge {
            sides += 1;
            prev_was_edge = true;
        }

        for point in &points[1..] {
            if prev.y != point.y {
                prev_was_edge = false;
            }

            let is_edge = !nodes_set.contains(&dir(point));
            if !is_edge {
                prev_was_edge = false;
            } else {
                if !prev_was_edge || prev.get_neighbour_right() != **point {
                    sides += 1;
                    prev_was_edge = true;
                }
            }

            prev = point;
        }
    }

    return sides;
}

fn count_sides_vertical(points: &mut Vec<&Node>, nodes_set: &HashSet<Node>) -> u32 {
    let mut sides = 0;

    for dir in [Node::get_neighbour_left, Node::get_neighbour_right] {
        // sort points horizontally
        points.sort_by_key(|point| (point.x, point.y));
        let mut prev: &Node = points[0];
        let mut prev_was_edge: bool = false;

        let is_edge = !nodes_set.contains(&dir(prev));
        if is_edge {
            sides += 1;
            prev_was_edge = true;
        }

        // check up
        for point in &points[1..] {
            if prev.x != point.x {
                prev_was_edge = false;
            }

            let is_edge = !nodes_set.contains(&dir(point));
            if !is_edge {
                prev_was_edge = false;
            } else {
                if !prev_was_edge || prev.get_neighbour_down() != **point {
                    sides += 1;
                    prev_was_edge = true;
                }
            }

            prev = point;
        }
    }

    return sides;
}

fn count_sides(points: &mut Vec<&Node>, nodes_set: &HashSet<Node>) -> u32 {
    // count up down left right
    return count_sides_horizontal(points, nodes_set) + count_sides_vertical(points, nodes_set);
}

pub fn part2(input: &str) -> u32 {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| Node {
                x: x as i32,
                y: y as i32,
                id: char,
            })
        })
        .collect::<Vec<_>>();
    let nodes_set = nodes.iter().cloned().collect::<HashSet<_>>();

    let mut group_to_points = HashMap::new();
    let mut group_to_node = HashMap::new();
    let mut node_to_group = HashMap::new();

    for (i, node) in nodes.iter().enumerate() {
        let all_neighbours = node.get_neighbours();
        let neighbours: Vec<_> = all_neighbours
            .iter()
            .filter_map(|neighbour| nodes_set.get(&neighbour))
            .collect();

        let mut groups: Vec<_> = neighbours
            .into_iter()
            .filter_map(|neighbour| node_to_group.get(&neighbour))
            .map(|group| *group)
            .collect();

        // add new group
        if groups.len() == 0 {
            groups.push(i);
        }

        groups.sort();
        groups.dedup();
        let group = groups[0];
        let to_merge = &groups[1..];

        node_to_group.entry(node).or_insert(group);
        group_to_node.entry(group).or_insert(Vec::new()).push(node);
        let points = group_to_points.entry(group).or_insert(Vec::new());
        points.push(node);

        for neighbour_group in to_merge {
            // merge polygon
            let old_nodes = group_to_points.remove(neighbour_group).unwrap();
            let new_nodes = group_to_points.get_mut(&group).unwrap();
            new_nodes.extend(old_nodes);

            // copy nodes to new group
            let to_copy: Vec<&Node> = group_to_node.get(&neighbour_group).unwrap().clone();
            group_to_node
                .get_mut(&group)
                .unwrap()
                .extend(to_copy.clone());

            // update node lookup
            for n in &to_copy {
                let g = node_to_group.get_mut(n).unwrap();
                *g = group;
            }

            group_to_node.remove(&neighbour_group);
        }
    }

    let mut result = 0;
    for (group, points) in group_to_points.iter_mut() {
        let sides = count_sides(points, &nodes_set);
        result += sides * points.len() as u32;
    }

    result
}
