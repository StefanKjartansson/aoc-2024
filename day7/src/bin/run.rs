use day7::{parse, part_one, part_two, Equation};

pub fn main() {
    static INPUT: &str = include_str!("./input");
    let equations: Vec<Equation> = parse(INPUT);
    println!("{}", part_one(&equations));
    println!("{}", part_two(&equations));
}
