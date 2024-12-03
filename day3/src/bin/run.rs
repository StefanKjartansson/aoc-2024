use day3::{mul, Mode};

fn main() {
    static INPUT: &str = include_str!("./input");
    let res = mul(INPUT, Mode::Normal);
    println!("{}", res);
    let cres = mul(INPUT, Mode::Conditional);
    println!("{}", cres);
}
