use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

struct Grid {
    height: usize,
    width: usize,
    node_to_neighbour: Vec<Vec<usize>>,
    start: usize,
    goal: usize,
}

impl Grid {
    fn distance(from: usize, to: usize) -> u32 {
        if from % 4 == to % 4 {
            return 1;
        }
        return 1000;
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
            let current_pos = node / 4;
            let current_dir = node % 4;

            let goal_pos = goal / 4;
            let goal_dir = goal % 4;

            let cur_row = current_pos / (self.width * 4);
            let cur_col = current_pos % (self.width * 4);

            let goal_row = goal_pos / (self.width * 4);
            let goal_col = goal_pos % (self.width * 4);

            let dx = (cur_col as i32 - goal_col as i32).abs();
            let dy = (cur_row as i32 - goal_row as i32).abs();
            let manhattan_dist = (dx + dy) as u32;

            let raw_diff = (current_dir as i32 - goal_dir as i32).rem_euclid(4) as u32;
            let orientation_diff = raw_diff.min(4 - raw_diff);
            let turn_cost = orientation_diff * 1000;
            manhattan_dist + turn_cost
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

    pub fn a_star_all_paths(&self, start: usize, goal: usize) -> Vec<Vec<usize>> {
        fn reconstruct_all_paths(
            parents: &HashMap<usize, Vec<usize>>,
            start: usize,
            goal: usize
        ) -> Vec<Vec<usize>> {
            let mut results = Vec::new();
            let mut path_buffer = Vec::new();

            fn backtrack(
                parents: &HashMap<usize, Vec<usize>>,
                current: usize,
                start: usize,
                path_buffer: &mut Vec<usize>,
                results: &mut Vec<Vec<usize>>,
            ) {
                if current == start {
                    let mut full_path = path_buffer.clone();
                    full_path.push(start);
                    full_path.reverse();
                    results.push(full_path);
                    return;
                }
                if let Some(pars) = parents.get(&current) {
                    for &p in pars {
                        path_buffer.push(current);
                        backtrack(parents, p, start, path_buffer, results);
                        path_buffer.pop();
                    }
                }
            }

            backtrack(parents, goal, start, &mut path_buffer, &mut results);
            results
        }

        // Your heuristic
        let h = |node: usize| -> u32 {
            0
        };

        let mut result = Vec::new();
        let mut open = BinaryHeap::new();
        open.push((Reverse(0), start));

        // Using Vec<usize> so each node can have multiple parents
        let mut parents: HashMap<usize, Vec<usize>> = HashMap::new();

        let mut g_score = vec![u32::MAX; self.node_to_neighbour.len()];
        g_score[start] = 0;

        let mut f_score = vec![u32::MAX; self.node_to_neighbour.len()];
        f_score[start] = h(start);

        let mut visited = HashSet::new();

        while let Some((Reverse(_), current)) = open.pop() {
            if current == goal {
                // Don’t push into `result` yet; we’ll do a single backtrack
                // at the end for all paths.
            }

            if !visited.insert(current) {
                continue;
            }

            for &neighbor in &self.node_to_neighbour[current] {
                let tentative_g_score = g_score[current] + Self::distance(current, neighbor);

                if tentative_g_score < g_score[neighbor] {
                    // Found a strictly better path
                    parents.insert(neighbor, vec![current]);
                    g_score[neighbor] = tentative_g_score;
                    f_score[neighbor] = tentative_g_score + h(neighbor);
                    open.push((Reverse(f_score[neighbor]), neighbor));
                } else if tentative_g_score == g_score[neighbor] {
                    // Found an equally good path => add another parent
                    parents.entry(neighbor).or_insert_with(Vec::new).push(current);
                }
            }
        }

        // Now reconstruct *all shortest paths* from `start` to `goal`.
        if g_score[goal] != u32::MAX {
            result = reconstruct_all_paths(&parents, start, goal);
        }

        result
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|line| line.len()).ok_or(())? * 4;
        let height = s.lines().count();

        let coords = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| (y * width + x * 4, char))
        });
        let mut node_to_neighbour = vec![vec![]; width * height];

        let mut start = 0;
        let mut goal = 0;
        for (coord, c) in coords {
            if c == 'S' {
                start = coord;
                continue;
            }
            if c == 'E' {
                goal = coord;
                continue;
            }
            if c != '.' {
                continue;
            }

            // east, south, west, north
            for dir in 0..4 {
                for i in 0..4 {
                    if i == dir {
                        continue;
                    }
                    node_to_neighbour[coord + dir].push(coord + i);
                }
            }

            node_to_neighbour[coord + 0].push(coord + 4); // east
            node_to_neighbour[coord + 1].push(coord + width + 1); // south
            node_to_neighbour[coord + 2].push(coord - 4 + 2); // west
            node_to_neighbour[coord + 3].push(coord - width + 3); // north
        }

        // east, south, west, north
        for dir in 0..4 {
            for i in 0..4 {
                if i == dir {
                    continue;
                }
                node_to_neighbour[start + dir].push(start + i);
            }
        }

        node_to_neighbour[start + 0].push(start + 4); // east
        node_to_neighbour[start + 1].push(start + width + 1); // south
        node_to_neighbour[start + 2].push(start - 4 + 2); // west
        node_to_neighbour[start + 3].push(start - width + 3); // north

        Ok(Grid {
            height,
            width: width / 4,
            node_to_neighbour,
            start,
            goal,
        })
    }
}

impl Grid {
    fn visualize(&self, path: &Vec<usize>) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord: usize = y * self.width * 4 + x * 4;

                let c = if self.goal == coord {
                    'E'
                } else if self.start == coord {
                    'S'
                } else if self.node_to_neighbour[coord].is_empty() {
                    '#'
                } else {
                    '.'
                };

                let c = if path.contains(&(coord)) {
                    '>'
                } else if path.contains(&(coord + 1)) {
                    'v'
                } else if path.contains(&(coord + 2)) {
                    '<'
                } else if path.contains(&(coord + 3)) {
                    '^'
                } else {
                    c
                };

                result.push(c);
            }
            result.push('\n');
        }
        return result;
    }
}

const DIR: usize = 1;

pub fn part1(input: &str) -> u32 {
    let grid = input.parse::<Grid>().unwrap();

    (0..DIR)
        .filter_map(|dir| grid.a_star(grid.start, grid.goal + dir))
        .map(|path| {
            path.windows(2)
                .map(|ab| Grid::distance(ab[0], ab[1]))
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let grid = input.parse::<Grid>().unwrap();

    let unique = (0..DIR)
        .flat_map(|dir| grid.a_star_all_paths(grid.start, grid.goal + dir))
        .map(|path| {
            (
                path.clone(),
                path.windows(2)
                    .map(|ab| Grid::distance(ab[0], ab[1]))
                    .sum::<u32>(),
            )
        })
        .filter(|(_, s)| *s == 88468)
        .flat_map(|(p, s)| p)
        .map(|x| x - x % 4)
        .collect::<HashSet<_>>();
    unique.len() as u32
}