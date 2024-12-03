#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Conditional,
}

pub fn parse_normal(s: &str) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for (idx, c) in s.chars().enumerate() {
        if c == '(' {
            if s[idx - 3..idx] != *"mul" {
                continue;
            }
            let start = idx + 1;
            let mut end = start;
            while s.chars().nth(end).unwrap() != ')' {
                end += 1;
            }
            let mut parts = s[start..end].split(",");
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    if let Ok(x) = first.parse() {
                        if let Ok(y) = second.parse() {
                            out.push((x, y));
                            continue;
                        }
                    }
                }
            }
        }
    }
    out
}

pub fn parse_conditional(s: &str) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    let mut should_append = true;
    for (idx, c) in s.chars().enumerate() {
        if c == '(' {
            if idx - 4 > 0 && s[idx - 5..idx] == *"don't" {
                should_append = false;
                continue;
            }
            if idx - 1 > 0 && s[idx - 2..idx] == *"do" {
                should_append = true;
                continue;
            }
            if idx - 2 > 0 && s[idx - 3..idx] != *"mul" {
                continue;
            }
            let start = idx + 1;
            let mut end = start;
            while s.chars().nth(end).unwrap() != ')' {
                end += 1;
            }
            let mut parts = s[start..end].split(",");
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    if let Ok(x) = first.parse() {
                        if let Ok(y) = second.parse() {
                            if should_append {
                                out.push((x, y));
                            }
                            continue;
                        }
                    }
                }
            }
        }
    }
    out
}

pub fn mul(s: &str, mode: Mode) -> i32 {
    let f = match mode {
        Mode::Normal => parse_normal,
        Mode::Conditional => parse_conditional,
    };
    let v = f(s);
    let mut result = 0;
    for (x, y) in v {
        result += x * y;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mode() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let v = parse_normal(data);
        assert_eq!(v, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
        assert_eq!(mul(data, Mode::Normal), 161);
    }

    #[test]
    fn test_conditional_mode() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let v = parse_conditional(data);
        assert_eq!(v, vec![(2, 4), (8, 5)]);
        assert_eq!(mul(data, Mode::Conditional), 48);
    }
}
