use aoc24::aoc;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, IResult};

#[derive(Debug, Default)]
struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
}

#[derive(Debug, PartialEq)]
enum RunResult {
    Ok,
    Output(u64),
    Halted,
}

impl Cpu {
    pub fn combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }
    pub fn run(&mut self, instructions: &[u64]) -> RunResult {
        if self.instruction_pointer >= instructions.len() {
            return RunResult::Halted;
        }

        let cmd = Operation::from(instructions[self.instruction_pointer]);
        let operand = instructions[self.instruction_pointer + 1];

        let mut skip_incr = false;
        let mut out: Option<u64> = None;

        match cmd {
            Operation::Adv => {
                let combo_operand = self.combo_operand(operand);
                let new_a = self.a / 2u64.pow(combo_operand as u32);
                self.a = new_a;
            }
            Operation::Bxl => {
                let new_b = self.b ^ operand;
                self.b = new_b;
            }
            Operation::Bst => {
                let combo_operand = self.combo_operand(operand);
                let new_b = combo_operand % 8;
                self.b = new_b;
            }
            Operation::Jnz => {
                if self.a != 0 {
                    self.instruction_pointer = operand as usize;
                    skip_incr = true;
                }
            }
            Operation::Bxc => {
                let new_b = self.b ^ self.c;
                self.b = new_b;
            }
            Operation::Out => {
                let value = self.combo_operand(operand) % 8;
                out = Some(value);
            }
            Operation::Bdv => {
                let combo_operand = self.combo_operand(operand);
                let new = self.a / 2u64.pow(combo_operand as u32);
                self.b = new;
            }
            Operation::Cdv => {
                let combo_operand = self.combo_operand(operand);
                let new = self.a / 2u64.pow(combo_operand as u32);
                self.c = new;
            }
        }

        if !skip_incr {
            self.instruction_pointer += 2;
        }

        match out {
            Some(x) => RunResult::Output(x),
            None => RunResult::Ok,
        }
    }
}

enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Operation {
    fn from(value: u64) -> Self {
        match value {
            0 => Operation::Adv,
            1 => Operation::Bxl,
            2 => Operation::Bst,
            3 => Operation::Jnz,
            4 => Operation::Bxc,
            5 => Operation::Out,
            6 => Operation::Bdv,
            7 => Operation::Cdv,
            _ => panic!("Unknown operation {value}"),
        }
    }
}

fn parse(input: &str) -> IResult<&str, (Cpu, Vec<u64>)> {
    let (input, _) = tag("Register A: ")(input)?;
    let (input, a) = nom::character::complete::u64(input)?;
    let (input, _) = tag("\nRegister B: ")(input)?;
    let (input, b) = nom::character::complete::u64(input)?;
    let (input, _) = tag("\nRegister C: ")(input)?;
    let (input, c) = nom::character::complete::u64(input)?;

    let (input, _) = tag("\n\nProgram: ")(input)?;
    let (input, instructions) = separated_list0(tag(","), nom::character::complete::u64)(input)?;

    Ok((
        input,
        (
            Cpu {
                a,
                b,
                c,
                instruction_pointer: 0,
            },
            instructions,
        ),
    ))
}

fn run_to_completion(cpu: &mut Cpu, instructions: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    loop {
        match cpu.run(instructions) {
            RunResult::Ok => (),
            RunResult::Halted => {
                break;
            }
            RunResult::Output(x) => {
                out.push(x);
            }
        }
    }
    out
}

fn part_one(input: &str) -> String {
    let (_leftover, (mut cpu, instructions)) =
        parse(input).expect("Failed to parse machine and instructions");
    let out = run_to_completion(&mut cpu, &instructions);
    out.into_iter().join(",")
}

fn part_two(input: &str) -> u64 {
    let (_leftover, (mut cpu, instructions)) =
        parse(input).expect("Failed to parse machine and instructions");
    // assume last instruction is jnz
    let program_oneshot = &instructions[..instructions.len() - 2];
    find_a_value(&mut cpu, program_oneshot, &instructions, 0).expect("Failed to find a value")
}

fn find_a_value(cpu: &mut Cpu, program_oneshot: &[u64], target: &[u64], a: u64) -> Option<u64> {
    if target.is_empty() {
        return Some(a);
    }
    // We're looking to display the last instruction of the program
    let wanted_output = target.last().expect("At least one value in target");
    let b = cpu.b;
    let c = cpu.c;
    // The first instruction of the inputs is always "a modulo 8", meaning we're looking for a value
    // between 0 and 7 that gives the correct output for the program
    for x in 0..8 {
        let reg_a = a << 3 | x;
        cpu.a = reg_a;
        cpu.b = b;
        cpu.c = c;
        cpu.instruction_pointer = 0;
        // Running the program in one-shot should always output just one value
        let out = run_to_completion(cpu, program_oneshot);
        assert!(out.len() == 1);

        if out[0] == *wanted_output {
            if let Some(v) = find_a_value(cpu, program_oneshot, &target[..target.len() - 1], reg_a)
            {
                return Some(v);
            }
        }
    }
    None
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT_TWO: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn day17_unit() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut cpu = Cpu {
            c: 9,
            ..Cpu::default()
        };
        assert_eq!(cpu.run(&[2, 6]), RunResult::Ok);
        assert_eq!(cpu.b, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut cpu = Cpu {
            a: 10,
            ..Cpu::default()
        };
        assert_eq!(
            run_to_completion(&mut cpu, &[5, 0, 5, 1, 5, 4]),
            vec![0, 1, 2]
        );

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut cpu = Cpu {
            a: 2024,
            ..Cpu::default()
        };
        assert_eq!(
            run_to_completion(&mut cpu, &[0, 1, 5, 4, 3, 0]),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(cpu.a, 0);

        // Other test cases in the problem ...
    }

    #[test]
    fn day17() {
        assert_eq!(part_one(INPUT), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part_two(INPUT_TWO), 117440);
    }
}
