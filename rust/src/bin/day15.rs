use std::collections::{HashMap, HashSet};

use aoc24::aoc;
use maplit::hashset;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Input {
    grid: HashMap<(i32, i32), Tile>,
    moves: Vec<Direction>,
    start: (i32, i32),
}

fn parse(input: &str, part_two: bool) -> Input {
    let mut parts = input.split("\n\n");
    let input_grid = parts.next().expect("2 parts");
    let mut start = (0, 0);
    let mut grid: HashMap<(i32, i32), Tile> = HashMap::default();

    for (y, line) in input_grid.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    if part_two {
                        grid.insert((x as i32 * 2, y as i32), Tile::Wall);
                        grid.insert((x as i32 * 2 + 1, y as i32), Tile::Wall);
                    } else {
                        grid.insert((x as i32, y as i32), Tile::Wall);
                    }
                }
                'O' => {
                    if part_two {
                        grid.insert((x as i32 * 2, y as i32), Tile::BoxLeft);
                        grid.insert((x as i32 * 2 + 1, y as i32), Tile::BoxRight);
                    } else {
                        grid.insert((x as i32, y as i32), Tile::Box);
                    }
                }
                '@' => {
                    if part_two {
                        start = (x as i32 * 2, y as i32);
                    } else {
                        start = (x as i32, y as i32);
                    }
                }
                _ => (),
            }
        }
    }

    let moves = parts.next().expect("2 parts");
    let moves = moves
        .trim()
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '^' => Some(Direction::Up),
            '\n' => None,
            _ => panic!("Unexpected char {}", c),
        })
        .collect();

    Input { grid, moves, start }
}

fn dir_delta(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    }
}

fn sum_of_coords(grid: &HashMap<(i32, i32), Tile>) -> i32 {
    grid.iter()
        .filter_map(|((x, y), tile)| match tile {
            Tile::Box | Tile::BoxLeft => Some(100 * y + x),
            Tile::Wall => None,
            Tile::BoxRight => None,
        })
        .sum()
}

enum MoveOutcome {
    Blocked,
    Allowed(Tracklist),
}

// Dirty solution for a queue that has unique position and iterates in the same order as stuff were
// added.
#[derive(Debug, Default)]
struct Tracklist {
    queue: Vec<(i32, i32)>,
    set: HashSet<(i32, i32)>,
}

impl Tracklist {
    pub fn add(&mut self, pos: (i32, i32)) {
        if self.set.contains(&pos) {
            return;
        }
        self.set.insert(pos);
        self.queue.push(pos);
    }
}

fn perform_move(grid: &mut HashMap<(i32, i32), Tile>, coord: (i32, i32), target: (i32, i32)) {
    if let Some(current_tile) = grid.get(&coord).copied() {
        grid.insert(target, current_tile);
    }
    grid.remove(&coord);
}

fn do_move(
    grid: &HashMap<(i32, i32), Tile>,
    coord: (i32, i32),
    dir: Direction,
    tracklist: &mut HashSet<(i32, i32)>,
) -> MoveOutcome {
    let delta = dir_delta(dir);
    let target = (coord.0 + delta.0, coord.1 + delta.1);
    tracklist.insert(target);

    match grid.get(&target).copied() {
        None => MoveOutcome::Allowed(Tracklist {
            queue: vec![coord],
            set: hashset![coord],
        }),
        Some(Tile::Wall) => MoveOutcome::Blocked,
        Some(Tile::Box) => match do_move(grid, target, dir, tracklist) {
            MoveOutcome::Blocked => MoveOutcome::Blocked,
            MoveOutcome::Allowed(mut tracklist) => {
                tracklist.add(coord);
                MoveOutcome::Allowed(tracklist)
            }
        },
        Some(x @ Tile::BoxRight) | Some(x @ Tile::BoxLeft) => {
            let other = match x {
                Tile::BoxLeft => Direction::Right,
                Tile::BoxRight => Direction::Left,
                _ => unreachable!(),
            };

            let delta = dir_delta(other);
            let other_target = (target.0 + delta.0, target.1 + delta.1);

            let self_move = do_move(grid, target, dir, tracklist);
            let other_move = if !tracklist.contains(&other_target) {
                do_move(grid, other_target, dir, tracklist)
            } else {
                MoveOutcome::Allowed(Tracklist::default())
            };
            match (self_move, other_move) {
                (MoveOutcome::Blocked, _) | (_, MoveOutcome::Blocked) => MoveOutcome::Blocked,
                (MoveOutcome::Allowed(mut tracklist), MoveOutcome::Allowed(q2)) => {
                    for q in q2.queue {
                        tracklist.add(q);
                    }
                    tracklist.add(coord);
                    MoveOutcome::Allowed(tracklist)
                }
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let Input {
        mut grid,
        moves,
        start,
    } = parse(input, false);
    let mut pos = start;

    for m in moves {
        match do_move(&grid, pos, m, &mut HashSet::default()) {
            MoveOutcome::Allowed(tracklist) => {
                // Move robot
                let delta = dir_delta(m);
                pos.0 += delta.0;
                pos.1 += delta.1;

                // Move all items in the tracklist
                for q in tracklist.queue {
                    let target = (q.0 + delta.0, q.1 + delta.1);
                    perform_move(&mut grid, q, target);
                }
            }
            MoveOutcome::Blocked => (),
        }
    }

    sum_of_coords(&grid)
}

fn part_two(input: &str) -> i32 {
    let Input {
        mut grid,
        moves,
        start,
    } = parse(input, true);
    let mut pos = start;

    for m in moves {
        match do_move(&grid, pos, m, &mut HashSet::default()) {
            MoveOutcome::Allowed(tracklist) => {
                // Move robot
                let delta = dir_delta(m);
                pos.0 += delta.0;
                pos.1 += delta.1;

                // Move all items in the tracklist
                for q in tracklist.queue {
                    let target = (q.0 + delta.0, q.1 + delta.1);
                    perform_move(&mut grid, q, target);
                }
            }
            MoveOutcome::Blocked => (),
        }
    }

    sum_of_coords(&grid)
}

aoc!(part_one, part_two);

// const INPUT: &str = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########
//
// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
//
// For fun and debugs
// fn _display(grid: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) -> String {
//     let mut string = String::new();
//     for y in 0..10 {
//         for x in 0..20 {
//             let xy = (x, y);
//             if pos == xy {
//                 string.push('@');
//                 continue;
//             }
//             match grid.get(&xy) {
//                 Some(Tile::Wall) => string.push('#'),
//                 Some(Tile::Box) => string.push('O'),
//                 Some(Tile::BoxLeft) => string.push('['),
//                 Some(Tile::BoxRight) => string.push(']'),
//                 None => string.push('.'),
//             }
//         }
//         string.push('\n');
//     }
//     string
// }
//
// // Enable to play the robot (HJKL)
// fn main() {
//     let Input {
//         mut grid,
//         moves,
//         start,
//     } = parse(INPUT, true);
//     let mut pos = start;
//     let mut moves = moves.into_iter();
//
//     loop {
//         println!("{}", display(&grid, pos));
//         let mut buffer = String::new();
//         let stdin = std::io::stdin();
//         let _ = stdin.read_line(&mut buffer);
//         let dir = match buffer.trim() {
//             "h" => Direction::Left,
//             "j" => Direction::Down,
//             "k" => Direction::Up,
//             "l" => Direction::Right,
//             "c" => moves.next().expect("Ran out of moves"),
//             _ => todo!(),
//         };
//         match do_move(&grid, pos, dir, &mut HashSet::default()) {
//             MoveOutcome::Allowed(tracklist) => {
//                 // Move robot
//                 let delta = dir_delta(dir);
//                 pos.0 += delta.0;
//                 pos.1 += delta.1;
//
//                 // Move all items in the tracklist
//                 for q in tracklist.queue {
//                     let target = (q.0 + delta.0, q.1 + delta.1);
//                     perform_move(&mut grid, q, target);
//                 }
//             }
//             MoveOutcome::Blocked => (),
//         }
//     }
// }

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT_TWO: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn day15() {
        assert_eq!(part_one(INPUT), 2028);
        assert_eq!(part_two(INPUT_TWO), 9021);
    }
}
