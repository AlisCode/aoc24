use std::collections::HashSet;

use aoc24::aoc;

struct Map {
    obstacles: HashSet<(i32, i32)>,
    start_position: (i32, i32),
    limit: (i32, i32),
}

fn parse(input: &str) -> Map {
    let mut start_position: (i32, i32) = (0, 0);
    let mut obstacles = HashSet::default();
    let mut limit = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            limit.0 = limit.0.max(x as i32);
            match c {
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                '^' => {
                    start_position = (x as i32, y as i32);
                }
                '.' => (),
                c => panic!("Unsupported char {c}"),
            }
        }
        limit.1 = limit.1.max(y as i32);
    }
    Map {
        obstacles,
        start_position,
        limit,
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Top,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn turn_90deg(self) -> Direction {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }
}

fn part_one(input: &str) -> usize {
    let Map {
        obstacles,
        start_position,
        limit,
    } = parse(input);

    let mut visited: HashSet<(i32, i32)> = HashSet::default();
    let mut direction = Direction::Top;
    let mut position = start_position;

    loop {
        let (dx, dy) = direction.delta();
        position.0 += dx;
        position.1 += dy;
        visited.insert(position);

        if position.0 < 0 || position.0 >= limit.0 || position.1 < 0 || position.1 >= limit.1 {
            return visited.len();
        }

        let lookahead = (position.0 + dx, position.1 + dy);
        if obstacles.contains(&lookahead) {
            direction = direction.turn_90deg();
        }
    }
}

fn can_escape(
    mut visited: HashSet<((i32, i32), Direction)>,
    obstacles: &HashSet<(i32, i32)>,
    mut position: (i32, i32),
    mut direction: Direction,
    limit: (i32, i32),
) -> bool {
    loop {
        loop {
            if !visited.insert((position, direction)) {
                // Guard in a loop
                return false;
            }
            let (dx, dy) = direction.delta();
            let lookahead = (position.0 + dx, position.1 + dy);
            if !obstacles.contains(&lookahead) {
                break;
            }
            direction = direction.turn_90deg();
        }

        let (dx, dy) = direction.delta();
        position.0 += dx;
        position.1 += dy;

        if position.0 < 0 || position.0 > limit.0 || position.1 < 0 || position.1 > limit.1 {
            // Guard escaped
            return true;
        }
    }
}

fn part_two(input: &str) -> usize {
    let Map {
        mut obstacles,
        start_position,
        limit,
    } = parse(input);

    let mut path: HashSet<(i32, i32)> = HashSet::default();
    let mut direction = Direction::Top;
    let mut position = start_position;

    loop {
        let (dx, dy) = direction.delta();
        position.0 += dx;
        position.1 += dy;
        path.insert(position);

        if position.0 < 0 || position.0 >= limit.0 || position.1 < 0 || position.1 >= limit.1 {
            break;
        }

        let lookahead = (position.0 + dx, position.1 + dy);
        if obstacles.contains(&lookahead) {
            direction = direction.turn_90deg();
        }
    }

    let mut total = 0;
    for (x, y) in path {
        if !obstacles.insert((x, y)) {
            continue;
        }
        if !can_escape(
            HashSet::default(),
            &obstacles,
            start_position,
            Direction::Top,
            limit,
        ) {
            total += 1;
        }
        obstacles.remove(&(x, y));
    }
    total
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn day6() {
        assert_eq!(part_one(INPUT), 41);
        assert_eq!(part_two(INPUT), 6);
    }
}
