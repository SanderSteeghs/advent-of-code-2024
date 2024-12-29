use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
struct Grid {
    height: usize,
    width: usize,
    node_to_neighbour: Vec<Vec<usize>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut node_to_neighbour = vec![vec![]; width * height];
        let num_nodes = node_to_neighbour.len();
        for idx in 0..node_to_neighbour.len() {
            if idx % width != 0 {
                node_to_neighbour[idx].push(idx - 1);
            } // left
            if idx >= width {
                node_to_neighbour[idx].push(idx - width);
            } // up
            if (idx + 1) % width != 0 && idx + 1 < num_nodes {
                node_to_neighbour[idx].push(idx + 1);
            } // right
            if idx + width < num_nodes {
                node_to_neighbour[idx].push(idx + width);
            } // down
        }

        Grid {
            height,
            width,
            node_to_neighbour,
        }
    }

    fn distance(from: usize, to: usize) -> u32 {
        1
    }

    fn a_star(&self, start: usize, goal: usize) -> Option<Vec<usize>> {
        fn reconstruct_path(came_from: &HashMap<usize, usize>, current: usize) -> Vec<usize> {
            let mut current = current;
            let mut path = vec![current];
            while let Some(&prev) = came_from.get(&current) {
                current = prev;
                path.push(current);
            }
            path.reverse();
            path
        }

        let h = |node: usize| -> u32 {
            let x = node % self.width;
            let y = node / self.width;

            let g_x = self.width - 1;
            let g_y = self.height - 1;

            (x.abs_diff(g_x) + y.abs_diff(g_y)) as u32
         };

        let mut open = BinaryHeap::new();
        open.push((Reverse(0), start));

        let mut came_from = HashMap::new();
        let mut g_score = vec![u32::MAX; self.node_to_neighbour.len()];
        g_score[start] = 0;

        let mut f_score = vec![u32::MAX; self.node_to_neighbour.len()];
        f_score[start] = h(start);

        let mut visited = HashSet::new();

        while let Some((Reverse(_), current)) = open.pop() {
            if current == goal {
                return Some(reconstruct_path(&came_from, current));
            }

            if !visited.insert(current) {
                continue;
            }

            for &neighbor in &self.node_to_neighbour[current] {
                let tentative_g_score = g_score[current] + Grid::distance(current, neighbor);

                if tentative_g_score < g_score[neighbor] {
                    came_from.insert(neighbor, current);
                    g_score[neighbor] = tentative_g_score;
                    f_score[neighbor] = tentative_g_score + h(neighbor);

                    open.push((Reverse(f_score[neighbor]), neighbor));
                }
            }
        }

        None
    }
}

impl Grid {
    fn visualize(&self, path: &Vec<usize>) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord: usize = y * self.width + x;

                let c = if self.node_to_neighbour[coord].is_empty() {
                    '#'
                } else {
                    '.'
                };

                let c = if path.contains(&coord) { 'x' } else { c };

                result.push(c);
            }
            result.push('\n');
        }
        return result;
    }
}

pub fn part1(input: &str, sim: usize, width: usize, height: usize) -> usize {
    let mut grid = Grid::new(width, height);

    let bytes = input
        .lines()
        .take(sim)
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));

    for (x, y) in bytes {
        let idx = y * width + x;
        grid.node_to_neighbour[idx].clear();
    }

    let path = grid.a_star(0, width * height - 1).unwrap();
    path.len() - 1
}

pub fn part2(input: &str, width: usize, height: usize) -> (usize, usize) {
    let grid = Grid::new(width, height);

    let bytes: Vec<_> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    let mut min = 0;
    let mut max = bytes.len();
    while min <= max {
        let mid = (max + min) / 2;
        let mut g = grid.clone();

        for (x, y) in &bytes[..mid] {
            let idx = y * width + x;
            g.node_to_neighbour[idx].clear();
        }
        let path = g.a_star(0, width * height - 1);
        if path.is_none() {
            if min.abs_diff(max) == 1 {
                return bytes[mid - 1];
            }
            max = mid - 1;
            continue;
        }

        min = mid + 1;
    }

    panic!();
}
