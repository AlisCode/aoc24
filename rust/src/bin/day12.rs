use std::collections::{HashMap, HashSet};

use aoc24::aoc;

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

type Regions = Vec<HashSet<(i32, i32)>>;
fn parse_regions(grid: HashMap<(i32, i32), char>) -> Regions {
    let mut close: HashSet<(i32, i32)> = HashSet::default();
    let mut regions = Vec::with_capacity(100);

    for ((x, y), c) in &grid {
        if close.contains(&(*x, *y)) {
            continue;
        }
        let mut region = HashSet::new();
        populate_region(&mut region, &grid, &mut close, *x, *y, *c);
        regions.push(region);
    }

    regions
}

fn populate_region(
    region: &mut HashSet<(i32, i32)>,
    grid: &HashMap<(i32, i32), char>,
    closed: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    c: char,
) {
    closed.insert((x, y));
    region.insert((x, y));
    for (nx, ny) in NEIGHBORS {
        let n = (x + nx, y + ny);
        if closed.contains(&n) {
            continue;
        }
        let Some(value) = grid.get(&n) else { continue };
        if c == *value {
            populate_region(region, grid, closed, n.0, n.1, c);
        }
    }
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Bottom,
    Direction::Left,
    Direction::Top,
];

fn compute_perimeter(region: &HashSet<(i32, i32)>) -> usize {
    let mut perimeter = 0;
    for (x, y) in region {
        for (nx, ny) in NEIGHBORS {
            let n = (x + nx, y + ny);
            if !region.contains(&n) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

fn compute_perimeter_discounted(region: &HashSet<(i32, i32)>) -> usize {
    let mut perimeter = HashSet::<((i32, i32), Direction)>::default();
    for (x, y) in region {
        for ((nx, ny), dir) in NEIGHBORS.iter().zip(DIRECTIONS) {
            let n = (x + nx, y + ny);
            if !region.contains(&n) {
                perimeter.insert((n, dir));
            }
        }
    }

    let mut close: HashSet<((i32, i32), Direction)> = HashSet::with_capacity(perimeter.len());
    let mut sides = 0;
    for (pos, dir) in &perimeter {
        sides += populate_side_in_closed_list(&mut close, &perimeter, pos.0, pos.1, *dir);
    }
    sides
}

fn populate_side_in_closed_list(
    close: &mut HashSet<((i32, i32), Direction)>,
    perim: &HashSet<((i32, i32), Direction)>,
    x: i32,
    y: i32,
    dir: Direction,
) -> usize {
    if close.contains(&((x, y), dir)) {
        return 0;
    }
    close.insert(((x, y), dir));
    for (nx, ny) in NEIGHBORS {
        let n = (x + nx, y + ny);
        if perim.contains(&(n, dir)) {
            populate_side_in_closed_list(close, perim, n.0, n.1, dir);
        }
    }
    1
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let regions = parse_regions(grid);
    regions
        .into_iter()
        .map(|region| {
            let area = region.len();
            let perimeter = compute_perimeter(&region);
            area * perimeter
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let grid = parse(input);
    let regions = parse_regions(grid);
    regions
        .into_iter()
        .map(|region| {
            let area = region.len();
            let perimeter = compute_perimeter_discounted(&region);
            area * perimeter
        })
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const SIMPLE_INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn day12() {
        assert_eq!(part_one(INPUT), 1930);
        assert_eq!(part_two(INPUT), 1206);
    }

    #[test]
    fn day12_parse_regions() {
        let grid = parse(SIMPLE_INPUT);
        let regions = parse_regions(grid);
        let a = HashSet::<(i32, i32)>::from_iter([(0, 0), (1, 0), (2, 0), (3, 0)]);
        let b = HashSet::<(i32, i32)>::from_iter([(0, 1), (1, 1), (0, 2), (1, 2)]);
        let c = HashSet::<(i32, i32)>::from_iter([(2, 1), (2, 2), (3, 2), (3, 3)]);
        let d = HashSet::<(i32, i32)>::from_iter([(3, 1)]);
        let e = HashSet::<(i32, i32)>::from_iter([(0, 3), (1, 3), (2, 3)]);
        assert!(regions.contains(&a));
        assert!(regions.contains(&b));
        assert!(regions.contains(&c));
        assert!(regions.contains(&d));
        assert!(regions.contains(&e));
    }

    #[test]
    fn day12_perimeters() {
        let a = HashSet::<(i32, i32)>::from_iter([(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(compute_perimeter(&a), 10);

        let b = HashSet::<(i32, i32)>::from_iter([(0, 1), (1, 1), (0, 2), (1, 2)]);
        assert_eq!(compute_perimeter(&b), 8);

        let c = HashSet::<(i32, i32)>::from_iter([(2, 1), (2, 2), (3, 2), (3, 3)]);
        assert_eq!(compute_perimeter(&c), 10);

        let d = HashSet::<(i32, i32)>::from_iter([(3, 1)]);
        assert_eq!(compute_perimeter(&d), 4);

        let e = HashSet::<(i32, i32)>::from_iter([(0, 3), (1, 3), (2, 3)]);
        assert_eq!(compute_perimeter(&e), 8);
    }
}
