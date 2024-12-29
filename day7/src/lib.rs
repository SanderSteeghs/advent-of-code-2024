use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct EquationError;

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    ans: i64,
    nums: Vec<i64>,
}

fn concat(a: i64, b: i64) -> i64 {
    let num_digits = i64::ilog10(b) + 1;
    let result = a * i64::pow(10, num_digits as u32) + b;
    return result;
}

impl Equation {
    fn is_valid(&self, use_concat: bool) -> bool {

        #[derive(Debug)]
        struct Item {
            idx: usize,
            intermediate: i64,
        }

        let mut queue = VecDeque::new();
        queue.push_front(Item{idx: 0, intermediate: self.nums[0] * self.nums[1]});
        if use_concat {
            queue.push_front(Item{idx: 0, intermediate: concat(self.nums[0], self.nums[1])});
        }
        queue.push_front(Item{idx: 0, intermediate: self.nums[0] + self.nums[1]});

        while let Some(item) = queue.pop_front() {
            if item.intermediate > self.ans {
                continue;
            }

            if item.idx == self.nums.len() - 2 {
                if item.intermediate == self.ans {
                    return true;
                }
                continue;
            }

            let num = self.nums[item.idx + 2];
            if use_concat {
                queue.push_front(Item{idx: item.idx+1, intermediate: concat(item.intermediate, num)});
            }
            queue.push_front(Item{idx: item.idx+1, intermediate: item.intermediate * num});
            queue.push_front(Item{idx: item.idx+1, intermediate: item.intermediate + num});
        }

        false
    }
}

impl FromStr for Equation {
    type Err = EquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.find(':').ok_or(EquationError {})?;
        let (ans, nums) = s.split_at(mid);
        let ans = ans.parse::<i64>().map_err(|_| EquationError {})?;

        let nums: Result<Vec<i64>, _> =
            nums[2..].split(' ').map(|num| num.parse::<i64>()).collect();
        let nums = nums.map_err(|_| EquationError {})?;

        Ok(Equation { ans, nums })
    }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<Equation>().expect(&format!("invalid line \"{line}\"")))
        .filter(|equation| equation.is_valid(false))
        .map(|equation| equation.ans)
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    input
    .lines()
    .map(|line| line.parse::<Equation>().expect("invalid line"))
    .filter(|equation| equation.is_valid(true))
    .map(|equation| equation.ans)
    .sum::<i64>()
}
