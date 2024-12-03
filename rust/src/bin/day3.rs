use aoc24::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many0, many_till},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
struct MulToken {
    a: i32,
    b: i32,
}

enum Token {
    Mul(MulToken),
    Do,
    Dont,
}

fn parse_mul_token(input: &str) -> IResult<&str, MulToken> {
    let (input, _mul) = tag("mul(")(input)?;
    let (input, a) = nom::character::complete::i32(input)?;
    let (input, _comma) = tag(",")(input)?;
    let (input, b) = nom::character::complete::i32(input)?;
    let (input, _mul_end) = tag(")")(input)?;
    Ok((input, MulToken { a, b }))
}

fn parse_next_token(input: &str) -> IResult<&str, Token> {
    let parse_mul_token = parse_mul_token.map(Token::Mul);
    let parse_do = tag("do()").map(|_| Token::Do);
    let parse_dont = tag("don't()").map(|_| Token::Dont);
    let parse_token = alt((parse_mul_token, parse_do, parse_dont));
    let (input, (_garbage, token)) = many_till(map(anychar, drop), parse_token)(input)?;
    Ok((input, token))
}

fn parse(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, token) = many0(parse_next_token)(input)?;
    Ok((input, token))
}

fn part_one(input: &str) -> i32 {
    let (_leftover, tokens) = parse(input).expect("Failed to parse input");
    tokens
        .into_iter()
        .filter_map(|tok| match tok {
            Token::Mul(MulToken { a, b }) => Some(a * b),
            Token::Do | Token::Dont => None,
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let (_leftover, tokens) = parse(input).expect("Failed to parse input");

    let (_enabled, total) = tokens
        .into_iter()
        .fold((true, 0), |(enabled, total), tok| match tok {
            Token::Do => (true, total),
            Token::Dont => (false, total),
            Token::Mul(MulToken { a, b }) if enabled => (enabled, total + a * b),
            Token::Mul(MulToken { .. }) => (enabled, total),
        });
    total
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_mul_token() {
        assert_eq!(
            parse_mul_token("mul(10,25)"),
            Ok(("", MulToken { a: 10, b: 25 }))
        );

        assert_eq!(
            parse_mul_token("mul(1,2)az3"),
            Ok(("az3", MulToken { a: 1, b: 2 }))
        );
    }

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_TWO: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day3() {
        assert_eq!(part_one(INPUT), 161);
        assert_eq!(part_two(INPUT_TWO), 48);
    }
}
