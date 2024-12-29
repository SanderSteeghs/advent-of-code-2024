use std::str::FromStr;

fn dv(num: u64, denum: u64) -> u64 {
    num / (2i64.pow(denum as u32)) as u64
}

fn xl(a: u64, b: u64) -> u64 {
    a ^ b
}

fn st(a: u64) -> u64 {
    a % 8
}

fn jnz(cond: u64, jmp: usize, curr_pc: usize) -> usize {
    if cond == 0 {
        return curr_pc + 2;
    }

    return jmp;
}

fn out(val: u64) -> u8 {
    (val % 8) as u8
}

#[derive(Clone)]
struct Program {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    program: Vec<u8>,
    pc: usize,
}

impl Program {
    fn run(mut self) -> Vec<u8> {
        let mut output = vec![];

        while self.pc < self.program.len() - 1 {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];

            let to_val = |op: u8| -> u64 {
                match op {
                    0..=3 => op as u64,
                    4 => self.reg_a,
                    5 => self.reg_b,
                    6 => self.reg_c,
                    7.. => panic!(),
                }
            };

            let mut new_pc = self.pc + 2;

            match opcode {
                0 => self.reg_a = dv(self.reg_a, to_val(operand)),
                1 => self.reg_b = xl(self.reg_b, operand as u64),
                2 => self.reg_b = st(to_val(operand)),
                3 => new_pc = jnz(self.reg_a, operand as usize, self.pc),
                4 => self.reg_b = xl(self.reg_b, self.reg_c),
                5 => output.push(out(to_val(operand))),
                6 => self.reg_b = dv(self.reg_a, to_val(operand)),
                7 => self.reg_c = dv(self.reg_a, to_val(operand)),
                _ => panic!(),
            }

            self.pc = new_pc;
        }

        output
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_reg(input: &str) -> u64 {
            input.split_whitespace().last().unwrap().parse().unwrap()
        }

        let mut lines = s.lines();
        let lines = lines.by_ref();

        let reg_a = parse_reg(lines.next().unwrap());
        let reg_b = parse_reg(lines.next().unwrap());
        let reg_c = parse_reg(lines.next().unwrap());

        lines.next();

        let program: Vec<u8> = lines.next().unwrap()["Program: ".len()..]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Program {
            reg_a,
            reg_b,
            reg_c,
            program,
            pc: 0,
        })
    }
}

pub fn part1(input: &str) -> Vec<u8> {
    let mut program = input.parse::<Program>().unwrap();
    program.reg_a = 48378511622144;
    program.run()
}

pub fn part2(input: &str) -> u64 {
    panic!("did this one by hand.. sorry!");
}
