use aoc24::aoc;

#[derive(Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn safe(&self) -> bool {
        let cmp = self.levels[0].cmp(&self.levels[1]);
        self.levels
            .iter()
            .zip(self.levels.iter().skip(1))
            .all(|(a, b)| a.cmp(b) == cmp && (1..=3).contains(&i32::abs(a - b)))
    }

    pub fn without(&self, idx: usize) -> Report {
        let mut report = self.clone();
        report.levels.remove(idx);
        report
    }
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split(" ")
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect();
            Report { levels }
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|r| r.safe()).count()
}

// It ain't stupid if it works.
fn part_two(input: &str) -> usize {
    let reports = parse(input);
    reports
        .iter()
        .filter(|r| {
            (0..r.levels.len())
                .map(|idx| r.without(idx))
                .any(|r| r.safe())
                || r.safe()
        })
        .count()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn day2() {
        assert_eq!(part_one(INPUT), 2);
        assert_eq!(part_two(INPUT), 4);
    }
}
