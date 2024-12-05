use std::collections::{HashMap, HashSet};

use aoc24::aoc;

#[derive(Debug)]
struct Input {
    dependencies: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

fn parse(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let rules: Vec<(i32, i32)> = sections
        .next()
        .expect("Expected 2 sections")
        .lines()
        .map(|l| {
            let mut pages = l
                .split("|")
                .map(|n| n.parse::<i32>().expect("Failed to parse page nb"));
            let a = pages.next().expect("Expected 2 numbers");
            let b = pages.next().expect("Expected 2 numbers");
            (a, b)
        })
        .collect();
    let updates: Vec<Vec<i32>> = sections
        .next()
        .expect("Expected 2 sections")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i32>().expect("Failed to parse page nb"))
                .collect()
        })
        .collect();

    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::default();
    for (before, after) in rules {
        let deps = dependencies.entry(after).or_default();
        deps.insert(before);
    }

    Input {
        dependencies,
        updates,
    }
}

fn middle(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

fn update_safe(dependencies: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> bool {
    let mut seen: HashSet<i32> = HashSet::default();
    let open: HashSet<i32> = update.iter().copied().collect();

    update.iter().all(|v| {
        seen.insert(*v);
        match dependencies.get(v) {
            None => true,
            Some(deps) => {
                let wanted_deps: HashSet<i32> = open.intersection(deps).copied().collect();
                (&wanted_deps - &seen).is_empty()
            }
        }
    })
}

fn fixed_update(dependencies: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> Vec<i32> {
    let open: HashSet<i32> = update.iter().copied().collect();

    let deps: HashMap<i32, HashSet<i32>> = open
        .iter()
        .map(|x| match dependencies.get(x) {
            None => (*x, HashSet::<i32>::default()),
            Some(deps) => (*x, open.intersection(deps).copied().collect()),
        })
        .collect();

    let mut close: HashSet<i32> = HashSet::default();
    let mut fixed = Vec::new();
    while fixed.len() != update.len() {
        for x in &open - &close {
            let deps = &deps[&x];
            if (deps - &close).is_empty() {
                fixed.push(x);
                close.insert(x);
            }
        }
    }

    fixed
}

fn part_one(input: &str) -> i32 {
    let Input {
        dependencies,
        updates,
    } = parse(input);
    updates
        .iter()
        .filter_map(|update| update_safe(&dependencies, update).then_some(middle(update)))
        .sum()
}

fn part_two(input: &str) -> i32 {
    let Input {
        dependencies,
        updates,
    } = parse(input);
    updates
        .iter()
        .filter_map(|update| {
            if update_safe(&dependencies, update) {
                return None;
            }
            let fixed = fixed_update(&dependencies, update);
            Some(middle(&fixed))
        })
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn day5_part_one() {
        let Input {
            dependencies,
            updates,
        } = parse(INPUT);
        assert!(update_safe(&dependencies, &updates[0]));
    }

    #[test]
    fn day5() {
        assert_eq!(part_one(INPUT), 143);
        assert_eq!(part_two(INPUT), 123);
    }
}
