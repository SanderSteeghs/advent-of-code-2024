const MUL_START: &str = "mul(";
const MUL_SEP: char = ',';
const MUL_END: char = ')';

fn parse_mul(string: &str) -> Option<(i32, i32)> {
    let mut parts = string.split(MUL_SEP);

    let a = parts.next().map(|n| n.parse::<i32>())?;
    let b = parts.next().map(|n| n.parse::<i32>())?;
    if parts.next().is_some() {
        return None;
    }

    if let (Ok(a), Ok(b)) = (a, b) {
        Some((a, b))
    } else {
        None
    }
}

pub fn parse_muls(string: &str, conditional: bool) -> i32 {
    let mut result = 0;
    let mut remaining = string;
    let mut enabled = true;

    while let Some(idx) = remaining.find(MUL_START) {
        // for the next "mul(" find if there are any do() or don't()'s
        if conditional {
            let pre = &remaining[..idx];
            let disable = pre.rfind("don't()");
            let enable = pre.rfind("do()");

            enabled = match (disable, enable) {
                (Some(disable), Some(enable)) => disable < enable,
                (Some(_), None) => false,
                (None, Some(_)) => true,
                _ => enabled,
            };
        }

        // skip "mul(" in the remaing string
        remaining = &remaining[idx + MUL_START.len()..];

        // then see if we are enabled or not
        if conditional && !enabled {
            continue;
        }

        // now we skip to the end of the mul operator ')'
        let Some(end_idx) = remaining.find(MUL_END) else {
            continue;
        };

        // and see if the captured content matches our criterea
        let content = &remaining[..end_idx];
        if let Some((a, b)) = parse_mul(content) {
            result += a * b; // update result
            remaining = &remaining[end_idx + 1..]; // skip the found mul
        }
    }

    result
}


pub fn parse_muls2(string: &str, conditional: bool) -> i32 {
    if !conditional {
        return string.split("mul(")
            .filter_map(|str| str.find(')').map(|idx| &str[..idx]))
            .filter_map(parse_mul)
            .fold(0, |x, (a, b)| x + (a * b));
    }

    string.split(')')
        .scan(true, |enabled, str| {
            if str.ends_with("do(") {
                *enabled = true;
                return Some(None);
            }
            if str.ends_with("don't(") {
                *enabled = false;
                return Some(None);
            }
            if !(*enabled) {
                return Some(None);
            }
            str
                .rfind("mul(")
                .map(|idx| &str[idx + 4..])
                .map(parse_mul)
                .or(Some(None))
        })
        .flatten()
        .fold(0, |x, (a, b)| x + (a * b))
}