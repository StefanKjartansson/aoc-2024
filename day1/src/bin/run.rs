use day1::{parse, similarity_score, total_distance};

fn main() {
    static input: &str = include_str!("./input");
    let (a, b) = parse(input);
    let distance = total_distance(&a, &b);
    println!("{}", distance);
    let sim = similarity_score(&a, &b);
    println!("{}", sim);
}
