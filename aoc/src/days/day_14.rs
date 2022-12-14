use crate::Problem;
use std::collections::HashSet;

pub struct DayFourteen;

fn simulate_drop(coords: &HashSet<(u32, u32)>, max_y: u32, floor: bool) -> Option<(u32, u32)> {
    let mut coord = (500, 0);
    loop {
        let next_steps = [(coord.0, coord.1 + 1), (coord.0 - 1, coord.1 + 1), (coord.0 + 1, coord.1 + 1)];
        match next_steps.iter().find(|&&item| !coords.contains(&item)) {
            Some(x) => {coord = *x;},
            None => {return Some(coord)}, // No next step
        };
        if coord.1 > max_y {
            match floor {
                true => return Some(coord),
                false => return None,
            }
        }
    }
}

fn get_walls(input: &str) -> HashSet<(u32, u32)> {
    let mut walls = HashSet::new();

    for line in input.lines() {
        let corners: Vec<(u32, u32)> = line
            .split(" -> ")
            .map(|x| (x.split(",").collect::<Vec<&str>>()[0].parse().unwrap(), x.split(",").collect::<Vec<&str>>()[1].parse().unwrap()))
            .collect();
        for i in 0..corners.len()-1 {
            let from: (u32, u32) = corners[i];
            let to: (u32, u32) = corners[i+1];
            
            if from.0 == to.0 {
                if to.1 > from.1 {
                    for j in from.1..to.1+1 {
                        walls.insert((from.0, j));
                    }
                } else {
                    for j in to.1..from.1+1 {
                        walls.insert((from.0, j));
                    }
                }
            } else if from.1 == to.1 {
                if to.0 > from.0 {
                    for j in from.0..to.0+1 {
                        walls.insert((j, from.1));
                    }
                } else {
                    for j in to.0..from.0+1 {
                        walls.insert((j, from.1));
                    }
                }
            } else {
                panic!("From and to don't make sense");
            }
        }
    }
    
    walls
}


impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let mut coords = get_walls(&input);
        let max_y: u32 = coords.iter().map(|x| x.1).max().unwrap();

        let mut count = 0;
        loop {
            match simulate_drop(&coords, max_y, false) {
                Some(coord) => {coords.insert(coord); count += 1;},
                None => {break;},
            }
        }

        format!("Number until fall: {count}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut coords = get_walls(&input);
        let max_y: u32 = coords.iter().map(|x| x.1).max().unwrap();

        let mut count = 0;
        loop {
            let coord = simulate_drop(&coords, max_y, true).unwrap(); // Should never be None with floor
            coords.insert(coord);
            count += 1;
            if coord == (500, 0) {
                break;
            }
        }
        format!("Number until bottleneck: {count}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_sand_drop_p1() {
        let output = DayFourteen{}.part_one(&INPUT);
        assert_eq!(output, "Number until fall: 24")
    }

    #[test]
    fn test_sand_drop_p2() {
        let output = DayFourteen{}.part_two(&INPUT);
        assert_eq!(output, "Number until bottleneck: 93")
    }
}
