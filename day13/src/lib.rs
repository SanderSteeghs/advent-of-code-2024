use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Line {
    a: i64,
    b: i64,
    c: i64,
}

impl Line {
    fn new(step_x: i64, step_y: i64, point: &Pos) -> Self {
        let (x1, y1) = (point.x, point.y);
        let a = step_y;
        let b = -(step_x);
        let c = ((-b) * y1) - (a * x1);

        Line { a, b, c }
    }

    fn intersection(&self, other: &Line) -> Pos {
        let a1 = self.a;
        let a2 = other.a;

        let b1 = self.b;
        let b2 = other.b;

        let c1 = self.c;
        let c2 = other.c;

        let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy = s
            .split(',')
            .map(|x| x.split('=').last().ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;

        let x = xy[0].parse::<i64>().map_err(|_| ())?;
        let y = xy[1].parse::<i64>().map_err(|_| ())?;

        Ok(Pos { x, y })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Button {
    name: String,
    cost: u32,
    x: i64,
    y: i64,
}

impl FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let name = split.next().ok_or(())?;
        let name = name.to_string();

        let xy = s
            .split(',')
            .map(|x| x.split('+').last().ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;

        let x = xy[0].parse::<i64>().map_err(|_| ())?;
        let y = xy[1].parse::<i64>().map_err(|_| ())?;

        let cost = if name == "Button A" {
            3
        } else if name == "Button B" {
            1
        } else {
            unimplemented!()
        };

        Ok(Button {
            cost,
            name: name.to_string(),
            x,
            y,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    buttons: Vec<Button>,
    prize: Pos,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let buttons = lines[0..lines.len() - 1]
            .iter()
            .take_while(|line| line.starts_with("Button"))
            .map(|line| line.parse::<Button>())
            .collect::<Result<Vec<_>, _>>()?;

        let prize = lines.last().expect("invalid line")["Prize: ".len()..].parse::<Pos>()?;
        Ok(Machine { buttons, prize })
    }
}

pub fn part1(input: &str) -> u64 {
    let machines = input
        .split("\n\n")
        .map(|m| m.parse::<Machine>().expect("invalid machine"));

    let mut result = 0;

    for machine in machines {
        assert!(machine.buttons.len() == 2);

        let p = &machine.prize;
        let (m1, m1_cost) = (&machine.buttons[0], machine.buttons[0].cost as i64);
        let (m2, m2_cost) = (&machine.buttons[1], machine.buttons[1].cost as i64);

        let l1 = Line::new(m1.x, m1.y, &Pos { x: 0, y: 0 });
        let l2 = Line::new(m2.x, m2.y, &machine.prize);
        let intersect = l1.intersection(&l2);

        // check if the intersection point could ever be reached by both lines
        if intersect.x % m1.x == 0
            && (p.x - intersect.x) % m2.x == 0
            && intersect.x >= 0
            && intersect.x <= p.x
        {
            let cost = (intersect.x / m1.x).abs() * m1_cost;
            let cost = cost + ((p.x - intersect.x) / m2.x).abs() * m2_cost;
            let cost = cost as u64;

            result += cost;
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let machines = input
        .split("\n\n")
        .map(|m| m.parse::<Machine>().expect("invalid machine"));

    let mut result = 0;

    for machine in machines {
        assert!(machine.buttons.len() == 2);

        let mut p = machine.prize;
        p.x += 10000000000000;
        p.y += 10000000000000;

        let (m1, m1_cost) = (&machine.buttons[0], machine.buttons[0].cost as i64);
        let (m2, m2_cost) = (&machine.buttons[1], machine.buttons[1].cost as i64);

        let l1 = Line::new(m1.x, m1.y, &Pos { x: 0, y: 0 });
        let l2 = Line::new(m2.x, m2.y, &p);
        let intersect = l1.intersection(&l2);

        // check if the intersection point could ever be reached by both lines
        if intersect.x % m1.x == 0
            && (p.x - intersect.x) % m2.x == 0
            && intersect.x >= 0
            && intersect.x <= p.x
        {
            let cost = (intersect.x / m1.x).abs() * m1_cost;
            let cost = cost + ((p.x - intersect.x) / m2.x).abs() * m2_cost;
            let cost = cost as u64;

            result += cost;
        }
    }

    result
}
