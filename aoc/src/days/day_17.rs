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
    height: u32,
    coords: HashSet<(u32, u32)>,
    directions: Vec<Direction>,
    direction_count: usize
}

impl Pyroclastic {
    fn empty_space(&self, shape: &[(u32, u32)], x: u32, y: u32) -> bool {
        return {
            ! shape.iter().any(|coord| self.coords.contains(&(coord.0 + x, coord.1 + y)))
        }
    }
    
    fn run(&mut self, shape_number: u32) {
        let mut direction_index = 0;
        for shape_index in 0..shape_number {
            let shape = SHAPES[(shape_index % SHAPE_COUNT) as usize];
            let mut x = 3;
            let mut y = self.height + 4;
            loop {
                // Move left/right
                let direction = &self.directions[(direction_index % self.direction_count) as usize];
                direction_index += 1;

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
                    self.height = max(self.height, shape.iter().map(|x| x.1 + y).max().unwrap());
                    for coord in shape {
                        self.coords.insert((coord.0 + x, coord.1 + y));
                    }

                    break;
                }

            }
        }
    }

    fn new(input: &str) -> Pyroclastic { 
        let coords: HashSet<(u32, u32)> = HashSet::new();
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
        pyroclastic.run(2022);
        let height = pyroclastic.height;
        format!("Height: {height}")
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
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
}
