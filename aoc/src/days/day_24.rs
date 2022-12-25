use crate::Problem;
use std::collections::{VecDeque, HashSet};

pub struct DayTwentyFour;

#[derive(Debug)]
enum Direction {
    Up, 
    Down,
    Left,
    Right
}

struct Map {
    blizzards: Vec<Vec<Option<Direction>>>,
    x_extent: u32,
    y_extent: u32,
    memoize: Vec<HashSet<(u32, u32)>>
}

impl Map {
    fn update_blizzards(&mut self, move_num: u32) {
        let mut blizzards = HashSet::new();

        for (i, line) in self.blizzards.iter().enumerate() {
            for (j, direction) in line.iter().enumerate() {
                let x_b = j as i32;
                let y_b = i as i32;

                if direction.is_none() {
                    continue;
                }

                let blizzard_coord = match direction.as_ref().unwrap() {
                    Direction::Up => (x_b as u32, (((y_b-move_num as i32 - 1).rem_euclid(self.y_extent as i32-2)) + 1) as u32),
                    Direction::Down => (x_b as u32, (((y_b+move_num as i32 - 1).rem_euclid(self.y_extent as i32-2)) + 1) as u32),
                    Direction::Left => ((((x_b-move_num as i32 - 1).rem_euclid(self.x_extent as i32-2)) + 1) as u32, y_b as u32),
                    Direction::Right => ((((x_b+move_num as i32 - 1).rem_euclid(self.x_extent as i32-2)) + 1) as u32, y_b as u32),
                };
                blizzards.insert(blizzard_coord);
            }
        }

        self.memoize.push(blizzards);
        return;
    }

    fn can_move(&mut self, x: u32, y: u32, move_num: u32) -> bool {
        if x == self.x_extent-2 && y == self.y_extent-1 {
            return true;
        }

        if x == 1 && y == 0 {
            return true;
        }

        if x == 0 || y == 0 || x >= self.x_extent-1 || y >= self.y_extent-1 {
            // Reached a side wall 
            return false;
        }

        if self.memoize.len() < move_num as usize {
            self.update_blizzards(move_num);
        }
        return ! self.memoize[move_num as usize-1].contains(&(x, y));
    }

    fn minimum_moves(&mut self, start_num: u32, reverse: bool) -> Option<u32> {
        let (from, to) = match reverse {
            false => ((1,0), (self.x_extent-2, self.y_extent-1)),
            true => ((self.x_extent-2, self.y_extent-1), (1,0))
        };

        let mut queue = VecDeque::from([(from, start_num)]);
        let mut visited: HashSet<((u32, u32), u32)> = HashSet::new();

        loop {
            let ((x, y), current_distance) = match queue.pop_front() {
                None => {return None;},
                Some(x) => x,
            };

            if (x, y) == to {
                return Some(current_distance);
            }
        
            let mut possible_squares = Vec::from([(x+1, y), (x, y+1)]);
            if x > 0 {
                possible_squares.push((x-1, y));
            }
            if y > 0 {
                possible_squares.push((x, y-1));
            }
            
            for square in possible_squares {
                if !visited.contains(&(square, current_distance + 1)) && self.can_move(square.0, square.1, current_distance+1) {
                    visited.insert((square, current_distance + 1));
                    queue.push_back((square, current_distance + 1));
                }
            }
    
            // Wait here for a turn
            if !visited.contains(&((x, y), current_distance + 1)) && self.can_move(x, y, current_distance+1) {
                visited.insert(((x, y), current_distance + 1));
                queue.push_back(((x, y), current_distance + 1));
            }
        }
    }

    fn new(input: &str) -> Map {
        let mut map = Vec::new();

        let y_extent = input.lines().collect::<Vec<&str>>().len() as u32;
        let x_extent = input.lines().collect::<Vec<&str>>()[0].len() as u32;
        for line in input.lines() {
            let row = line.chars().map(|character| 
                match character {
                '^' => Some(Direction::Up),
                '>' => Some(Direction::Right),
                '<' => Some(Direction::Left),
                'v' => Some(Direction::Down),
                _ => None
            }).collect();
            map.push(row);
        }


        Map { blizzards: map, x_extent, y_extent, memoize: Vec::new() }
    }
}


impl Problem for DayTwentyFour {
    fn part_one(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        let moves = map.minimum_moves(0, false).unwrap();
        format!("Minimum moves: {moves}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        let leg_1 = map.minimum_moves(0, false).unwrap();
        let leg_2 = map.minimum_moves(leg_1, true).unwrap();
        let leg_3 = map.minimum_moves(leg_2, false).unwrap();
        format!("Minimum moves: {leg_3}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_map_p1() {
        let output = DayTwentyFour{}.part_one(&INPUT);
        assert_eq!(output, "Minimum moves: 18")
    }

    #[test]
    fn test_map_p2() {
        let output = DayTwentyFour{}.part_two(&INPUT);
        assert_eq!(output, "Minimum moves: 54")
    }
}
