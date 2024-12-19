use aoc24::aoc;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashSet;

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut digits = line
                .split(',')
                .map(|x| x.parse().expect("Failed to parse digit"));
            (
                digits.next().expect("two digits"),
                digits.next().expect("Two digits"),
            )
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    solve_part_one(input, 70, 1024)
}

fn part_two(input: &str) -> String {
    let (a, b) = solve_part_two(input, 70);
    format!("{a},{b}")
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve_part_one(input: &str, grid_size: i32, take: usize) -> usize {
    let bytes: HashSet<(i32, i32)> = parse(input).into_iter().take(take).collect();
    let (_, cost) = dijkstra(
        &(0, 0),
        |(x, y)| {
            let mut nodes: Vec<((i32, i32), usize)> = Vec::with_capacity(4);
            for (dx, dy) in NEIGHBORS {
                let x = x + dx;
                let y = y + dy;
                if (0..=grid_size).contains(&x)
                    && (0..=grid_size).contains(&y)
                    && !bytes.contains(&(x, y))
                {
                    nodes.push(((x, y), 1));
                }
            }
            nodes
        },
        |(x, y)| *x == grid_size && *y == grid_size,
    )
    .expect("Failed to find path");
    cost
}

fn solve_part_two(input: &str, grid_size: i32) -> (i32, i32) {
    let all_bytes = parse(input);
    let mut min = 0;
    let mut max = all_bytes.len();
    // Manual binary search to find the first byte that blocks the path.
    // Could be rewritten to use part 1, and could be done faster ig
    let idx = loop {
        if min == max {
            break min;
        }
        let idx = (max - min) / 2 + min;
        let bytes: HashSet<_> = all_bytes.iter().take(idx + 1).copied().collect();
        let has_path = dijkstra(
            &(0, 0),
            |(x, y)| {
                let mut nodes: Vec<((i32, i32), usize)> = Vec::with_capacity(4);
                for (dx, dy) in NEIGHBORS {
                    let x = x + dx;
                    let y = y + dy;
                    if (0..=grid_size).contains(&x)
                        && (0..=grid_size).contains(&y)
                        && !bytes.contains(&(x, y))
                    {
                        nodes.push(((x, y), 1));
                    }
                }
                nodes
            },
            |(x, y)| *x == grid_size && *y == grid_size,
        )
        .is_some();
        if max - min == 1 {
            if has_path {
                break min;
            }
            break max;
        }
        if has_path {
            min = idx;
        } else {
            max = idx;
        }
    };
    all_bytes[idx + 1]
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn day18() {
        assert_eq!(solve_part_one(INPUT, 6, 12), 22);
        assert_eq!(solve_part_two(INPUT, 6), (6, 1));
    }
}
