use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/01.txt");

fn main() -> Result<()> {
    println!("Day 01:");
    let r1 = part_1(INPUT)?;
    println!("\t1: {r1}");
    let r2 = part_2(INPUT)?;
    println!("\t2: {r2}");

    Ok(())
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l: &str| {
            let (d1, d2) = l.split_once("   ").unwrap();
            (d1.parse::<i32>().unwrap(), d2.parse::<i32>().unwrap())
        })
        .multiunzip()
}

fn part_1(input: &str) -> Result<i32> {
    let (l1, l2) = parse_lists(input);
    Ok(l1
        .iter()
        .sorted()
        .zip(l2.iter().sorted())
        .fold(0i32, |a, (d1, d2)| a + (d1 - d2).abs()))
}

fn part_2(input: &str) -> Result<i32> {
    let (l1, l2) = parse_lists(input);
    let mut counts = HashMap::new();
    for c in l2 {
        *(counts.entry(c).or_insert(0)) += 1;
    }

    Ok(l1
        .into_iter()
        .map(|v| v * counts.get(&v).unwrap_or(&0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test1() {
        assert_eq!(part_1(TEST).unwrap(), 11)
    }

    #[test]
    fn answer1() {
        assert_eq!(part_1(INPUT).unwrap(), 2066446)
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(TEST).unwrap(), 31)
    }

    #[test]
    fn answer2() {
        assert_eq!(part_2(INPUT).unwrap(), 24931009)
    }
}
