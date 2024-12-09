use std::str::FromStr;

use day5::Input;

fn main() {
    static INPUT: &str = include_str!("./input");
    let res = Input::from_str(INPUT).unwrap();
    println!("{}", res.part_one());
    println!("{}", res.part_two());
}
