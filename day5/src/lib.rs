use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn is_valid_ordering(rule: &Vec<i32>, ordering: &HashMap<i32, HashSet<i32>>) -> bool {
    if rule.len() == 0 {
        return false;
    }
    for (i, val) in rule.iter().enumerate() {
        if !ordering.contains_key(&val) {
            continue;
        }
        // look back, if any of the previous values are in the map, then it's invalid
        for j in 0..i {
            if ordering[&val].contains(&rule[j]) {
                return false;
            }
        }
        // look forward, if any of the next values are not in the map, then it's invalid
        for j in i + 1..rule.len() {
            if !ordering[&val].contains(&rule[j]) {
                return false;
            }
        }
    }
    true
}

fn sort_invalid_rule(rule: &Vec<i32>, ordering: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let rule_set: HashSet<i32> = rule.clone().into_iter().collect();
    let mut index_to_override = HashMap::new();

    for num in rule {
        if !ordering.contains_key(num) {
            continue;
        }
        index_to_override.insert(
            rule_set
                .intersection(&ordering[&num])
                .collect::<Vec<&i32>>()
                .len(),
            num,
        );
    }
    println!("ito: {:?}", index_to_override);

    let mut new_line = rule.clone();
    for (k, v) in index_to_override.into_iter() {
        new_line[k] = *v;
    }
    new_line
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    ordering: HashMap<i32, HashSet<i32>>,
    rules: Vec<Vec<i32>>,
}

impl Input {
    pub fn part_one(&self) -> i32 {
        let mut sum = 0;
        for r in self.rules.iter() {
            if is_valid_ordering(&r, &self.ordering) {
                let mid = r.len() / 2;
                sum += r[mid];
            }
        }
        sum
    }

    pub fn part_two(&self) -> i32 {
        let mut sum = 0;
        for r in self.rules.iter() {
            if is_valid_ordering(&r, &self.ordering) {
                continue;
            }
            let new_rule = sort_invalid_rule(&r, &self.ordering);
            println!("rule: {:?} -> {:?}", r, new_rule);
            let mid = new_rule.len() / 2;
            sum += new_rule[mid];
        }
        sum
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Input::default();
        let mut rules = false;
        s.lines().for_each(|line| {
            if line.trim().is_empty() {
                rules = true;
                return;
            }
            if !rules {
                let mut parts = line.trim().split("|");
                let before: i32 = parts.next().unwrap().parse().unwrap();
                let after: i32 = parts.next().unwrap().parse().unwrap();
                let v = input
                    .ordering
                    .entry(before)
                    .or_insert(HashSet::<i32>::new());
                v.insert(after);
            } else {
                let parts = line.trim().split(",");
                let mut vec = Vec::new();
                parts.for_each(|part| {
                    let x = part.parse().unwrap();
                    vec.push(x);
                });
                input.rules.push(vec);
            }
        });
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = Input::from_str(&input).unwrap();
        assert_eq!(result.rules.len(), 6);
        println!("{:#?}", result);
        assert_eq!(result.part_one(), 143);
        assert_eq!(result.part_two(), 123);
        //assert_eq!(1, 2);
    }
}
