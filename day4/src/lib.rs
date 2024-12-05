pub fn parse(s: &str) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    s.lines().for_each(|line| {
        let mut vec = Vec::new();
        line.chars().for_each(|c| {
            vec.push(c);
        });
        out.push(vec);
    });
    out
}

pub fn count(v: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let rows = v.len();
    let cols = v[0].len();
    for (ridx, row) in v.iter().enumerate() {
        for (cidx, c) in row.iter().enumerate() {
            if *c == 'X' {
                if cidx + 3 < cols
                    && row[cidx + 1] == 'M'
                    && row[cidx + 2] == 'A'
                    && row[cidx + 3] == 'S'
                {
                    count += 1;
                }
                if (cidx as i32 - 3) >= 0
                    && row[cidx - 1] == 'M'
                    && row[cidx - 2] == 'A'
                    && row[cidx - 3] == 'S'
                {
                    count += 1;
                }
                if ridx + 3 < rows
                    && v[ridx + 1][cidx] == 'M'
                    && v[ridx + 2][cidx] == 'A'
                    && v[ridx + 3][cidx] == 'S'
                {
                    count += 1;
                }
                if (ridx as i32 - 3) >= 0
                    && v[ridx - 1][cidx] == 'M'
                    && v[ridx - 2][cidx] == 'A'
                    && v[ridx - 3][cidx] == 'S'
                {
                    count += 1;
                }
                if ridx + 3 < rows
                    && cidx + 3 < cols
                    && v[ridx + 1][cidx + 1] == 'M'
                    && v[ridx + 2][cidx + 2] == 'A'
                    && v[ridx + 3][cidx + 3] == 'S'
                {
                    count += 1;
                }
                if ridx + 3 < rows
                    && (cidx as i32 - 3) >= 0
                    && v[ridx + 1][cidx - 1] == 'M'
                    && v[ridx + 2][cidx - 2] == 'A'
                    && v[ridx + 3][cidx - 3] == 'S'
                {
                    count += 1;
                }
                if (ridx as i32 - 3) >= 0
                    && (cidx as i32 - 3) >= 0
                    && v[ridx - 1][cidx - 1] == 'M'
                    && v[ridx - 2][cidx - 2] == 'A'
                    && v[ridx - 3][cidx - 3] == 'S'
                {
                    count += 1;
                }
                if (ridx as i32 - 3) >= 0
                    && cidx + 3 < cols
                    && v[ridx - 1][cidx + 1] == 'M'
                    && v[ridx - 2][cidx + 2] == 'A'
                    && v[ridx - 3][cidx + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn count_mas(v: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let rows = v.len();
    let cols = v[0].len();
    for (ridx, row) in v.iter().enumerate() {
        if ridx == 0 || ridx == rows - 1 {
            continue;
        }
        for (cidx, c) in row.iter().enumerate() {
            if cidx == 0 || cidx == cols - 1 {
                continue;
            }
            if *c == 'A' {
                let diagonal_1 = (v[ridx - 1][cidx - 1] == 'M' && v[ridx + 1][cidx + 1] == 'S')
                    || (v[ridx - 1][cidx - 1] == 'S' && v[ridx + 1][cidx + 1] == 'M');
                let diagonal_2 = (v[ridx - 1][cidx + 1] == 'M' && v[ridx + 1][cidx - 1] == 'S')
                    || (v[ridx - 1][cidx + 1] == 'S' && v[ridx + 1][cidx - 1] == 'M');
                if diagonal_1 && diagonal_2 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let v = parse(&input);
        let result = count(&v);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_mas() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let v = parse(&input);
        let result = count_mas(&v);
        assert_eq!(result, 9);
    }
}
