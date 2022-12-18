use crate::Problem;
use std::collections::HashSet;
use std::cmp::max;

pub struct DaySeventeen;

const CHAMBER_WIDTH: u32 = 7;

const SHAPES: [&'static [(u32, u32)]; 5]= [
    // ####
    &[(0,0), (1,0), (2,0), (3,0)],
    // .#.
    // ###
    // .#.
    &[(1,0), (0,1), (1,1), (2,1), (1,2)],
    // ..#
    // ..#
    // ###
    &[(0,0), (1,0), (2,0), (2,1), (2,2)],
    // #
    // #
    // #
    // #
    &[(0,0), (0,1), (0,2), (0,3)],
    // ##
    // ##
    &[(0,0), (0,1), (1,0), (1,1)],
];

const SHAPE_COUNT: u32 = 5;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

struct Pyroclastic {
    height: u64,
    coords: HashSet<(u32, u64)>,
    directions: Vec<Direction>,
    direction_count: usize
}

impl Pyroclastic {
    fn empty_space(&self, shape: &[(u32, u32)], x: u32, y: u64) -> bool {
        return {
            ! shape.iter().any(|coord| self.coords.contains(&(coord.0 + x, coord.1 as u64 + y)))
        }
    }


    fn run(&mut self, shape_total: u64, stop_at_cycle: bool) -> Option<u64> {
        let mut direction_index = 0;
        for shape_count in 0..shape_total {
            let shape_index = (shape_count % SHAPE_COUNT as u64) as usize;
            let shape = SHAPES[shape_index];
            let mut x = 3;
            let mut y = self.height + 4;
            let mut first_move = true;
            loop {
                if stop_at_cycle && shape_count != 0 && shape_index == 0 && direction_index == 0 && !first_move {
                    return Some(shape_count)
                }

                // Move left/right
                let direction = &self.directions[direction_index as usize];
                direction_index = (direction_index + 1) % self.direction_count;
                first_move = false;

                let x_trial: u32 = match direction {
                    Direction::Right => {x + 1},
                    Direction::Left => {x - 1},
                }; 


                if x_trial > 0 && ! shape.iter().any(|coord| coord.0 + x_trial > CHAMBER_WIDTH)
                    && self.empty_space(shape, x_trial, y) {
                    x = x_trial;
                }

                // Move down
                if y > 1 && self.empty_space(shape, x, y - 1) {
                    y -= 1;
                } else {
                    self.height = max(self.height, shape.iter().map(|x| x.1 as u64 + y).max().unwrap());
                    for coord in shape {
                        self.coords.insert((coord.0 + x, coord.1 as u64 + y));
                    }

                    break;
                }

            }
        }

        return None //@@@ split this out into two different functions?
    }

    fn new(input: &str) -> Pyroclastic { 
        let coords: HashSet<(u32, u64)> = HashSet::new();
        let height = 0;
        let mut directions: Vec<Direction> = Vec::new();
        for direction in input.chars() {
            directions.push(
                match direction {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => panic!("Unrecognised direction {direction}")
                }
            )
        }
        let direction_count = directions.len();

        Pyroclastic { height, coords, directions, direction_count }
    }
}


impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let mut pyroclastic = Pyroclastic::new(&input);
        pyroclastic.run(2022, false);
        let height = pyroclastic.height;
        format!("Height: {height}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut pyroclastic_cycle = Pyroclastic::new(&input);
        let cycle = pyroclastic_cycle.run(1000000000000, true).unwrap();
        let cycle_height = pyroclastic_cycle.height;

        let mut pyroclastic = Pyroclastic::new(&input);
        pyroclastic.run(1000000000000 % cycle, false);
        let height = cycle_height * (1000000000000/cycle) + pyroclastic.height;
        format!("Height: {height}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_beacons_p1() {
        let output = DaySeventeen{}.part_one(&INPUT);
        assert_eq!(output, "Height: 3068")
    }

    #[test]
    fn test_beacons_p2() {
        let output = DaySeventeen{}.part_two(&INPUT);
        assert_eq!(output, "Height: 1514285714288")
    }
}
