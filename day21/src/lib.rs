use std::collections::HashMap;

struct Directional;

impl Directional {
    fn get_len(from: char, to: char) -> u64 {
        match (from, to) {
            ('A', 'A') => 0,
            ('A', '^') => 1,
            ('A', '<') => 3,
            ('A', 'v') => 2,
            ('A', '>') => 1,

            ('^', 'A') => 1,
            ('^', '^') => 0,
            ('^', '<') => 2,
            ('^', 'v') => 1,
            ('^', '>') => 2,

            ('<', 'A') => 3,
            ('<', '^') => 2,
            ('<', '<') => 0,
            ('<', 'v') => 1,
            ('<', '>') => 2,

            ('v', 'A') => 2,
            ('v', '^') => 1,
            ('v', '<') => 1,
            ('v', 'v') => 0,
            ('v', '>') => 1,

            ('>', 'A') => 1,
            ('>', '^') => 2,
            ('>', '<') => 2,
            ('>', 'v') => 1,
            ('>', '>') => 0,

            _ => panic!(),
        }
    }

    fn get_move(from: char, r#move: char) -> char {
        match (from, r#move) {
            ('A', '^') => panic!(),
            ('A', '<') => '^',
            ('A', 'v') => '>',
            ('A', '>') => panic!(),

            ('^', '^') => panic!(),
            ('^', '<') => panic!(),
            ('^', 'v') => 'v',
            ('^', '>') => 'A',

            ('<', '^') => panic!(),
            ('<', '<') => panic!(),
            ('<', 'v') => panic!(),
            ('<', '>') => 'v',

            ('v', '^') => '^',
            ('v', '<') => '<',
            ('v', 'v') => panic!(),
            ('v', '>') => '>',

            ('>', '^') => 'A',
            ('>', '<') => 'v',
            ('>', 'v') => panic!(),
            ('>', '>') => panic!(),

            _ => panic!(),
        }
    }

    fn get_next_target(from: char, to: char) -> Vec<char> {
        match (from, to) {
            ('A', 'A') => vec![],
            ('A', '^') => vec!['<'],
            ('A', '<') => vec!['<', 'v'],
            ('A', 'v') => vec!['<', 'v'],
            ('A', '>') => vec!['v'],

            ('^', 'A') => vec!['>'],
            ('^', '^') => vec![],
            ('^', '<') => vec!['v'],
            ('^', 'v') => vec!['v'],
            ('^', '>') => vec!['>', 'v'],

            ('<', 'A') => vec!['>'],
            ('<', '^') => vec!['>'],
            ('<', '<') => vec![],
            ('<', 'v') => vec!['>'],
            ('<', '>') => vec!['>'],

            ('v', 'A') => vec!['^', '>'],
            ('v', '^') => vec!['^'],
            ('v', '<') => vec!['<'],
            ('v', 'v') => vec![],
            ('v', '>') => vec!['>'],

            ('>', 'A') => vec!['^'],
            ('>', '^') => vec!['^', '<'],
            ('>', '<') => vec!['<'],
            ('>', 'v') => vec!['<'],
            ('>', '>') => vec![],

            _ => panic!(),
        }
    }
}

fn get_path_len(
    from: &Vec<char>,
    to: &Vec<char>,
    cache: &mut HashMap<(Vec<char>, Vec<char>), u64>,
) -> u64 {
    fn _get_path_len(
        from: Vec<char>,
        to: Vec<char>,
        curr: Vec<char>,
        cache: &mut HashMap<(Vec<char>, Vec<char>), u64>,
    ) -> u64 {
        if let Some(score) = cache.get(&(from.clone(), to.clone())) {
            return *score;
        }

        // base case: we are at the first robot
        if from.len() == 1 && to.len() == 1 {
            let score = Directional::get_len(curr[0], to[0]);
            return score;
        }

        // get the original desired moving position
        let curr: Vec<_> = curr.iter().cloned().rev().take(to.len()).rev().collect();

        let replacements = Directional::get_next_target(from[0], to[0]);
        if replacements.len() == 0 {
            let score = _get_path_len(
                from[1..].to_vec(),
                to[1..].to_vec(),
                curr[1..].to_vec(),
                cache,
            );
            cache.insert((from, to), score);
            return score;
        }

        for replace in replacements {
            let mut score = 0;
            let mut curr = curr.clone();

            // move towards the correct n+1 position
            {
                let mut to = to.clone();
                to[1] = replace;
                score += _get_path_len(
                    from[1..].to_vec(),
                    to[1..].to_vec(),
                    curr[1..].to_vec(),
                    cache,
                );
            }
            curr[1] = replace;

            // take a move
            let next_move = Directional::get_move(curr[0], curr[1]);
            curr[0] = next_move;
            score += 1; // for pressing A once

            // continue moving
            score += _get_path_len(curr.clone(), to.clone(), curr.clone(), cache);

            let entry = cache.entry((from.clone(), to.clone())).or_insert(score);
            if score < *entry {
                *entry = score;
            }
        }

        *cache.get(&(from, to)).unwrap()
    }

    _get_path_len(from.to_vec(), to.to_vec(), from.to_vec(), cache)
}

struct Numpad;

impl Numpad {
    fn get_next_target(from: char, to: char) -> Vec<char> {
        match (from, to) {
            ('A', '0') => vec!['0'],
            ('A', '1') => vec!['0', '3'],
            ('A', '2') => vec!['0', '3'],
            ('A', '3') => vec!['3'],
            ('A', '4') => vec!['0', '3'],
            ('A', '5') => vec!['0', '3'],
            ('A', '6') => vec!['3'],
            ('A', '7') => vec!['0', '3'],
            ('A', '8') => vec!['0', '3'],
            ('A', '9') => vec!['3'],
            ('A', 'A') => vec![],

            ('0', '0') => vec![],
            ('0', '1') => vec!['2'],
            ('0', '2') => vec!['2'],
            ('0', '3') => vec!['2', 'A'],
            ('0', '4') => vec!['2'],
            ('0', '5') => vec!['2'],
            ('0', '6') => vec!['2', 'A'],
            ('0', '7') => vec!['2'],
            ('0', '8') => vec!['2'],
            ('0', '9') => vec!['2', 'A'],
            ('0', 'A') => vec!['A'],

            ('1', '0') => vec!['2'],
            ('1', '1') => vec![],
            ('1', '2') => vec!['2'],
            ('1', '3') => vec!['2'],
            ('1', '4') => vec!['4'],
            ('1', '5') => vec!['2', '4'],
            ('1', '6') => vec!['2', '4'],
            ('1', '7') => vec!['4'],
            ('1', '8') => vec!['2', '4'],
            ('1', '9') => vec!['2', '4'],
            ('1', 'A') => vec!['2'],

            ('2', '0') => vec!['0'],
            ('2', '1') => vec!['1'],
            ('2', '2') => vec![],
            ('2', '3') => vec!['3'],
            ('2', '4') => vec!['1', '5'],
            ('2', '5') => vec!['5'],
            ('2', '6') => vec!['6'],
            ('2', '7') => vec!['1', '5'],
            ('2', '8') => vec!['5'],
            ('2', '9') => vec!['5', '3'],
            ('2', 'A') => vec!['0', '3'],

            ('3', '0') => vec!['2', 'A'],
            ('3', '1') => vec!['2'],
            ('3', '2') => vec!['2'],
            ('3', '3') => vec![],
            ('3', '4') => vec!['2', '6'],
            ('3', '5') => vec!['2', '6'],
            ('3', '6') => vec!['6'],
            ('3', '7') => vec!['2', '6'],
            ('3', '8') => vec!['2', '6'],
            ('3', '9') => vec!['6'],
            ('3', 'A') => vec!['A'],

            ('4', '0') => vec!['1', '5'],
            ('4', '1') => vec!['1'],
            ('4', '2') => vec!['2', '5'],
            ('4', '3') => vec!['1', '5'],
            ('4', '4') => vec![],
            ('4', '5') => vec!['5'],
            ('4', '6') => vec!['5'],
            ('4', '7') => vec!['7'],
            ('4', '8') => vec!['5', '7'],
            ('4', '9') => vec!['5', '7'],
            ('4', 'A') => vec!['1', '5'],

            ('5', '0') => vec!['2'],
            ('5', '1') => vec!['2', '4'],
            ('5', '2') => vec!['2'],
            ('5', '3') => vec!['2', '6'],
            ('5', '4') => vec!['4'],
            ('5', '5') => vec![],
            ('5', '6') => vec!['6'],
            ('5', '7') => vec!['4', '8'],
            ('5', '8') => vec!['8'],
            ('5', '9') => vec!['8', '6'],
            ('5', 'A') => vec!['2', '6'],

            ('6', '0') => vec!['5', '3'],
            ('6', '1') => vec!['5', '3'],
            ('6', '2') => vec!['5', '3'],
            ('6', '3') => vec!['3'],
            ('6', '4') => vec!['5'],
            ('6', '5') => vec!['5'],
            ('6', '6') => vec![],
            ('6', '7') => vec!['5', '9'],
            ('6', '8') => vec!['5', '9'],
            ('6', '9') => vec!['9'],
            ('6', 'A') => vec!['3'],

            ('7', '0') => vec!['4', '8'],
            ('7', '1') => vec!['4'],
            ('7', '2') => vec!['4', '8'],
            ('7', '3') => vec!['4', '8'],
            ('7', '4') => vec!['4'],
            ('7', '5') => vec!['4', '8'],
            ('7', '6') => vec!['4', '8'],
            ('7', '7') => vec![],
            ('7', '8') => vec!['8'],
            ('7', '9') => vec!['8'],
            ('7', 'A') => vec!['4', '8'],

            ('8', '0') => vec!['5'],
            ('8', '1') => vec!['5', '7'],
            ('8', '2') => vec!['5'],
            ('8', '3') => vec!['5', '9'],
            ('8', '4') => vec!['5', '7'],
            ('8', '5') => vec!['5'],
            ('8', '6') => vec!['5', '9'],
            ('8', '7') => vec!['7'],
            ('8', '8') => vec![],
            ('8', '9') => vec!['9'],
            ('8', 'A') => vec!['5', '9'],

            ('9', '0') => vec!['8', '6'],
            ('9', '1') => vec!['8', '6'],
            ('9', '2') => vec!['8', '6'],
            ('9', '3') => vec!['6'],
            ('9', '4') => vec!['8', '6'],
            ('9', '5') => vec!['8', '6'],
            ('9', '6') => vec!['6'],
            ('9', '7') => vec!['8'],
            ('9', '8') => vec!['8'],
            ('9', '9') => vec![],
            ('9', 'A') => vec!['6'],

            _ => vec![],
        }
    }

    fn get_dir(from: char, to: char) -> char {
        match (from, to) {
            ('A', '0') => '<',
            ('A', '3') => '^',

            ('0', '2') => '^',
            ('0', 'A') => '>',

            ('1', '4') => '^',
            ('1', '2') => '>',

            ('2', '1') => '<',
            ('2', '5') => '^',
            ('2', '3') => '>',
            ('2', '0') => 'v',

            ('3', '2') => '<',
            ('3', '6') => '^',
            ('3', 'A') => 'v',

            ('4', '1') => 'v',
            ('4', '5') => '>',
            ('4', '7') => '^',

            ('5', '4') => '<',
            ('5', '8') => '^',
            ('5', '6') => '>',
            ('5', '2') => 'v',

            ('6', '5') => '<',
            ('6', '9') => '^',
            ('6', '3') => 'v',

            ('7', '4') => 'v',
            ('7', '8') => '>',

            ('8', '5') => 'v',
            ('8', '7') => '<',
            ('8', '9') => '>',

            ('9', '8') => '<',
            ('9', '6') => 'v',

            _ => panic!("{from} {to}"),
        }
    }
}

fn get_paths(curr: char, code: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    fn _get_paths(curr: char, code: &Vec<char>, path: &mut Vec<char>, result: &mut Vec<Vec<char>>) {
        if code.len() == 0 {
            result.push(path.to_vec());
            return;
        }

        let target = code[0];
        if curr == target {
            path.push('A');
            _get_paths(curr, &code[1..].to_vec(), path, result);
        }

        let nexts = Numpad::get_next_target(curr, target);
        for next in nexts {
            let mut path = path.clone();
            let dir = Numpad::get_dir(curr, next);
            path.push(dir);
            _get_paths(next, code, &mut path, result);
        }
    }

    let mut path = vec!['A'];
    _get_paths(curr, &code.chars().collect(), &mut path, &mut result);

    result
}

fn run<const NUM_ROBOTS: usize>(input: &str) -> u64 {
    let mut cache= HashMap::new();
    let mut result = 0;

    for line in input.lines() {
        let mut scores = vec![];
        let paths = get_paths('A', line);

        for path in paths {
            let mut len = path.len() as u64 - 1;

            for window in path.windows(2) {
                let from = &window[0];
                let to = &window[1];

                let mut start = ['A'; NUM_ROBOTS];
                let mut goal = ['A'; NUM_ROBOTS];

                start[0] = *from;
                goal[0] = *to;

                let fast_score = get_path_len(&start.to_vec(), &goal.to_vec(), &mut cache);
                len += fast_score;
            }

            scores.push(len);
        }
        let len = *scores.iter().min().unwrap();

        result += len * line[..line.len() - 1].parse::<u64>().unwrap();
    }

    result
}

pub fn part1(input: &str) -> u64 {
    run::<2>(input)
}

pub fn part2(input: &str) -> u64 {
    run::<25>(input)
}
