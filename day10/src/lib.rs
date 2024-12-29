use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    id: u32,
    num: u8,
    forward: Vec<NodeRef>,
    reaches_9: u32,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

fn dfs(root: NodeRef, target: u8) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    let mut result = HashSet::new();
    while let Some(node) = queue.pop_front() {
        // Check if this is the node we are looking for
        if node.borrow().num == target {
            result.insert(node.borrow().id);
            continue;
        }

        // Add left and write to the back of the queue for DFS
        for forward in &node.borrow().forward {
            queue.push_front(forward.clone());
        }
    }

    // Search completed and node was not found
    result.len() as u32
}

fn build_nodes(input: &str) -> Vec<NodeRef> {
    let mut nodes: Vec<_> = input
        .lines()
        .flat_map(|line| line.as_bytes().into_iter().map(|x| x - b'0'))
        .enumerate()
        .map(|(i, num)| Node {
            id: i as u32,
            num: num as u8,
            forward: Vec::new(),
            reaches_9: 0,
        })
        .map(|node| Rc::new(RefCell::new(node)))
        .collect();
    let width = input.lines().next().unwrap().len();
    let mut sorted_nodes = nodes.clone();

    sorted_nodes.sort_unstable_by_key(|node| node.borrow().num);

    for node in sorted_nodes.iter().rev() {
        let mut neighbour_indices = Vec::new();
        let idx = node.borrow().id as usize;

        if idx % width != 0 {
            neighbour_indices.push(idx - 1);
        } // left
        if idx >= width {
            neighbour_indices.push(idx - width);
        } // up
        if (idx + 1) % width != 0 && idx + 1 < nodes.len() {
            neighbour_indices.push(idx + 1);
        } // right
        if idx + width < nodes.len() {
            neighbour_indices.push(idx + width);
        } // down

        let neighbours = neighbour_indices
            .iter()
            .map(|idx| &nodes[*idx])
            .filter(|n| n.borrow().num == node.borrow().num + 1)
            .map(|n| n.clone())
            .collect::<Vec<_>>();

        node.borrow_mut().forward = neighbours;
        if node.borrow().num == 9 {
            node.borrow_mut().reaches_9 = 1;
            continue;
        }

        let sum = node
            .borrow()
            .forward
            .iter()
            .map(|n| n.borrow().reaches_9)
            .sum();
        node.borrow_mut().reaches_9 = sum;
    }

    return nodes;
}

pub fn part1(input: &str) -> u32 {
    let nodes = build_nodes(input);

    nodes
        .into_iter()
        .filter(|node| node.borrow().num == 0)
        .map(|trailstart| dfs(trailstart.clone(), 9))
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    let nodes = build_nodes(input);

    // for n in nodes.iter().filter(|n| n.borrow().num == 9) {
    //     n.borrow_mut().reaches_9 = 1;
    // }

    // // build op a cache
    // for i in (0..=8).rev() {
    //     for node in nodes.iter().filter(|n| n.borrow().num == i) {
    //         let sum = node
    //             .borrow()
    //             .forward
    //             .iter()
    //             .map(|n| n.borrow().reaches_9)
    //             .sum();
    //         node.borrow_mut().reaches_9 = sum;
    //     }
    // }

    nodes
        .iter()
        .filter(|n| n.borrow().num == 0)
        .map(|n| n.borrow().reaches_9)
        .sum()
    // nodes
    //     .into_iter()
    //     .filter(|node| node.borrow().num == 0)
    //     .map(|trailstart| dfs_distinct(trailstart.clone(), 9))
    //     .sum::<u32>()
}
