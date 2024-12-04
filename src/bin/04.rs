use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/04.txt");

fn main() -> Result<()> {
    println!("Day 04:");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let grid = WordSearch::from_str(input)?;
    grid.find_xmas()
}

fn part_2(input: &str) -> Result<usize> {
    let grid = WordSearch::from_str(input)?;
    grid.find_mas_xs()
}

struct WordSearch {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

enum Direction {
    Left,
    Right,
}

impl WordSearch {
    fn find_mas_xs(&self) -> Result<usize> {
        self.grid
            .iter()
            .enumerate()
            .take(self.height - 1) // Skip last row
            .skip(1) // Skip 1st row
            .flat_map(move |(row, row_vals)| {
                row_vals
                    .iter()
                    .enumerate()
                    .take(self.width - 1) // Skip last col
                    .skip(1) // Skip 1st col
                    .map(move |(col, c)| {
                        if *c == 'A' {
                            // Center of cross
                            match (
                                self.check_diag(row - 1, col + 1, Direction::Left, "MAS"),
                                self.check_diag(row - 1, col - 1, Direction::Right, "MAS"),
                            ) {
                                (Ok(left), Ok(right)) => Ok(left && right),
                                (Err(el), _) => Err(el),
                                (_, Err(er)) => Err(er),
                            }
                        } else {
                            Ok(false)
                        }
                    })
            })
            .try_fold(0, |acc, res| res.map(|v| if v { acc + 1 } else { acc }))
    }

    fn find_xmas(&self) -> Result<usize> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(move |(row, row_vals)| {
                row_vals.iter().enumerate().flat_map(move |(col, c)| {
                    if *c == 'S' || *c == 'X' {
                        vec![
                            self.check_down(row, col),
                            self.check_right(row, col),
                            self.check_diag(row, col, Direction::Left, "XMAS"),
                            self.check_diag(row, col, Direction::Right, "XMAS"),
                        ]
                    } else {
                        vec![]
                    }
                })
            })
            .try_fold(0, |acc, res| res.map(|v| if v { acc + 1 } else { acc }))
    }

    fn check_index(&self, row: usize, col: usize) -> Result<()> {
        if !(row < self.height && col < self.width) {
            bail!("Cell is not in the grid")
        }

        Ok(())
    }

    fn check_right(&self, row: usize, col: usize) -> Result<bool> {
        self.check_index(row, col)?;
        if col > self.width - 4 {
            return Ok(false);
        }

        let w = (0..4).map(|d| self.grid[row][col + d]).join("");
        Ok(w == "XMAS" || w == "SAMX")
    }

    // checks if the 4 letters below
    fn check_down(&self, row: usize, col: usize) -> Result<bool> {
        self.check_index(row, col)?;
        if row > self.height - 4 {
            return Ok(false);
        }

        let w = (0..4).map(|d| self.grid[row + d][col]).join("");

        Ok(w == "XMAS" || w == "SAMX")
    }

    // Checks if the 2 diagonals starting from (row,col) and going down
    // contain the word or the reverse
    fn check_diag(&self, row: usize, col: usize, dir: Direction, word: &str) -> Result<bool> {
        self.check_index(row, col)?;

        let rev = word.chars().rev().join("");

        // If we are too low there cannot be any diagonals
        if row > self.height - word.len() {
            return Ok(false);
        }

        // Check that it is possible to make a diagonal
        match dir {
            Direction::Left => {
                if col < word.len() - 1 {
                    return Ok(false);
                }
            }
            Direction::Right => {
                if col > self.width - word.len() {
                    return Ok(false);
                }
            }
        }

        let diag = (0..word.len())
            .map(|d| match dir {
                Direction::Right => self.grid[row + d][col + d],
                Direction::Left => self.grid[row + d][col.saturating_sub(d)],
            })
            .join("");

        Ok(diag == word || diag == rev)
    }
}

impl FromStr for WordSearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let height = grid.len();
        let width = grid
            .iter()
            .fold(None, |acc, line| match acc {
                Some(prev_len) => (line.len() == prev_len).then_some(prev_len),
                None => Some(line.len()),
            })
            .context("All lines must have the same width")?;

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

impl Debug for WordSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad = self.height.ilog10() as usize + 1;
        let s = self
            .grid
            .iter()
            .enumerate()
            .map(|(idx, line)| format!("{idx: >pad$} {}", line.iter().join("")))
            .join("\n");

        write!(f, "Grid({} x {})\n{s}", self.height, self.width)
    }
}

impl Display for WordSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid.iter().map(|l| l.iter().join("")).join("\n")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_1: &str = r"..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const TEST_2: &str = r"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

    const TEST_3: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test1() {
        assert_eq!(part_1(TEST_1).unwrap(), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(part_1(TEST_2).unwrap(), 18)
    }

    #[test]
    fn test3() {
        assert_eq!(part_1(TEST_3).unwrap(), 18)
    }

    #[test]
    fn input1() {
        assert_eq!(part_1(INPUT).unwrap(), 2504)
    }

    #[test]
    fn test4() {
        assert_eq!(part_2(TEST_3).unwrap(), 9)
    }

    #[test]
    fn input2() {
        assert_eq!(part_2(INPUT).unwrap(), 1923)
    }
}
