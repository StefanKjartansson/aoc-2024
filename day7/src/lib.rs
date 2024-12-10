use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    Concatenation,
}

fn dfs(nums: &[i64], current: i64, results: &mut Vec<i64>, mode: Mode) {
    if nums.is_empty() {
        results.push(current);
        return;
    }
    let next = nums[0];
    let rest = &nums[1..];
    dfs(rest, next + current, results, mode);
    dfs(rest, next * current, results, mode);
    if mode == Mode::Concatenation {
        if let Ok(val) = format!("{}{}", current, next).parse::<i64>() {
            dfs(rest, val, results, mode);
        }
    }
}

pub struct Equation {
    pub target: i64,
    pub nums: Vec<i64>,
}

impl Equation {
    pub fn part_one(&self) -> i64 {
        let mut results = vec![];
        dfs(&self.nums, 0, &mut results, Mode::Normal);
        if results.contains(&self.target) {
            self.target
        } else {
            0
        }
    }

    pub fn part_two(&self) -> i64 {
        let mut results = vec![];
        dfs(&self.nums, 0, &mut results, Mode::Normal);
        if results.contains(&self.target) {
            self.target
        } else {
            dfs(&self.nums, 0, &mut results, Mode::Concatenation);
            if results.contains(&self.target) {
                self.target
            } else {
                0
            }
        }
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let target_s = parts.next().unwrap();
        let target = target_s[..target_s.len() - 1].parse().unwrap();
        let mut nums = Vec::new();
        while let Some(val) = parts.next() {
            nums.push(val.parse().unwrap());
        }
        Ok(Equation { target, nums })
    }
}

pub fn parse(s: &str) -> Vec<Equation> {
    s.lines()
        .map(|line| Equation::from_str(line).unwrap())
        .collect()
}

pub fn part_one(eqs: &Vec<Equation>) -> i64 {
    eqs.iter().map(|eq| eq.part_one()).sum()
}

pub fn part_two(eqs: &Vec<Equation>) -> i64 {
    eqs.iter().map(|eq| eq.part_two()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = parse(input);
        assert_eq!(result.len(), 9);
        assert_eq!(part_one(&result), 3749);
        assert_eq!(part_two(&result), 11387);
    }
}
