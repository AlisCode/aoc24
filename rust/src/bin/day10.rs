use std::collections::{HashMap, HashSet};

use aoc24::aoc;

struct Grid {
    elevations: HashMap<(i32, i32), i32>,
    potential_trailheads: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> Grid {
    let mut potential_trailheads: HashSet<(i32, i32)> = HashSet::default();
    let mut elevations: HashMap<(i32, i32), i32> = HashMap::default();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let elevation = (c as u8 - b'0') as i32;
            if elevation == 0 {
                potential_trailheads.insert((x as i32, y as i32));
            }
            elevations.insert((x as i32, y as i32), elevation);
        }
    }

    Grid {
        elevations,
        potential_trailheads,
    }
}

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn end_of_trails_reachable_from(
    elevations: &HashMap<(i32, i32), i32>,
    position: (i32, i32),
    starting_value: i32,
) -> HashSet<(i32, i32)> {
    let value = elevations.get(&position);
    match value {
        Some(9) if starting_value == 9 => {
            let mut pos = HashSet::default();
            pos.insert(position);
            pos
        }
        Some(x) if *x == starting_value => NEIGHBORS
            .iter()
            .flat_map(|delta| {
                let neighbor = (position.0 + delta.0, position.1 + delta.1);
                end_of_trails_reachable_from(elevations, neighbor, starting_value + 1)
            })
            .collect(),
        Some(_) | None => HashSet::default(),
    }
}

fn trails_reachable_from(
    elevations: &HashMap<(i32, i32), i32>,
    position: (i32, i32),
    starting_value: i32,
) -> usize {
    let value = elevations.get(&position);
    match value {
        Some(9) if starting_value == 9 => 1,
        Some(x) if *x == starting_value => NEIGHBORS
            .iter()
            .map(|delta| {
                let neighbor = (position.0 + delta.0, position.1 + delta.1);
                trails_reachable_from(elevations, neighbor, starting_value + 1)
            })
            .sum(),
        Some(_) | None => 0,
    }
}

fn part_one(input: &str) -> usize {
    let Grid {
        elevations,
        potential_trailheads,
    } = parse(input);

    potential_trailheads
        .iter()
        .map(|position| {
            let end_of_trails = end_of_trails_reachable_from(&elevations, *position, 0);
            end_of_trails.len()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let Grid {
        elevations,
        potential_trailheads,
    } = parse(input);

    potential_trailheads
        .iter()
        .map(|position| trails_reachable_from(&elevations, *position, 0))
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn day10() {
        assert_eq!(part_one(INPUT), 36);
        assert_eq!(part_two(INPUT), 81);
    }
}
