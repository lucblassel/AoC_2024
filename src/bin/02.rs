use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/02.txt");

fn main() -> Result<()> {
    println!("Day 02:");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2_brute(INPUT)?);

    Ok(())
}

// Recursive testing to see if the list is OK
fn is_ok(previous: i32, rest: &[i32], ascending: bool, allow_error: bool, has_error: bool) -> bool {
    //eprintln!("\tprev:{previous} rest:{rest:?}");
    if rest.is_empty() {
        return true;
    }
    let current = rest[0];

    let order_ok = if ascending {
        previous <= current
    } else {
        previous >= current
    };

    let jump_ok = (1..=3).contains(&(current - previous).abs());

    if !order_ok || !jump_ok {
        if allow_error {
            if has_error {
                // We already used up our error
                false
            } else {
                // We skip this level
                is_ok(previous, &rest[1..rest.len()], ascending, allow_error, true)
                    || is_ok(current, &rest[1..rest.len()], ascending, allow_error, true)
            }
        } else {
            // We are strict...
            false
        }
    } else {
        // We check the rest of the levels
        is_ok(
            current,
            &rest[1..rest.len()],
            ascending,
            allow_error,
            has_error,
        )
    }
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect_vec();

            let asc = is_ok(levels[0], &levels[1..levels.len()], true, false, false);
            let dsc = is_ok(levels[0], &levels[1..levels.len()], false, false, false);

            asc || dsc
        })
        .filter(|&is_safe| is_safe)
        .count())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect_vec();

            let asc = is_ok(levels[0], &levels[1..levels.len()], true, true, false);
            let asc2 = is_ok(levels[0], &levels[2..levels.len()], true, true, true);
            if asc && asc2 {
                (levels, true)
            } else {
                let dsc = is_ok(levels[0], &levels[1..levels.len()], false, true, false);
                let dsc2 = is_ok(levels[0], &levels[2..levels.len()], false, true, true);
                (levels, dsc && dsc2)
            }
        })
        .filter(|(_, is_safe)| *is_safe)
        .count())
}

fn part_2_brute(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect_vec();

            let mut f2 = false;
            for skip in 0..levels.len() {
                let skipped = [&levels[0..skip], &levels[(skip + 1)..levels.len()]].concat();
                let asc = is_ok(skipped[0], &skipped[1..skipped.len()], true, false, false);
                let dsc = is_ok(skipped[0], &skipped[1..skipped.len()], false, false, false);

                if asc || dsc {
                    f2 = true;
                    break;
                }
            }

            (levels, f2)
        })
        .filter(|(_, is_safe)| *is_safe)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const PROBLEMS: &str = r"40 41 42 43 44 48 51 52
15 16 23 25 26 28 29
53 52 51 52 51 49 47 44
26 24 22 21 17 14 13
23 21 18 15 13 8 6 5
89 91 93 96 93 94 96 99
62 63 67 70 71 74 76 77
20 23 25 32 34
74 73 71 73 71
83 80 76 74 72
49 47 45 40 38 35
23 24 22 23 24
60 61 63 67 69
17 20 22 24 26 32 34 35
65 63 66 65 63 61 59 58
75 74 73 69 67 66 63
76 75 73 71 65 62
59 56 52 49 47";
    #[test]
    fn test1() {
        assert_eq!(part_1(TEST).unwrap(), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(part_2_brute(TEST).unwrap(), 4)
    }

    #[test]
    fn problems() {
        assert_eq!(part_2(PROBLEMS).unwrap(), 0);
    }
    #[test]
    fn answer1() {
        assert_eq!(part_1(INPUT).unwrap(), 218)
    }

    #[test]
    fn answer2() {
        assert_eq!(part_2_brute(INPUT).unwrap(), 290)
    }
}
