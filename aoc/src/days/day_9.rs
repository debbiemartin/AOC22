use crate::problem::Problem;
use std::collections::HashSet;
use regex::Regex;


pub struct DayNine {}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Debug)]
struct Move {
    direction: Direction,
    count: u8
}

struct Rope {
    knots: Vec<(i32, i32)>,
    knot_count: usize, 
    tail_visited: HashSet<(i32, i32)>,
    head_moves: Vec<Move>
}

impl Rope {
    fn update_knot(&mut self, coord: (i32, i32), knot_index: usize) {
        self.knots[knot_index] = coord;
        if knot_index == self.knot_count - 1 {
            self.tail_visited.insert(coord);
        }
    }

    fn do_move(&mut self, direction: &Direction) {
        // move head
        let head = self.knots[0];
        self.knots[0] = match direction {
            Direction::Up => (head.0, head.1 + 1),
            Direction::Down => (head.0, head.1 - 1),
            Direction::Right => (head.0 + 1, head.1),
            Direction::Left => (head.0 - 1, head.1),
        };

        // move tailing knots
        for i in 1..self.knot_count {
            let knot = self.knots[i];
            let knot_ahead = self.knots[i-1]; // this will have already been moved due to order of iteration
    
            //move tail
            let x_diff: i32 = knot_ahead.0 - knot.0;
            let y_diff: i32 = knot_ahead.1 - knot.1;
            if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                // touching
                return
            }
    
            if x_diff.abs() > 2 || y_diff.abs() > 2 {
                panic!("Can't move knot - distance too large to traverse");
            }


            if y_diff.abs() == 2 && x_diff.abs() <= 1 {
                self.update_knot((knot_ahead.0, knot.1 + (y_diff/2)), i)
            } else if x_diff.abs() == 2 && y_diff.abs() <= 1 {
                self.update_knot((knot.0 + (x_diff/2), knot_ahead.1), i);
            } else if y_diff.abs() == 2 && x_diff.abs() == 2 {
                self.update_knot((knot.0 + (x_diff/2), knot.1 + (y_diff/2)), i);
            } else {
                panic!("Couldn't move knot")
            }
        }
    }

    pub fn do_moves(&mut self) {
        // Take a copy because the self.do_move mutable reference 
        // could in theory change self.head_moves
        let head_moves_cp = self.head_moves.clone();
        for head_move in head_moves_cp {
            for _ in 0..head_move.count {
                self.do_move(&head_move.direction);  
            }
        }
    }

    pub fn new(input: &str, knot_count:usize) -> Rope {
        let mut head_moves = Vec::new();
        for line in input.lines() {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(?P<direction>[RLUD]+)\s(?P<count>[0-9]+)\s*$").unwrap();
            }
            
            let cap = RE.captures(line.trim()).unwrap();
            let direction_str: &str = cap.name("direction").unwrap().as_str();
            let direction: Direction = match direction_str {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction {direction_str}")
            };
            let count: u8 = cap.name("count").unwrap().as_str().parse::<u8>().unwrap();

            head_moves.push(Move { direction, count })
        }
        let tail_visited = HashSet::from([(0,0)]);
        let mut knots = Vec::new();
        for _ in 0..knot_count {
            knots.push((0,0));
        }

        Rope { knots:knots, knot_count:knot_count, tail_visited:tail_visited, head_moves:head_moves }
    }
}


impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let mut rope = Rope::new(input, 2);
        rope.do_moves();
        let visited_count = rope.tail_visited.len();
        format!("Number of positions visited by tail: {visited_count}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut rope = Rope::new(input, 10);
        rope.do_moves();
        let visited_count = rope.tail_visited.len();
        format!("Number of positions visited by tail: {visited_count}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_init() {
        let input="R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let rope = Rope::new(&input, 2);

        assert_eq!(rope.knots[0], (0,0));
        assert_eq!(rope.knots[1], (0,0));
        assert_eq!(rope.head_moves.len(), 8);
        assert_eq!(rope.head_moves[0].direction, Direction::Right);
        assert_eq!(rope.head_moves[0].count, 4)
    }

    #[test]
    fn test_rope_single_move() {
        let input="R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let mut rope = Rope::new(&input, 2);
        rope.do_move(&Direction::Right);
        assert_eq!(rope.knots[0], (1,0));
        assert_eq!(rope.knots[1], (0,0));

        rope.do_move(&Direction::Right);
        assert_eq!(rope.knots[0], (2,0));
        assert_eq!(rope.knots[1], (1,0));
    }

    #[test]
    fn test_tail_visited_count_p1() {
        let input="R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let output = DayNine{}.part_one(&input);
        assert_eq!(output, "Number of positions visited by tail: 13")
    }

    #[test]
    fn test_tail_visited_count_p2() {
        let input="R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let output = DayNine{}.part_two(&input);
        assert_eq!(output, "Number of positions visited by tail: 1")
    }

    #[test]
    fn test_tail_visited_count_large_example_p2() {
        let input="R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let output = DayNine{}.part_two(&input);
        assert_eq!(output, "Number of positions visited by tail: 36")
    }
}
