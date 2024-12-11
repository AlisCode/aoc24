use aoc24::aoc;
use fxhash::FxHashMap;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(" ")
        .map(|number| number.parse::<i64>().expect("to parse numbers"))
        .collect()
}

fn even_number_of_digits(i: i64) -> bool {
    i.ilog10() % 2 == 1
}

fn split_tuple(i: i64) -> (i64, i64) {
    let log = i.ilog10();
    let tenx = if log == 1 { 10 } else { 10i64.pow(log / 2 + 1) };
    let a = i / tenx;
    let b = i - a * tenx;
    (a, b)
}

fn number_of_stones_next(memo: &mut FxHashMap<(i64, i64), usize>, v: i64, max: i64) -> usize {
    if max == 0 {
        return 1;
    }

    if let Some(nb) = memo.get(&(v, max)) {
        return *nb;
    }

    let result = match v {
        0 => number_of_stones_next(memo, 1, max - 1),
        x if even_number_of_digits(x) => {
            let (a, b) = split_tuple(x);
            let a = number_of_stones_next(memo, a, max - 1);
            let b = number_of_stones_next(memo, b, max - 1);
            a + b
        }
        x => number_of_stones_next(memo, x * 2024, max - 1),
    };
    memo.insert((v, max), result);
    result
}

fn part_one(input: &str) -> usize {
    let state = parse(input);
    let mut memo = FxHashMap::default();
    state
        .into_iter()
        .map(|x| number_of_stones_next(&mut memo, x, 25))
        .sum()
}

fn part_two(input: &str) -> usize {
    let state = parse(input);
    let mut memo = FxHashMap::default();
    state
        .into_iter()
        .map(|x| number_of_stones_next(&mut memo, x, 75))
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_split() {
        assert_eq!(split_tuple(10), (1, 0));
        assert_eq!(split_tuple(1234), (12, 34));
        assert_eq!(split_tuple(123456), (123, 456));
    }

    #[test]
    fn day11() {
        assert_eq!(part_one(INPUT), 55312);
    }
}
