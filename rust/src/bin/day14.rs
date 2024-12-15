use std::cmp::Ordering;

use aoc24::aoc;
use nom::{bytes::complete::tag, IResult};

#[derive(Debug, PartialEq, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    pub fn simulate(&mut self, grid_width: i32, grid_height: i32, seconds: i32) {
        let Robot { pos, vel } = self;
        let moved_px = i32::wrapping_rem_euclid(pos.0 + vel.0 * seconds, grid_width);
        let moved_py = i32::wrapping_rem_euclid(pos.1 + vel.1 * seconds, grid_height);
        self.pos = (moved_px, moved_py);
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, px) = nom::character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, py) = nom::character::complete::i32(input)?;

    let (input, _) = tag(" v=")(input)?;
    let (input, vx) = nom::character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, vy) = nom::character::complete::i32(input)?;

    Ok((
        input,
        Robot {
            pos: (px, py),
            vel: (vx, vy),
        },
    ))
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (_, robot) = parse_robot(line).expect("Failed to parse robot");
            robot
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    do_part_one(input, 101, 103)
}

fn do_part_one(input: &str, grid_width: i32, grid_height: i32) -> usize {
    let mut robots = parse(input);

    let mut quads = vec![0, 0, 0, 0];
    for robot in &mut robots {
        robot.simulate(grid_width, grid_height, 100);
        let half_x = grid_width / 2;
        let half_y = grid_height / 2;
        match (robot.pos.0.cmp(&half_x), robot.pos.1.cmp(&half_y)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => (),
            (Ordering::Less, Ordering::Less) => quads[0] += 1,
            (Ordering::Less, Ordering::Greater) => quads[1] += 1,
            (Ordering::Greater, Ordering::Less) => quads[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quads[3] += 1,
        }
    }
    quads.into_iter().product()
}

aoc!(part_one);
// Part two I solved by finding a line of robots containing > 35 robots,
// and saying "clearly this is the right step".
//
// I solved this manually and dont see a clear way to automate that.
// Whoever you are, reading this implementation, you're on your own buddy.

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn day14() {
        assert_eq!(do_part_one(INPUT, 11, 7), 12);
    }

    #[test]
    fn day14_simulate() {
        let mut robot = Robot {
            pos: (2, 4),
            vel: (2, -3),
        };

        robot.simulate(11, 7, 1);
        assert_eq!(
            robot,
            Robot {
                pos: (4, 1),
                vel: (2, -3),
            }
        );

        robot.simulate(11, 7, 1);
        assert_eq!(
            robot,
            Robot {
                pos: (6, 5),
                vel: (2, -3),
            }
        );
    }
}
