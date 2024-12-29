pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut result = 0;

    // horizontals
    for line in input.lines() {
        result += line.matches("XMAS").count();
        result += line.matches("SAMX").count();
    }

    // verticals
    for i in 0..width {
        let mut str = String::new();
        for j in 0..height {
            str.push(lines[j][i]);
        }
        result += str.matches("XMAS").count();
        result += str.matches("SAMX").count();
    }

    // diagonals
    let mut ctr = 0;
    while ctr < 2 * height - 1 {
        let mut str = String::new();
        let mut str2 = String::new();
        for i in 0..width {
            for j in 0..height {
                if i + j == ctr {
                    str.push(lines[i][j]);
                    str2.push(lines[j][i]);
                }
            }
        }
        result += str.matches("XMAS").count();
        result += str.matches("SAMX").count();

        if str2.len() != height {
            result += str2.matches("XMAS").count();
            result += str2.matches("SAMX").count();
        }

        ctr += 1;
    }

    result
}

pub fn part2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let chars: Vec<_> = input.lines().flat_map(|l| l.chars().map(|c| c)).collect();
    chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'A')
        .map(|(idx, _)| idx)
        .filter(|idx| {
            idx % width > 0
                && idx % width < width - 1
                && *idx > width + 1
                && *idx < width * (height - 1) + 1
        })
        .filter(|idx| {
            let idx = *idx;

            let top_left = chars[idx - width - 1];
            let top_right = chars[idx - width + 1];
            let bottom_left = chars[idx + width - 1];
            let bottom_right = chars[idx + width + 1];

            let m1 = match (top_left, bottom_right) {
                ('M', 'S') => true,
                ('S', 'M') => true,
                _ => false,
            };

            let m2 = match (top_right, bottom_left) {
                ('M', 'S') => true,
                ('S', 'M') => true,
                _ => false,
            };

            m1 && m2
        })
        .count()
}
