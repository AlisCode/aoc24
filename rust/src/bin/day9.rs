use aoc24::aoc;

#[derive(Debug)]
struct Disk {
    data: Vec<Option<usize>>,
    first_empty_space: usize,
    last_file_idx: usize,
}

impl Disk {
    fn move_to_next_empty_space(&mut self) {
        let (index, _) = &self.data[self.first_empty_space..]
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_none())
            .expect("No more empty space");
        self.first_empty_space += index;
    }

    fn move_to_last_file_idx(&mut self) {
        let (last_file_idx_count, _) = self.data[0..=self.last_file_idx]
            .iter()
            .rev()
            .enumerate()
            .find(|(_, v)| v.is_some())
            .expect("No chunk of data");
        self.last_file_idx -= last_file_idx_count;
    }

    fn maybe_find_available_space_of_size(
        &self,
        last_file_size: usize,
        last_file_idx: usize,
    ) -> Option<usize> {
        let mut idx = self.first_empty_space;
        loop {
            let available_space = self.data[idx..].iter().take_while(|x| x.is_none()).count();
            if available_space >= last_file_size {
                return Some(idx);
            }
            let occupied = self.data[idx..].iter().take_while(|x| x.is_some()).count();
            idx += available_space + occupied;
            if idx >= last_file_idx {
                return None;
            }
        }
    }

    fn file_len_ending_at(&self, idx: usize) -> usize {
        let v = self.data[idx];
        self.data[0..=idx]
            .iter()
            .rev()
            .take_while(|x| **x == v)
            .count()
    }

    fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, v)| match v {
                Some(data) => data * idx,
                None => 0,
            })
            .sum()
    }
}

fn parse(input: &str) -> Disk {
    let mut file = true;
    let mut data = Vec::new();
    for (idx, c) in input.chars().enumerate() {
        let space = (c as u8 - b'0') as usize;
        let file_id = idx / 2;
        let value = match file {
            true => Some(file_id),
            false => None,
        };
        data.extend((0..space).map(|_| value));
        file = !file;
    }
    let mut disk = Disk {
        last_file_idx: data.len() - 1,
        first_empty_space: 0,
        data,
    };
    disk.move_to_next_empty_space();
    disk.move_to_last_file_idx();
    disk
}

fn part_one(input: &str) -> usize {
    let mut disk = parse(input);
    while disk.last_file_idx > disk.first_empty_space {
        disk.data.swap(disk.last_file_idx, disk.first_empty_space);
        disk.move_to_next_empty_space();
        disk.move_to_last_file_idx();
    }
    disk.checksum()
}

fn part_two(input: &str) -> usize {
    let mut disk = parse(input);
    let mut idx = disk.last_file_idx;
    while idx > disk.first_empty_space {
        let file_len = disk.file_len_ending_at(idx);
        if let Some(available_space) = disk.maybe_find_available_space_of_size(file_len, idx) {
            // TODO: swap slices ?
            for i in 0..file_len {
                disk.data.swap(idx - i, available_space + i);
            }
        }
        idx -= file_len;
        let blank_space = disk.data[..=idx]
            .iter()
            .rev()
            .take_while(|x| x.is_none())
            .count();
        idx -= blank_space;
    }
    disk.checksum()
}

aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn day9() {
        assert_eq!(part_one(INPUT), 1928);
        assert_eq!(part_two(INPUT), 2858);
    }
}
