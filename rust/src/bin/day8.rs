use aoc24::aoc;
use fxhash::{FxHashMap, FxHashSet};

pub struct Map {
    antennas: FxHashMap<char, FxHashSet<(i32, i32)>>,
    limit: (i32, i32),
}

fn parse(input: &str) -> Map {
    let mut antennas: FxHashMap<char, FxHashSet<(i32, i32)>> = FxHashMap::default();
    let mut limit = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            limit.0 = limit.0.max(x as i32);
            match c {
                '.' => (),
                c => {
                    let group = antennas.entry(c).or_default();
                    group.insert((x as i32, y as i32));
                }
            }
        }
        limit.1 = y as i32;
    }
    Map { antennas, limit }
}

struct AntinodeIterator {
    k: i32,
    a: (i32, i32),
    b: (i32, i32),
    limit: (i32, i32),
}

impl AntinodeIterator {
    pub fn new(limit: (i32, i32), a: (i32, i32), b: (i32, i32)) -> Self {
        AntinodeIterator { limit, k: 0, a, b }
    }
}

impl Iterator for AntinodeIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let Self { k, a, b, limit } = self;
        let inbounds = |x: i32, y: i32| x >= 0 && x <= limit.0 && y >= 0 && y <= limit.1;

        let (x, y) = ((*k + 1) * a.0 - *k * b.0, (*k + 1) * a.1 - *k * b.1);
        self.k += 1;

        inbounds(x, y).then_some((x, y))
    }
}

fn part_one(input: &str) -> usize {
    let Map { antennas, limit } = parse(input);

    antennas
        .into_iter()
        .flat_map(|(_, positions)| {
            let mut antinodes = FxHashSet::<(i32, i32)>::default();
            for a in &positions {
                for b in &positions {
                    if a == b {
                        continue;
                    }
                    let antinodes_a_to_b = AntinodeIterator::new(limit, *a, *b).skip(1).take(1);
                    let antinodes_b_to_a = AntinodeIterator::new(limit, *b, *a).skip(1).take(1);
                    antinodes.extend(antinodes_a_to_b.chain(antinodes_b_to_a))
                }
            }
            antinodes
        })
        .collect::<FxHashSet<_>>()
        .len()
}

fn part_two(input: &str) -> usize {
    let Map { antennas, limit } = parse(input);

    antennas
        .into_iter()
        .flat_map(|(_, positions)| {
            let mut antinodes = FxHashSet::<(i32, i32)>::default();
            for a in &positions {
                for b in &positions {
                    if a == b {
                        continue;
                    }
                    let antinodes_a_to_b = AntinodeIterator::new(limit, *a, *b);
                    let antinodes_b_to_a = AntinodeIterator::new(limit, *b, *a);
                    antinodes.extend(antinodes_a_to_b.chain(antinodes_b_to_a))
                }
            }
            antinodes
        })
        .collect::<FxHashSet<_>>()
        .len()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn day8() {
        assert_eq!(part_one(INPUT), 14);
        assert_eq!(part_two(INPUT), 34);
    }
}
