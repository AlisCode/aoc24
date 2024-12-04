use std::collections::HashMap;

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

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Need to redo this, but I can't think of anything smarter rn
fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let get = |x: i32, y: i32| grid.get(&(x, y));
    let get_words_starting_from_pos = |x: i32, y: i32| {
        DIRECTIONS.iter().map(move |&(dx, dy)| {
            (0..4)
                .filter_map(move |d| get(x + dx * d, y + dy * d))
                .collect::<String>()
        })
    };
    grid.iter()
        .filter_map(|((x, y), c)| {
            if *c == 'X' {
                Some(get_words_starting_from_pos(*x, *y))
            } else {
                None
            }
        })
        .flatten()
        .filter(|word| word == "XMAS")
        .count()
}

const DIRECTIONS_PART_TWO: [(i32, i32); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

// Need to redo this, but I can't think of anything smarter rn
fn part_two(input: &str) -> usize {
    let grid = parse(input);
    let get = |x: i32, y: i32| grid.get(&(x, y));
    let get_words_starting_from_pos = |x: i32, y: i32| -> Vec<String> {
        DIRECTIONS_PART_TWO
            .iter()
            .map(move |&(dx, dy)| {
                (0..3)
                    .filter_map(move |d| get(x + dx - dx * d, y + dy - dy * d))
                    .collect::<String>()
            })
            .collect()
    };

    grid.iter()
        .filter(|((x, y), c)| {
            if **c == 'A' {
                let words = get_words_starting_from_pos(*x, *y);
                words.iter().filter(|&w| w == "MAS").count() == 2
            } else {
                false
            }
        })
        .count()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn day4() {
        assert_eq!(part_one(INPUT), 18);
        assert_eq!(part_two(INPUT), 9);
    }
}
