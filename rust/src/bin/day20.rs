use std::collections::{HashMap, HashSet};

use aoc24::aoc;
use maplit::hashset;

struct Input {
    start: (i32, i32),
    end: (i32, i32),
    walls: HashSet<(i32, i32)>,
    limit_x: i32,
    limit_y: i32,
}

fn parse(input: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = HashSet::default();
    let mut limit_x = 0;
    let mut limit_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    walls.insert((x as i32, y as i32));
                }
                'S' => {
                    start = (x as i32, y as i32);
                }
                'E' => {
                    end = (x as i32, y as i32);
                }
                _ => panic!("Unexpected char {c}"),
            }
            limit_x = limit_x.max(x as i32);
        }
        limit_y = y as i32;
    }

    Input {
        start,
        end,
        walls,
        limit_x,
        limit_y,
    }
}

fn part_one(input: &str) -> usize {
    let input = parse(input);
    let cheats = all_cheats(input);
    cheats
        .into_iter()
        .filter_map(|(time_saved, how_much)| (time_saved >= 100).then_some(how_much))
        .sum()
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn shortest_path_cheating_at(input: &Input, cheat_at: HashSet<(i32, i32)>) -> usize {
    let Input {
        start,
        end,
        walls,
        limit_x,
        limit_y,
    } = input;

    let successors = |node: &(i32, i32)| {
        NEIGHBORS
            .iter()
            .filter_map(|delta| {
                let pos = (node.0 + delta.0, node.1 + delta.1);
                if pos.0 < 0
                    || pos.1 < 0
                    || pos.0 >= *limit_x
                    || pos.1 >= *limit_y
                    || (walls.contains(&pos) && !cheat_at.contains(&pos))
                {
                    return None;
                }
                Some((pos, 1))
            })
            .collect::<Vec<_>>()
    };
    let heuristic = |node: &(i32, i32)| {
        let dx = node.0 as f32 - end.0 as f32;
        let dy = node.1 as f32 - end.1 as f32;
        (dx * dx + dy * dy).sqrt() as usize
    };
    let success = |node: &(i32, i32)| node == end;

    let (_, cost) = pathfinding::directed::astar::astar(start, successors, heuristic, success)
        .expect("Failed to find solution");

    cost
}

// picoseconds saved -> nb of cheats
fn all_cheats(input: Input) -> HashMap<usize, usize> {
    let no_cheating = shortest_path_cheating_at(&input, HashSet::default());
    let mut cheats: HashMap<usize, usize> = HashMap::default();
    for wall in &input.walls {
        let best_path = shortest_path_cheating_at(&input, hashset![*wall]);
        if best_path >= no_cheating {
            continue;
        }
        let saved = no_cheating - best_path;
        let entry = cheats.entry(saved).or_default();
        *entry += 1;
    }

    cheats
}

aoc!(part_one);

#[cfg(test)]
pub mod tests {
    use maplit::hashmap;

    use super::*;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn day20() {
        let input = parse(INPUT);
        assert_eq!(
            all_cheats(input),
            hashmap! {
                2 => 14,
                4 => 14,
                6 => 2,
                8 => 4,
                10 => 2,
                12 => 3,
                20 => 1,
                36 => 1,
                38 => 1,
                40 => 1,
                64 => 1,
            }
        )
    }
}
