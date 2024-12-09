use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Visited,
    Unvisited,
    Obstruction,
    Guard(Direction),
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub matrix: Vec<Vec<Cell>>,
}

impl Grid {
    fn find_guard(&self) -> Option<(usize, usize, Direction)> {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let Cell::Guard(dir) = cell {
                    return Some((i, j, *dir));
                }
            }
        }
        None
    }

    fn traverse(&mut self) -> (bool, HashSet<(usize, usize)>) {
        let mut visited_cells = HashSet::new();
        let mut visited_guards = HashSet::new();
        while let Some((row, col, dir)) = self.find_guard() {
            if visited_guards.contains(&(row, col, dir)) {
                return (true, visited_cells);
            }
            visited_guards.insert((row, col, dir));
            visited_cells.insert((row, col));
            match dir {
                Direction::Up => {
                    self.matrix[row][col] = Cell::Visited;
                    for y in (0..row).rev() {
                        if let Cell::Obstruction = self.matrix[y][col] {
                            self.matrix[y + 1][col] = Cell::Guard(Direction::Right);
                            break;
                        }
                        visited_cells.insert((y, col));
                        self.matrix[y][col] = Cell::Visited;
                    }
                }
                Direction::Right => {
                    self.matrix[row][col] = Cell::Visited;
                    for x in col + 1..self.matrix[row].len() {
                        if let Cell::Obstruction = self.matrix[row][x] {
                            self.matrix[row][x - 1] = Cell::Guard(Direction::Down);
                            break;
                        }
                        visited_cells.insert((row, x));
                        self.matrix[row][x] = Cell::Visited;
                    }
                }
                Direction::Down => {
                    self.matrix[row][col] = Cell::Visited;
                    for y in row + 1..self.matrix.len() {
                        if let Cell::Obstruction = self.matrix[y][col] {
                            self.matrix[y - 1][col] = Cell::Guard(Direction::Left);
                            break;
                        }
                        visited_cells.insert((y, col));
                        self.matrix[y][col] = Cell::Visited;
                    }
                }
                Direction::Left => {
                    self.matrix[row][col] = Cell::Visited;
                    for x in (0..col).rev() {
                        if let Cell::Obstruction = self.matrix[row][x] {
                            self.matrix[row][x + 1] = Cell::Guard(Direction::Up);
                            break;
                        }
                        visited_cells.insert((row, x));
                        self.matrix[row][x] = Cell::Visited;
                    }
                }
            }
        }
        (false, visited_cells)
    }

    pub fn part_one(&mut self) -> i32 {
        let _ = self.traverse();
        self.matrix
            .iter()
            .flatten()
            .filter(|c| **c == Cell::Visited)
            .count() as i32
    }

    pub fn part_two(&mut self) -> i32 {
        let (_, visited) = self.clone().traverse();
        let mut count = 0;
        for (row, col) in visited {
            let mut copy = self.clone();
            copy.matrix[row][col] = Cell::Obstruction;
            let (is_loop, _) = copy.traverse();
            if is_loop {
                count += 1;
            }
        }
        count
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
                    '.' => Cell::Unvisited,
                    '#' => Cell::Obstruction,
                    '^' => Cell::Guard(Direction::Up),
                    '>' => Cell::Guard(Direction::Down),
                    '<' => Cell::Guard(Direction::Left),
                    'v' => Cell::Guard(Direction::Right),
                    'X' => Cell::Visited,
                    _ => todo!(),
                };
                row.push(cell);
            });
            matrix.push(row);
        });
        Ok(Grid { matrix })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {
            for cell in row {
                let c = match cell {
                    Cell::Visited => 'X',
                    Cell::Unvisited => '.',
                    Cell::Obstruction => '#',
                    Cell::Guard(Direction::Up) => '^',
                    Cell::Guard(Direction::Down) => 'v',
                    Cell::Guard(Direction::Left) => '<',
                    Cell::Guard(Direction::Right) => '>',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.matrix.len(), 10);
        assert_eq!(grid.clone().part_one(), 41);
        assert_eq!(grid.clone().part_two(), 6);
    }
}
