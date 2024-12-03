use day2::{count_safe, parse, ReportMode};

fn main() {
    static input: &str = include_str!("./input");
    let v = parse(input);
    let count = count_safe(&v, ReportMode::Normal);
    println!("{}", count);
    let count = count_safe(&v, ReportMode::Dampener);
    println!("{}", count);
}
