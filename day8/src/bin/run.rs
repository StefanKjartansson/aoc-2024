use day8::Grid;

pub fn main() {
    static INPUT: &str = include_str!("./input");
    let grid: Grid = INPUT.parse().unwrap();
    let marked: Grid = grid.part_one();
    println!("{}", marked);
    println!("{}", marked.count_antinodes());
    println!("{}", grid.part_two().count_antinodes());
}
