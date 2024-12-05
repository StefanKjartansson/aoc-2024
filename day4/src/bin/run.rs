use day4::{count, count_mas, parse};

fn main() {
    static INPUT: &str = include_str!("./input");
    let res = parse(INPUT);
    println!("{}", count(&res));
    println!("{}", count_mas(&res));
}
