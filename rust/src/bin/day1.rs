use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), l| {
            let mut numbers = l
                .split("   ")
                .map(|x| x.parse::<i32>().expect("Failed to parse digits"));
            left.push(numbers.next().expect("Should have 2 numbers"));
            right.push(numbers.next().expect("Should have 2 numbers"));
            (left, right)
        })
}

fn part_one(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| i32::abs(a - b))
        .sum()
}

fn part_two(input: &str) -> i32 {
    let (left, right) = parse(input);

    let mut count: HashMap<i32, i32> = HashMap::default();
    for x in right {
        let entry = count.entry(x).or_default();
        *entry += 1;
    }

    left.into_iter()
        .map(|x| x * count.get(&x).copied().unwrap_or(0))
        .sum()
}

aoc24::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn day1() {
        assert_eq!(super::part_one(TEST_INPUT), 11);
        assert_eq!(super::part_two(TEST_INPUT), 31);
    }
}
