use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    s.lines().for_each(|line| {
        let mut parts = line.trim().split("   ");
        let x = parts.next().unwrap().parse().unwrap();
        a.push(x);
        let y = parts.next().unwrap().parse().unwrap();
        b.push(y);
    });
    (a, b)
}

pub fn total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();
    for x in left {
        left_heap.push(Reverse(x));
    }
    for x in right {
        right_heap.push(Reverse(x));
    }
    let mut distance = 0;
    while !left_heap.is_empty() && !right_heap.is_empty() {
        let left = left_heap.pop().unwrap().0;
        let right = right_heap.pop().unwrap().0;
        if right > left {
            distance += right - left;
        } else {
            distance += left - right;
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    static input: &str = include_str!("input");

    #[test]
    fn exampl_test() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
        let (a, b) = parse(data);

        assert_eq!(total_distance(a, b), 11);
    }
}
