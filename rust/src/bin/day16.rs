use aoc24::aoc;
use fxhash::FxHashSet;

struct Labyrinth {
    grid: Vec<char>,
    width: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turns(self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::West, Direction::East],
            Direction::South => [Direction::West, Direction::East],
            Direction::East => [Direction::North, Direction::South],
            Direction::West => [Direction::North, Direction::South],
        }
    }
}

type Node = (usize, Direction);
impl Labyrinth {
    // I bet this could be done smarter and guide astar better
    pub fn heuristic(&self, idx: usize) -> usize {
        let x = idx % self.width;
        let y = idx / self.width;
        let endx = self.end % self.width;
        let endy = self.end / self.width;
        let dx = x as f32 - endx as f32;
        let dy = y as f32 - endy as f32;
        (dx * dx + dy * dy).sqrt() as usize
    }

    pub fn neighbor(&self, idx: usize, dir: Direction) -> usize {
        let x = idx % self.width;
        let y = idx / self.width;
        let (nx, ny) = match dir {
            Direction::North => (x, y.checked_sub(1).expect("invalid coord")),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x.checked_sub(1).expect("invalid coord"), y),
        };
        ny * self.width + nx
    }

    // NOTE: I think Rust Edition 2024 gets rid of the 'a hack ?
    pub fn neighbors<'a>(&'a self, node: &Node) -> impl Iterator<Item = (Node, usize)> + 'a {
        let next = ((self.neighbor(node.0, node.1), node.1), 1);
        std::iter::once(next)
            .chain(
                node.1
                    .turns()
                    .map(|dir| ((self.neighbor(node.0, dir), dir), 1001)),
            )
            .filter(|((idx, _), _)| self.grid[*idx] != '#')
    }
}

fn parse(input: &str) -> Labyrinth {
    let mut start = 0;
    let mut end = 0;
    let width = input.find('\n').expect("to find length");
    let mut grid = Vec::with_capacity(width * width);
    for c in input.chars() {
        match c {
            'E' => end = grid.len(),
            'S' => start = grid.len(),
            '\n' => continue,
            _ => (),
        }
        grid.push(c);
    }

    Labyrinth {
        grid,
        width,
        start,
        end,
    }
}

fn part_one(input: &str) -> usize {
    let laby = parse(input);
    let start = (laby.start, Direction::East);
    let (_path, cost) = pathfinding::directed::astar::astar(
        &start,
        |node| laby.neighbors(node),
        |(idx, _)| laby.heuristic(*idx),
        |(idx, _)| laby.end == *idx,
    )
    .expect("Failed to find path");
    cost
}

fn part_two(input: &str) -> usize {
    let laby = parse(input);
    let start = (laby.start, Direction::East);
    let (all_path, _) = pathfinding::directed::astar::astar_bag(
        &start,
        |node| laby.neighbors(node),
        |(idx, _)| laby.heuristic(*idx),
        |(idx, _)| laby.end == *idx,
    )
    .expect("Failed to find path");

    all_path
        .into_iter()
        .flat_map(|path| path.into_iter().map(|(idx, _dir)| idx))
        .collect::<FxHashSet<_>>()
        .len()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn day16() {
        assert_eq!(part_one(INPUT), 7036);
        assert_eq!(part_two(INPUT), 45);
    }
}
