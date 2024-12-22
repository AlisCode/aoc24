use aoc24::aoc;
use fxhash::FxHashMap;
use itertools::Itertools;

struct SecretNumber {
    secret: i64,
}

impl Iterator for SecretNumber {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let SecretNumber { secret } = self;

        let a = *secret * 64;
        *secret ^= a;
        *secret %= 16777216;

        let b = *secret / 32;
        *secret ^= b;
        *secret %= 16777216;

        let c = *secret * 2048;
        *secret ^= c;
        *secret %= 16777216;

        Some(*secret)
    }
}

struct PriceIter {
    previous: i64,
    secret: SecretNumber,
}

impl Iterator for PriceIter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let secret = self.secret.next()?;
        let nb = secret % 10;
        let delta = nb - self.previous;
        self.previous = nb;
        Some((delta, nb))
    }
}

impl PriceIter {
    fn new(secret: i64) -> Self {
        let previous = secret % 10;
        PriceIter {
            previous,
            secret: SecretNumber { secret },
        }
    }
}

fn part_one(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    numbers
        .into_iter()
        .filter_map(|x| SecretNumber { secret: x }.nth(1999))
        .sum()
}

fn part_two(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    numbers
        .into_iter()
        .map(|x| {
            PriceIter::new(x).take(2000).tuple_windows().fold(
                FxHashMap::<[i64; 4], i64>::default(),
                |mut acc, ((da, _), (db, _), (dc, _), (dd, bananas))| {
                    let key = [da, db, dc, dd];
                    acc.entry(key).or_insert(bananas);
                    acc
                },
            )
        })
        .fold(FxHashMap::<[i64; 4], i64>::default(), |mut acc, entries| {
            for (k, v) in entries.into_iter() {
                let entry = acc.entry(k).or_default();
                *entry += v;
            }
            acc
        })
        .into_values()
        .max()
        .expect("Failed to find max")
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024";

    const SECOND_INPUT: &str = "1
2
3
2024";

    #[test]
    fn day22_secret_number() {
        let mut iter = SecretNumber { secret: 123 };
        assert_eq!(iter.next().expect("next number"), 15887950);
        assert_eq!(iter.next().expect("next number"), 16495136);
        assert_eq!(iter.next().expect("next number"), 527345);
        assert_eq!(iter.next().expect("next number"), 704524);
    }

    #[test]
    fn day22_price_iter() {
        let mut iter = PriceIter::new(123);
        assert_eq!(iter.next().expect("next price"), (-3, 0));
        assert_eq!(iter.next().expect("next price"), (6, 6));
        assert_eq!(iter.next().expect("next price"), (-1, 5));
        assert_eq!(iter.next().expect("next price"), (-1, 4));
    }

    #[test]
    fn day22() {
        assert_eq!(part_one(INPUT), 37327623);
        assert_eq!(part_two(SECOND_INPUT), 23);
    }
}
