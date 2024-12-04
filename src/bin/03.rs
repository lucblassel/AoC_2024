use anyhow::Result;
use regex::Regex;

const INPUT: &str = include_str!("../../inputs/03.txt");

fn main() -> Result<()> {
    println!("Day 03:");
    println!("\t1: {}", part1(INPUT)?);
    println!("\t2: {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    Ok(re.captures_iter(input).fold(0, |acc, cap| {
        let (_, [n1, n2]) = cap.extract();
        // We can unwrap because we are sure to get digits
        let n1 = n1.parse::<i32>().unwrap();
        let n2 = n2.parse::<i32>().unwrap();

        acc + n1 * n2
    }))
}

fn part2(input: &str) -> Result<i32> {
    //eprintln!("TESTING p2 on {input}");
    let re = Regex::new(r"((do|don't)\(\)|mul\((\d+),(\d+)\))")?;
    let (acc, _) = re.captures_iter(input).fold((0, true), |(acc, act), cap| {
        match (cap.get(2), cap.get(3), cap.get(4)) {
            (Some(modifier), None, None) => match modifier.as_str() {
                "do" => (acc, true),
                "don't" => (acc, false),
                m => unreachable!("invalid modifier: {m:?}"),
            },
            (None, Some(lhs), Some(rhs)) => {
                if act {
                    let n1 = lhs.as_str().parse::<i32>().unwrap();
                    let n2 = rhs.as_str().parse::<i32>().unwrap();
                    (acc + n1 * n2, act)
                } else {
                    (acc, act)
                }
            }
            _ => unreachable!(),
        }
    });
    Ok(acc)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST1).unwrap(), 161)
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST2).unwrap(), 48)
    }

    #[test]
    fn input1() {
        assert_eq!(part1(INPUT).unwrap(), 159892596);
    }

    #[test]
    fn input2() {
        assert_eq!(part2(INPUT).unwrap(), 92626942);
    }
}
