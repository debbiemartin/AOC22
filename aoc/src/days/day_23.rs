use crate::Problem;
use std::collections::{HashSet, HashMap};

pub struct DayTwentyThree;

struct Map {
    elves: Vec<(i64, i64)>,
    preference: usize
}

impl Map {
    fn free_squares(&self) -> u64 {
        let x_min = self.elves.iter().map(|x| x.0).min().unwrap();
        let x_max = self.elves.iter().map(|x| x.0).max().unwrap();
        let y_min = self.elves.iter().map(|x| x.1).min().unwrap();
        let y_max = self.elves.iter().map(|x| x.1).max().unwrap();

        return (x_max - x_min + 1) as u64 * (y_max - y_min + 1) as u64 - self.elves.len() as u64
    }

    fn move_elf(&self, coord: (i64, i64), current_elves: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
        let (x,y) = coord;
        if ![(x-1, y+1), (x-1, y), (x-1, y-1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)].iter().any(|x| current_elves.contains(x)) {
            // No surrounding elves - don't move
            return None;
        }

        let trials = ["N", "S", "W", "E"];
        for i in 0..4 {
            let trial = trials[(i+self.preference) % 4];
            let (trials, result) = match trial {
                "N" => ([(x, y-1), (x+1, y-1), (x-1,y-1)], (x, y-1)),
                "S" => ([(x, y+1), (x+1, y+1), (x-1,y+1)], (x, y+1)),
                "W" => ([(x-1, y), (x-1, y-1), (x-1,y+1)], (x-1, y)),
                "E" => ([(x+1, y), (x+1, y-1), (x+1,y+1)], (x+1, y)),
                _ => panic!("Don't recognise {trial}"),
            };

            if ! trials.iter().any(|x| current_elves.contains(x)) {
                return Some(result);
            }
        }

        return None;
    }

    fn move_elves(&mut self) -> bool {
        let mut moved = false;
        let current_elves: HashSet<(i64, i64)> = self.elves.iter().map(|x| *x).collect();
        let mut elf_move_count: HashMap<(i64, i64), u32> = HashMap::new();
        let mut elf_moves: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

        for elf in &self.elves {
            if let Some(elf_move) = self.move_elf(*elf, &current_elves) {
                elf_moves.insert(*elf, elf_move);
                *elf_move_count.entry(elf_move).or_insert(0) += 1;
            }
        }
        
        self.preference = (self.preference + 1) % 4;

        // Make the new set of elves
        for i in 0..self.elves.len() {
            if let Some(elf_move) = elf_moves.get(&self.elves[i]) {
                if elf_move_count[&elf_move] == 1 {
                    self.elves[i] = *elf_move;
                    moved = true;
                }
            }
        }

        return moved;
    }

    fn move_elves_multi(&mut self, count: u8) {
        for _ in 0..count {
            self.move_elves();
        }
    }

    fn move_elves_until_stationary(&mut self) -> u32 {
        let mut i = 0;

        loop {
            i += 1;
            let moved = self.move_elves();
            if ! moved {
                return i;
            }
        }
    }

    fn new(input: &str) -> Map {
        let mut elves = Vec::new();

        for (i, line) in input.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character == '#' {
                    elves.push((j as i64, i as i64));
                }
            } 
        }

        Map { elves, preference: 0 }
    }
}


impl Problem for DayTwentyThree {
    fn part_one(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        map.move_elves_multi(10);
        let free_squares = map.free_squares();
        format!("Free squares: {free_squares}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        let moves = map.move_elves_until_stationary();
        format!("Moves until stationary: {moves}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_map_p1() {
        let output = DayTwentyThree{}.part_one(&INPUT);
        assert_eq!(output, "Free squares: 110")
    }

    #[test]
    fn test_map_p2() {
        let output = DayTwentyThree{}.part_two(&INPUT);
        assert_eq!(output, "Moves until stationary: 20")
    }
}
