use day6::Grid;

pub fn main() {
    static INPUT: &str = include_str!("./input");
    let grid: Grid = INPUT.parse().unwrap();
    println!("{}", grid.clone().part_one());
    println!("{}", grid.clone().part_two());
}
