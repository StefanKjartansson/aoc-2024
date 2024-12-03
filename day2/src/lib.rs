#[derive(Debug, PartialEq)]
pub enum ReportMode {
    Normal,
    Dampener,
}

#[derive(Debug, PartialEq)]
pub enum ReportType {
    Safe,
    Unsafe,
}

enum Direction {
    Increasing,
    Decreasing,
}

pub fn parse(s: &str) -> Vec<Vec<i32>> {
    let mut a = Vec::new();
    s.lines().for_each(|line| {
        let mut vec = Vec::new();
        let parts = line.trim().split(" ");
        parts.for_each(|part| {
            let x = part.parse().unwrap();
            vec.push(x);
        });
        a.push(vec);
    });
    a
}

pub fn is_monotonic_with_threshold(v: &Vec<i32>, threshold: i32) -> ReportType {
    let mut direction = Direction::Decreasing;

    for (i, x) in v.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let prev = v[i - 1];
        if i == 1 && prev < *x {
            direction = Direction::Increasing;
        }
        let diff = match direction {
            Direction::Increasing => *x - prev,
            Direction::Decreasing => prev - *x,
        };
        if diff < 1 {
            return ReportType::Unsafe;
        }
        if diff > threshold {
            return ReportType::Unsafe;
        }
    }
    ReportType::Safe
}

pub fn is_monotonic_with_dampener(v: &Vec<i32>, threshold: i32) -> ReportType {
    if is_monotonic_with_threshold(v, threshold) == ReportType::Safe {
        return ReportType::Safe;
    }
    for i in 0..v.len() {
        let mut new_v = v.clone();
        new_v.remove(i);
        if is_monotonic_with_threshold(&new_v, threshold) == ReportType::Safe {
            return ReportType::Safe;
        }
    }
    ReportType::Unsafe
}

pub fn count_safe(v: &Vec<Vec<i32>>, mode: ReportMode) -> i32 {
    let mut count = 0;
    for x in v {
        if mode == ReportMode::Normal {
            if is_monotonic_with_threshold(x, 3) == ReportType::Safe {
                count += 1;
            }
        } else if mode == ReportMode::Dampener {
            if is_monotonic_with_dampener(x, 3) == ReportType::Safe {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .trim();
        let v = parse(data);
        assert_eq!(is_monotonic_with_threshold(&v[0], 3), ReportType::Safe);
        assert_eq!(is_monotonic_with_threshold(&v[1], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_threshold(&v[2], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_threshold(&v[3], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_threshold(&v[4], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_threshold(&v[5], 3), ReportType::Safe);
        assert_eq!(count_safe(&v, ReportMode::Normal), 2);

        assert_eq!(is_monotonic_with_dampener(&v[0], 3), ReportType::Safe);
        assert_eq!(is_monotonic_with_dampener(&v[1], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_dampener(&v[2], 3), ReportType::Unsafe);
        assert_eq!(is_monotonic_with_dampener(&v[3], 3), ReportType::Safe);
        assert_eq!(is_monotonic_with_dampener(&v[4], 3), ReportType::Safe);
        assert_eq!(is_monotonic_with_dampener(&v[5], 3), ReportType::Safe);
    }
}
