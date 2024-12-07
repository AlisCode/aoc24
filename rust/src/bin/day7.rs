use aoc24::aoc;
use nom::{multi::separated_list1, IResult};

struct Equation {
    result: i64,
    parts: Vec<i64>,
}

fn concat(a: i64, b: i64) -> i64 {
    let tenx = 10i64.pow(b.ilog10() + 1);
    a * tenx + b
}

fn is_solvable(result: i64, curr: i64, parts: &[i64], can_concat: bool) -> bool {
    match parts.len() {
        0 => unreachable!(),
        1 => {
            curr + parts[0] == result
                || curr * parts[0] == result
                || can_concat && (concat(curr, parts[0]) == result)
        }
        _ => {
            let value = &parts[0];
            let next = &parts[1..];

            let added = curr + value;
            let multiplied = curr * value;
            let concatted = concat(curr, *value);

            (added <= result && is_solvable(result, added, next, can_concat))
                || (multiplied <= result && is_solvable(result, multiplied, next, can_concat))
                || (can_concat
                    && concatted.ilog10() <= result.ilog10()
                    && is_solvable(result, concatted, next, can_concat))
        }
    }
}

impl Equation {
    pub fn is_solvable(&self, can_concat: bool) -> bool {
        let Equation { result, parts } = self;
        is_solvable(*result, parts[0], &parts[1..], can_concat)
    }
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, result) = nom::character::complete::i64(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;
    let (input, parts) = separated_list1(
        nom::bytes::complete::tag(" "),
        nom::character::complete::i64,
    )(input)?;
    Ok((input, Equation { result, parts }))
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| parse_equation(line).expect("Failed to parse equation").1)
        .collect()
}

fn part_one(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter_map(|eq| eq.is_solvable(false).then_some(eq.result))
        .sum()
}

fn part_two(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter_map(|eq| eq.is_solvable(true).then_some(eq.result))
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn day7() {
        assert_eq!(part_one(INPUT), 3749);
        assert_eq!(part_two(INPUT), 11387);
    }

    #[test]
    fn should_concat() {
        assert_eq!(concat(12, 123), 12123);
    }
}
