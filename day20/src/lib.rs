use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
        1
    }

    fn all_reachable_in_n(&self, start: usize, max_steps: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_front((start, 0));

        let mut result = vec![];

        while let Some((node, depth)) = queue.pop_front() {
            if depth > max_steps {
                continue;
            }

            result.push(node);

            for neighbour in self.node_to_neighbour[node].iter().cloned() {
                if visited.insert(neighbour) {
                    queue.push_back((neighbour, depth + 1));
                }
            }
        }

        result
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

        let h = |node: usize| -> u32 { 0 };

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

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|line| line.len()).ok_or(())?;
        let height = s.lines().count();

        let coords = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| (y * width + x, char))
        });
        let mut node_to_neighbour = vec![vec![]; width * height];

        let mut start = 0;
        let mut goal = 0;
        for (idx, c) in coords {
            if c == 'S' {
                start = idx;
            } else if c == 'E' {
                goal = idx;
            } else if c != '.' {
                continue;
            }

            node_to_neighbour[idx] = vec![idx - 1, idx + 1, idx - width, idx + width];
        }

        Ok(Grid {
            height,
            width,
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
                let coord: usize = y * self.width + x;

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
                    let pos = path.iter().position(|p| *p == coord).unwrap();
                    pos.to_string().chars().next().unwrap()
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

pub fn part1(input: &str, at_least_n: usize) -> u32 {
    let grid = input.parse::<Grid>().unwrap();
    let path = grid.a_star(grid.start, grid.goal).unwrap();

    let mut saves_at_least_n = 0;
    for (p_idx, p) in path.iter().enumerate() {
        for n in &grid.node_to_neighbour[*p] {
            if !grid.node_to_neighbour[*n].is_empty() {
                // the neighbour in path was a empty space
                continue;
            }

            // check that the wall was not a boundary
            let w = *n;
            if w % grid.width < 1
                || w % grid.width > grid.width - 1
                || w < grid.width
                || w > grid.width * (grid.height - 1)
            {
                continue;
            }

            // if it was a wall, we are looking for an empty space on the other side
            let options = [w - 1, w + 1, w - grid.width, w + grid.width];
            let options = options
                .iter()
                .filter(|adj| !grid.node_to_neighbour[**adj].is_empty())
                .filter(|adj| **adj != *p);

            for option in options {
                let o_idx = path.iter().position(|o| o == option).unwrap();

                let w = grid.width;
                let dist = (*p % w).abs_diff(option % w) + (*p / w).abs_diff(option / w);
                if p_idx < o_idx {
                    let shortcut = o_idx - p_idx - dist;
                    if shortcut >= at_least_n {
                        saves_at_least_n += 1;
                    }
                }
            }
        }
    }

    saves_at_least_n
}

pub fn part2(input: &str, at_least_n: usize) -> u32 {
    let grid = input.parse::<Grid>().unwrap();
    let width = grid.width;
    let height = grid.height;
    let node_to_neighbour = (0..(width * height))
        .map(|idx| {
            let x = idx % width;
            let y = idx / width;
            let mut vec = vec![];
            if x >= 1 {
                vec.push(idx - 1);
            }
            if y >= 1 {
                vec.push(idx - width);
            }
            if x < width - 1 {
                vec.push(idx + 1);
            }
            if y < height - 1 {
                vec.push(idx + width);
            }
            vec
        })
        .collect();
    let full_grid = Grid {
        width,
        height,
        node_to_neighbour,
        start: 0,
        goal: 0,
    };

    let path = grid.a_star(grid.start, grid.goal).unwrap();
    let path_map: HashMap<_, _> = path.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    let mut saves_at_least_n = 0;

    for (p_idx, p) in path.iter().enumerate() {
        let options = full_grid.all_reachable_in_n(*p, 20);
        let w = grid.width;
        for option in options {
            let Some(o_idx) = path_map.get(&option) else {
                continue;
            };

            let dist = (*p % w).abs_diff(option % w) + (*p / w).abs_diff(option / w);

            // check that there was a shortcut
            if p_idx > *o_idx {
                continue;
            }

            let shortcut = o_idx - p_idx;

            // check if the shortcut was shorter than the distance travelled
            if shortcut <= dist {
                continue;
            }

            if shortcut - dist >= at_least_n {
                saves_at_least_n += 1;
            }
        }
    }

    saves_at_least_n
}
