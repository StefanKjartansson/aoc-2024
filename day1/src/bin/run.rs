use day1::{parse, total_distance};

fn main() {
    static input: &str = include_str!("input");
    let (a, b) = parse(input);
    let distance = total_distance(a, b);
    println!("{}", distance);
}
