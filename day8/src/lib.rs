use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Frequency(char),
    Antinode,
    Multiple(Vec<Cell>),
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub matrix: Vec<Vec<Cell>>,
}

impl Grid {
    fn set_antinode(matrix: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
        matrix[row][col] = match &matrix[row][col] {
            Cell::Empty => Cell::Antinode,
            Cell::Frequency(prev) => Cell::Multiple(vec![Cell::Frequency(*prev), Cell::Antinode]),
            Cell::Antinode => Cell::Antinode,
            Cell::Multiple(vec) => {
                let mut next = vec.clone();
                if next.contains(&Cell::Antinode) {
                    return;
                }
                next.push(Cell::Antinode);
                Cell::Multiple(next.to_vec())
            }
        };
    }

    fn within_bounds(x: i32, y: i32, grid_size: i32) -> bool {
        x >= 0 && y >= 0 && x < grid_size && y < grid_size
    }

    fn check(grid_size: i32, antinodes: &HashSet<(i32, i32)>, x: i32, y: i32) -> bool {
        return if x < 0 || y < 0 || x > grid_size - 1 || y > grid_size - 1 {
            false
        } else {
            !antinodes.contains(&(x, y))
        };
    }

    fn count_frequencies(&self) -> HashMap<char, Vec<(i32, i32)>> {
        let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() {
                if let Cell::Frequency(c) = self.matrix[row][col] {
                    // add all different frequencies to a hashmap [c].push((row, col))
                    frequencies
                        .entry(c)
                        .or_insert(Vec::new())
                        .push((row as i32, col as i32));
                }
            }
        }
        frequencies
    }

    pub fn part_one(&self) -> Grid {
        let mut matrix = self.matrix.clone();
        let mut antinodes = HashSet::new();

        let row_len = matrix.len();
        let frequencies = self.count_frequencies();

        for (_, coords) in frequencies.iter() {
            if coords.len() < 2 {
                continue;
            }
            let mut it = coords.iter().combinations(2);
            while let Some(pair) = it.next() {
                let from = pair[0];
                let to = pair[1];
                if from.0 == to.0 || from.1 == to.1 {
                    continue;
                }
                let dx = to.0 - from.0;
                let dy = to.1 - from.1;

                let pt1 = (to.0 + dx, to.1 + dy);
                let pt2 = (from.0 - dx, from.1 - dy);
                if Self::check(row_len as i32, &antinodes, pt1.0, pt1.1) {
                    antinodes.insert(pt1);
                }
                if Self::check(row_len as i32, &antinodes, pt2.0, pt2.1) {
                    antinodes.insert(pt2);
                }
            }
        }
        antinodes.iter().for_each(|(x, y)| {
            Self::set_antinode(&mut matrix, *x as usize, *y as usize);
        });
        Grid { matrix }
    }

    pub fn part_two(&self) -> Grid {
        let mut matrix = self.matrix.clone();
        let mut antinodes = HashSet::new();
        let row_len = matrix.len();
        let frequencies = self.count_frequencies();
        for (_, coords) in frequencies.iter() {
            if coords.len() < 2 {
                continue;
            }
            let mut it = coords.iter().combinations(2);
            while let Some(pair) = it.next() {
                let from = pair[0];
                let to = pair[1];
                if from.0 == to.0 || from.1 == to.1 {
                    continue;
                }
                let dx = to.0 - from.0;
                let dy = to.1 - from.1;

                let mut current = (from.0, from.1);
                while Self::within_bounds(current.0, current.1, row_len as i32) {
                    antinodes.insert(current);
                    current = (current.0 - dx, current.1 - dy);
                }
                current = (to.0, to.1);
                while Self::within_bounds(current.0, current.1, row_len as i32) {
                    antinodes.insert(current);
                    current = (current.0 + dx, current.1 + dy);
                }
            }
        }
        antinodes.iter().for_each(|(x, y)| {
            Self::set_antinode(&mut matrix, *x as usize, *y as usize);
        });
        Grid { matrix }
    }

    pub fn count_antinodes(&self) -> usize {
        let mut count = 0;
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() {
                match &self.matrix[row][col] {
                    Cell::Antinode => {
                        count += 1;
                    }
                    Cell::Multiple(values) => {
                        values.iter().for_each(|c| {
                            if let Cell::Antinode = c {
                                count += 1;
                            }
                        });
                    }
                    _ => {}
                }
            }
        }
        count
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {
            for cell in row {
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Frequency(c) => *c,
                    Cell::Antinode => '#',
                    Cell::Multiple(_) => 'M',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matrix = Vec::new();
        s.lines().for_each(|line| {
            let mut row = Vec::new();
            line.chars().for_each(|c| {
                let cell = match c {
                    '.' => Cell::Empty,
                    'A'..='Z' => Cell::Frequency(c),
                    'a'..='z' => Cell::Frequency(c),
                    '0'..='9' => Cell::Frequency(c),
                    _ => todo!(),
                };
                row.push(cell);
            });
            matrix.push(row);
        });
        Ok(Grid { matrix })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let grid: Grid = input.parse().unwrap();
        print!("{}", grid);
        println!("");
        let marked = grid.part_one();
        print!("{}", marked);
        assert_eq!(marked.count_antinodes(), 14);
        let pt2 = grid.part_two();
        println!("");
        print!("{}", pt2);
        assert_eq!(pt2.count_antinodes(), 34);
    }
}
