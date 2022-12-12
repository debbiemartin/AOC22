use crate::problem::Problem;
use crate::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};

pub struct DayTwelve;

struct HeightMap {
    map: Graph,
    end: NodeIndex,
    single_start: NodeIndex,
    possible_starts: HashSet<NodeIndex>
}

impl HeightMap {
    pub fn shortest_path(&mut self) -> u32 {
        self.map.bfs(self.single_start, self.end).unwrap()
    }

    pub fn shortest_path_from_a(&mut self) -> u32 {
        let mut distances = Vec::new();
        for start in &self.possible_starts {
            if let Some(x) = self.map.bfs(*start, self.end) {
                distances.push(x);
            } 
        }

        *distances.iter().min().unwrap()
    }

    fn get_letter_value(letter: char) -> i32 {
        let value = match letter {
            'S' => 'a',
            'E' => 'z',
            _ => letter,
        };
        assert!('a' as i32 <= value as i32 && value as i32 <= 'z' as i32);
        value as i32
    }

    pub fn new(input: &str) -> HeightMap {
        let mut single_start = None;
        let mut end = None;
        let mut grid: HashMap<(i32, i32), (char, usize)> = HashMap::new();
        let mut graph = Graph::new();
        let mut possible_starts = HashSet::new();

        for (i, line) in input.lines().enumerate() {
            for (j, letter) in line.trim().chars().enumerate() {
                let node_index = graph.add_node();
                if letter == 'S' {
                    single_start = Some(node_index);
                    possible_starts.insert(node_index);
                } else if letter == 'E' {
                    end = Some(node_index);
                } else if letter == 'a' {
                    possible_starts.insert(node_index);
                }
                grid.insert((i as i32, j as i32), (letter, node_index));
            }
        }

        // Add edges to graph
        for (coord, (letter, index)) in &grid {
            for neighbour in [(coord.0-1, coord.1), (coord.0+1, coord.1), (coord.0, coord.1-1), (coord.0, coord.1+1)] {
                if grid.contains_key(&neighbour) {
                    let (neighbour_letter, neighbour_index) = grid.get(&neighbour).unwrap();
                    if HeightMap::get_letter_value(*neighbour_letter) - HeightMap::get_letter_value(*letter) <= 1 {
                        graph.add_edge(*index, *neighbour_index);
                    }  
                }
            }
        } 

        let single_start_val = single_start.unwrap();
        let end_val = end.unwrap();
        HeightMap { map:graph, single_start:single_start_val, end:end_val, possible_starts:possible_starts }
    }
}

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let mut map = HeightMap::new(input);
        let shortest_path = map.shortest_path();
        format!("Shortest path: {shortest_path}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = HeightMap::new(input);
        let shortest_path = map.shortest_path_from_a();
        format!("Shortest path: {shortest_path}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_height_map_p1() {
        let output = DayTwelve{}.part_one(&INPUT);
        assert_eq!(output, "Shortest path: 31")
    }

    #[test]
    fn test_height_map_p2() {
        let output = DayTwelve{}.part_two(&INPUT);
        assert_eq!(output, "Shortest path: 29")
    }
}
