use aoc24::aoc;
use nom::{bytes::complete::tag, IResult};

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    // I've done the math.
    fn solve(&self) -> Option<(i64, i64)> {
        let (xa, ya) = self.a;
        let (xb, yb) = self.b;
        let (xp, yp) = self.prize;

        // Solve the following system of equation :
        //
        // Let alpha be the number of presses on the button A
        // Let beta be the number of presses on the button B
        //
        // +-
        // |   alpha*xa + beta*xb = xp
        // |   alpha*ya + beta*yb = yp
        // +-
        //
        // Valid answers are positive integers only, and we
        // want to discard all divisions by 0.

        let beta_nom = xa * yp - ya * xp;
        let beta_denom = xa * yb - ya * xb;

        if beta_denom == 0 || beta_nom % beta_denom != 0 {
            return None;
        }

        let beta = beta_nom / beta_denom;

        let alpha_nom = xp - xb * beta;
        let alpha_denom = xa;

        if alpha_denom == 0 || alpha_nom % alpha_denom != 0 {
            return None;
        }
        let alpha = (xp - xb * beta) / xa;

        if alpha < 0 || beta < 0 {
            return None;
        }

        Some((alpha, beta))
    }
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, _) = tag("Button A: X+")(input)?;
    let (input, xa) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, ya) = nom::character::complete::i64(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, _) = tag("Button B: X+")(input)?;
    let (input, xb) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, yb) = nom::character::complete::i64(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, _) = tag("Prize: X=")(input)?;
    let (input, xp) = nom::character::complete::i64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, yp) = nom::character::complete::i64(input)?;

    Ok((
        input,
        Machine {
            a: (xa, ya),
            b: (xb, yb),
            prize: (xp, yp),
        },
    ))
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| {
            let (_input, machine) = parse_machine(machine).expect("to parse machine");
            machine
        })
        .collect()
}

fn part_one(input: &str) -> i64 {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|machine| {
            machine.solve().and_then(|(a, b)| {
                if a <= 100 && b <= 100 {
                    Some(a * 3 + b)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|mut machine| {
            machine.prize.0 += 10000000000000;
            machine.prize.1 += 10000000000000;
            machine.solve().map(|(a, b)| a * 3 + b)
        })
        .sum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn day14_solve_machine() {
        let machines = parse(INPUT);
        assert_eq!(machines[0].solve(), Some((80, 40)));
        assert_eq!(machines[1].solve(), None);
        assert_eq!(machines[2].solve(), Some((38, 86)));
    }

    #[test]
    fn day13() {
        assert_eq!(part_one(INPUT), 480);
    }
}
