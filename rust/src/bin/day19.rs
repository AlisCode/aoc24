use std::collections::HashMap;

use aoc24::aoc;

struct Input<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

fn parse(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let patterns = input.next().expect("two elements").split(", ").collect();
    let designs = input.next().expect("two elements").lines().collect();

    Input { patterns, designs }
}

fn solve_design(memo: &mut HashMap<String, usize>, design: &str, patterns: &[&str]) -> usize {
    if let Some(memoed) = memo.get(design) {
        return *memoed;
    }

    let solved = patterns
        .iter()
        .map(|p| {
            if !design.starts_with(p) {
                return 0;
            }
            if design.len() == p.len() {
                1
            } else {
                solve_design(memo, &design[p.len()..], patterns)
            }
        })
        .sum();
    memo.insert(design.to_string(), solved);
    solved
}

fn part_one(input: &str) -> usize {
    let Input { patterns, designs } = parse(input);

    let mut memo = HashMap::default();
    designs
        .into_iter()
        .filter(|design| solve_design(&mut memo, design, &patterns) > 0)
        .count()
}

fn part_two(input: &str) -> usize {
    let Input { patterns, designs } = parse(input);

    let mut memo = HashMap::default();
    designs
        .into_iter()
        .map(|design| solve_design(&mut memo, design, &patterns))
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn day19() {
        assert_eq!(part_one(INPUT), 6);
        assert_eq!(part_two(INPUT), 16);
    }
}
