use crate::Problem;
use regex::Regex;

pub struct DayTwentyTwo;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn anticlockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3
        }
    }
}

#[derive(Debug)]
enum Square {
    Open,
    Wall,
    Empty
}

struct Map {
    current_square: (usize, usize),
    current_direction: Direction,
    rows: Vec<Vec<Square>>,
    row_count: usize,
    col_count: usize,
    instructions: Vec<(bool, u8)>
}

impl Map {
    fn score(&self) -> u32 {
        1000 * (self.current_square.1 as u32 + 1) + 4 * (self.current_square.0 as u32 + 1) + self.current_direction.score()
    }

    fn move_one(&mut self) -> bool {
        let mut new_square = self.current_square;
        loop {
            new_square = match self.current_direction {
                Direction::Up => (new_square.0, (new_square.1-1).rem_euclid(self.row_count)),
                Direction::Down => (new_square.0, (new_square.1+1).rem_euclid(self.row_count)),
                Direction::Left => ((new_square.0-1).rem_euclid(self.col_count), new_square.1),
                Direction::Right => ((new_square.0+1).rem_euclid(self.col_count), new_square.1),
            };

            match self.rows[new_square.1][new_square.0] {
                Square::Empty => continue,
                Square::Open => {self.current_square=new_square; return true;},
                Square::Wall => {return false;},
            }
        }
    }

    fn do_instructions(&mut self) {
        let instructions = self.instructions.clone();
        for instruction in instructions {
            self.current_direction = match instruction.0 {
                true => self.current_direction.clockwise(),
                false => self.current_direction.anticlockwise(),
            };

            for _ in 0..instruction.1 {
                let able_to_move = self.move_one();
                if ! able_to_move {
                    break;
                }
            };
        }
    }

    fn new(input: &str) -> Map {
        let map_instructions: Vec<&str> = input.split("\n\n").collect();

        let row_count = map_instructions[0].lines().collect::<Vec<&str>>().len();
        let col_count = map_instructions[0].lines().map(|x| x.len()).max().unwrap();

        let mut rows = Vec::new();
        for line in map_instructions[0].lines() {
            let chars: Vec<char> = line.chars().collect(); 
            let mut row = Vec::new();
            for i in 0..col_count {
                if i >= chars.len() {
                    row.push(Square::Empty);
                } else {
                    row.push(
                        match chars[i] {
                            ' ' => Square::Empty,
                            '.' => Square::Open,
                            '#' => Square::Wall,
                            _ => panic!("Unrecognised")
                        }
                    );
                }
            }
            rows.push(row);
        }
        let current_square = (rows[0].iter().position(|x| matches!(x, Square::Open)).unwrap(), 0);

        let mut instructions = Vec::new();
        let re = Regex::new(r"(?P<lr>[LR])(?P<number>[0-9]+)").unwrap();
        for cap in re.captures_iter(&format!("R{}", map_instructions[1])[..]) {
            let clockwise = match cap.name("lr").unwrap().as_str() {
                "L" => false,
                "R" => true,
                _ => panic!("Don't recognise")
            };

            instructions.push((clockwise, cap.name("number").unwrap().as_str().parse().unwrap()));
        }

        Map { 
            current_square: current_square, current_direction: Direction::Up, rows: rows, 
            row_count: row_count, col_count: col_count, instructions: instructions
        }
    }
}


impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        map.do_instructions();
        let score = map.score();
        format!("Score: {score}")
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_map_p1() {
        let output = DayTwentyTwo{}.part_one(&INPUT);
        assert_eq!(output, "Score: 6032")
    }
}
