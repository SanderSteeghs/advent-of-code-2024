use core::fmt;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::{cell::RefCell, collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
enum Parent {
    Ref(String),
    Value(bool),
}

impl fmt::Display for Parent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Parent::Ref(r) => r,
            Parent::Value(v) => {
                if *v {
                    "1"
                } else {
                    "0"
                }
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
struct Gate {
    name: String,
    parent_a: Parent,
    parent_b: Parent,
    op: Op,
    value: Option<bool>,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(gate: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = gate.split(' ').collect();
        let parent_a = split[0].to_string();
        let op = match split[1] {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!(),
        };
        let parent_b = split[2].to_string();
        let name = split[4].to_string();
        let gate = Gate {
            name: name.clone(),
            parent_a: Parent::Ref(parent_a),
            parent_b: Parent::Ref(parent_b),
            op,
            value: None,
        };
        Ok(gate)
    }
}

impl Gate {
    fn parent_to_value(&mut self, parent_ref: &str, parent_value: bool) -> bool {
        if matches!(&self.parent_a, Parent::Ref(n) if n == parent_ref) {
            self.parent_a = Parent::Value(parent_value);
        }
        if matches!(&self.parent_b, Parent::Ref(n) if n == parent_ref) {
            self.parent_b = Parent::Value(parent_value);
        }

        match (self.parent_a.clone(), self.parent_b.clone()) {
            (Parent::Value(a), Parent::Value(b)) => {
                self.value = match self.op {
                    Op::And => Some(a & b),
                    Op::Or => Some(a | b),
                    Op::Xor => Some(a ^ b),
                };
                return true;
            }
            _ => false,
        }
    }
}

fn execute(gate_lookup: &HashMap<String, RefCell<Gate>>, x: u64, y: u64) -> u64 {
    let mut parent_to_gate: HashMap<String, Vec<_>> = HashMap::new();
    for (_, gate) in gate_lookup {
        if let Parent::Ref(parent_a) = &gate.borrow().parent_a {
            let entry = parent_to_gate.entry(parent_a.to_string()).or_default();
            entry.push(gate);
        }
        if let Parent::Ref(parent_b) = &gate.borrow().parent_b {
            let entry = parent_to_gate.entry(parent_b.to_string()).or_default();
            entry.push(gate);
        }
    }

    let mut todo = vec![];

    for i in 0..=44 {
        let x_name = format!("x{:02}", i);
        let x_value = (x >> i & 1) == 1;

        let y_name = format!("y{:02}", i);
        let y_value = (y >> i & 1) == 1;

        for (name, value) in [(x_name, x_value), (y_name, y_value)] {
            let Some(gates) = parent_to_gate.get_mut(&name) else {
                continue;
            };

            for gate in gates {
                if gate.borrow_mut().parent_to_value(&name, value) {
                    todo.push(gate.borrow().name.clone());
                }
            }
        }
    }

    while let Some(name) = todo.pop() {
        let curr_value = gate_lookup[&name].borrow().value.unwrap();
        if let Some(gates) = parent_to_gate.get(&name) {
            for gate_rc in gates {
                let mut gate = gate_rc.borrow_mut();
                if gate.parent_to_value(&name, curr_value) {
                    todo.push(gate.name.clone());
                }
            }
        }
    }

    let mut result = 0;
    for i in 0.. {
        let z = format!("z{:02}", i);
        let Some(gate) = gate_lookup.get(&z) else {
            return result;
        };

        let Some(value) = gate.borrow().value else {
            return result;
        };
        result |= (value as u64) << i;
    }

    unreachable!()
}

pub fn part1(input: &str) -> u64 {
    let (intials, gates) = input.split_once("\n\n").unwrap();

    let mut gate_lookup: HashMap<String, RefCell<Gate>> = HashMap::new();

    for gate in gates.lines() {
        let gate = gate.parse::<Gate>().unwrap();
        gate_lookup.insert(gate.name.clone(), RefCell::new(gate));
    }

    let mut x: u64 = 0;
    let mut y: u64 = 0;
    for initial in intials.lines() {
        let name = initial.split(':').next().unwrap().to_string();
        let value = initial.chars().last().unwrap();
        let value = value.to_digit(10).unwrap() as u64;

        if name.starts_with('x') {
            let bit = name[1..].parse::<u32>().unwrap();
            x |= value << bit;
        }

        if name.starts_with('y') {
            let bit = name[1..].parse::<u32>().unwrap();
            y |= value << bit;
        }
    }

    execute(&gate_lookup, x, y)
}

fn write_dot(gates: &[&Gate], file_name: &str, colors: &HashSet<String>) {
    let mut dot = "digraph graphname {\n".to_string();
    for gate in gates {
        let mut label = format!("[label=\"{}\\n{}\"", gate.name, gate.op);
        if colors.contains(&gate.name) {
            label.push_str(" color=red style=filled");
        }
        label.push(']');

        dot.push_str(&format!("\t{} {}\n", gate.name, label));
        dot.push_str(&format!("\t{} -> {}\n", gate.parent_a, gate.name));
        dot.push_str(&format!("\t{} -> {}\n", gate.parent_b, gate.name));
    }
    dot.push('}');
    let mut file = File::create(file_name).expect("could not create file");
    file.write_all(&dot.as_bytes())
        .expect("could not write file");
}

fn find_path(
    root: &str,
    target: &str,
    gate_lookup: &HashMap<String, RefCell<Gate>>,
) -> Vec<String> {
    let mut frontier: VecDeque<String> = VecDeque::new();
    let mut visited: HashMap<String, String> = HashMap::new();

    let root = gate_lookup[root].borrow();

    frontier.push_front(root.name.clone());
    visited.insert(root.name.clone(), root.name.to_string());

    while let Some(curr) = frontier.pop_front() {
        let Some(curr) = gate_lookup.get(&curr) else {
            continue;
        };
        let curr = curr.borrow();

        if curr.name == target {
            break;
        }

        match (curr.parent_a.clone(), curr.parent_b.clone()) {
            (Parent::Ref(a), Parent::Ref(b)) => {
                if !visited.contains_key(&a) {
                    visited.insert(a.clone(), curr.name.clone());
                    frontier.push_back(a.clone());
                }

                if !visited.contains_key(&b) {
                    visited.insert(b.clone(), curr.name.clone());
                    frontier.push_back(b.clone());
                }
            }
            _ => panic!(),
        }
    }

    /* Follow the White rabbit */
    let mut path: Vec<String> = Vec::new();

    let mut p = target.to_string();
    path.push(p.to_string());

    while p != root.name {
        let Some(next) = visited.get(&p) else {
            return path;
        };
        p = next.clone();
        path.push(p.to_string());
    }

    path.reverse();

    path
}

fn get_op_path(gates: &Vec<String>, gate_lookup: &HashMap<String, RefCell<Gate>>) -> Vec<Op> {
    if gates.is_empty() {
        return Vec::new();
    }

    gates
        .iter()
        .take(gates.len() - 1)
        .map(|x| gate_lookup[x].borrow().op.clone())
        .collect()
}

fn swap(a: &str, b: &str, gate_lookup: &mut HashMap<String, RefCell<Gate>>) {}

pub fn part2(input: &str) -> String {
    let (intials, gates) = input.split_once("\n\n").unwrap();

    let mut gate_lookup: HashMap<String, RefCell<Gate>> = HashMap::new();

    for gate in gates.lines() {
        let gate = gate.parse::<Gate>().unwrap();
        gate_lookup.insert(gate.name.clone(), RefCell::new(gate));
    }

    let borrowed: Vec<_> = gate_lookup.values().map(|rc| rc.borrow()).collect();
    let gates: Vec<&_> = borrowed.iter().map(|r| &**r).collect();
    // write_dot(&gates, "graph.dot");

    // let mut xi_zi: HashMap<_, Vec<Vec<String>>> = HashMap::new();
    // let mut yi_zi: HashMap<_, Vec<Vec<String>>> = HashMap::new();
    // let mut xi_zi1: HashMap<_, Vec<Vec<String>>> = HashMap::new();
    // let mut yi_zi1: HashMap<_, Vec<Vec<String>>> = HashMap::new();

    let xi_zi_path = vec![Op::Xor, Op::Xor];
    let yi_zi_path = vec![Op::Xor, Op::Xor];
    let xi_zi1_path = vec![Op::Xor, Op::Or, Op::And];
    let yi_zi1_path = vec![Op::Xor, Op::Or, Op::And];

    let mut wrong = HashSet::new();
    for i in 0..44 {
        let xi = format!("x{:02}", i);
        let yi = format!("y{:02}", i);
        let zi = format!("z{:02}", i);
        let zi1 = format!("z{:02}", i + 1);

        let mut valid = true;

        let xi_zi = find_path(&zi, &xi, &gate_lookup);
        if get_op_path(&xi_zi, &gate_lookup) != xi_zi_path {
            valid = false;
        }

        let yi_zi = find_path(&zi, &yi, &gate_lookup);
        if get_op_path(&yi_zi, &gate_lookup) != yi_zi_path {
            valid = false;
        }

        let xi_zi1 = find_path(&zi1, &xi, &gate_lookup);
        if get_op_path(&xi_zi1, &gate_lookup) != xi_zi1_path {
            valid = false;
        }

        let yi_zi1 = find_path(&zi1, &yi, &gate_lookup);
        if get_op_path(&yi_zi1, &gate_lookup) != yi_zi1_path {
            valid = false;
        }

        if !valid {
            wrong.extend(xi_zi);
            wrong.extend(yi_zi);
            wrong.extend(xi_zi1);
            wrong.extend(yi_zi1);
        }
    }

    write_dot(&gates, "graph.dot", &wrong);

    panic!("manual inspection required to solve the exersice..");
}

// faulty: mdg
