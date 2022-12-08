use crate::problem::Problem;
use std::collections::HashMap;

pub struct DayEight {}


struct Forest {
    trees: HashMap<(usize, usize), u32>,
    row_size: usize, 
    col_size: usize
}

impl Forest {
    fn row(&self, row_num: usize) -> Vec<u32> {
        let mut row: Vec<u32> = Vec::new();
        for i in 0..self.col_size {
            row.push(*self.trees.get(&(row_num, i)).unwrap());
        }
        return row;
    }

    fn col(&self, column_num: usize) -> Vec<u32> {
        let mut column: Vec<u32> = Vec::new();
        for i in 0..self.row_size {
            column.push(*self.trees.get(&(i, column_num)).unwrap());
        }
        return column;
    }

    fn get_distance_to_tall_tree(&self, tree_section: Vec<u32>, reverse: bool) -> usize {
        if tree_section.len() == 1 {
            return 0;
        }

        let mut 
        tree_section_cp = tree_section.clone();
        if reverse {
            tree_section_cp.reverse()
        }

        for (i, tree) in tree_section_cp[1..].iter().enumerate() {
            if *tree >= tree_section_cp[0] {
                return i + 1;
            }     
        }
        return tree_section_cp.len() - 1
    }

    fn scenic_score(&self, row_num: usize, column_num: usize) -> usize {
        let row = self.row(row_num);
        let scenic_score_left = self.get_distance_to_tall_tree(row[0..column_num + 1].to_vec(), true);
        let scenic_score_right = self.get_distance_to_tall_tree(row[column_num..].to_vec(), false);

        let col = self.col(column_num);
        let scenic_score_above = self.get_distance_to_tall_tree(col[0..row_num + 1].to_vec(), true);
        let scenic_score_below =self.get_distance_to_tall_tree(col[row_num..].to_vec(), false);
        
        scenic_score_left * scenic_score_right * scenic_score_below * scenic_score_above
    }

    fn is_visible(&self, row_num: usize, column_num: usize) -> bool {
        if row_num == 0 || row_num == self.row_size - 1 || column_num == 0 || column_num == self.col_size - 1 {
            // Outer edge tree
            return true
        }
        
        let row: Vec<u32> = self.row(row_num);

        if row[column_num] > *row[0..column_num].iter().max().unwrap() {
            // Visible from the left
            return true
        }
        if row[column_num] > *row[column_num+1..].iter().max().unwrap() {
            // Visible from the right
            return true
        }

        let column: Vec<u32> = self.col(column_num);
        
        assert_eq!(column[row_num], row[column_num]); // Same tree
        if column[row_num] > *column[0..row_num].iter().max().unwrap() {
            // Visible from above
            return true
        }
        if column[row_num] > *column[row_num+1..].iter().max().unwrap() {
            // Visible from below
            return true
        }
        
        // Not visible
        false
    }

    pub fn num_visible(&self) -> u32 {
        self.trees.keys().fold(0, |sum, x| match self.is_visible(x.0, x.1) { true => sum + 1, false => sum} ) 
    }

    pub fn max_scenic_score(&self) -> usize {
        self.trees.keys().map(|k| self.scenic_score(k.0, k.1)).max().unwrap() 
    }

    pub fn new(input: &str) -> Forest {
        let mut trees: HashMap<(usize, usize), u32> = HashMap::new();
        let mut grid: Vec<Vec<u32>> = Vec::new();
        for line in input.lines() {
            grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
        }

        for (row, line) in grid.iter().enumerate() {
            for (col, tree) in line.iter().enumerate() {
                trees.insert((row, col), *tree);
            }
        }
        let row_size: usize = grid[0].len();
        let col_size: usize = grid.len();
        Forest { trees, row_size, col_size }
    }
}


impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let forest = Forest::new(input);
        let visible_count: u32 = forest.num_visible();
        format!("Visible trees: {visible_count}")
    }

    fn part_two(&self, input: &str) -> String {
        let forest = Forest::new(input);
        let max_scenic: usize = forest.max_scenic_score();
        format!("Max scenic score: {max_scenic}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let input="30373
25512
65332
33549
35390";
        let forest = Forest::new(&input);

        // Outer trees
        assert_eq!(forest.is_visible(0, 0), true);
        assert_eq!(forest.is_visible(0, 4), true);
        assert_eq!(forest.is_visible(3, 0), true);
        assert_eq!(forest.is_visible(3, 4), true);

        // Inner trees
        assert_eq!(forest.is_visible(1, 1), true);
        assert_eq!(forest.is_visible(1, 2), true);
        assert_eq!(forest.is_visible(1, 3), false);
        assert_eq!(forest.is_visible(3, 2), true);
        assert_eq!(forest.is_visible(2, 1), true);
        assert_eq!(forest.is_visible(2, 2), false);
        assert_eq!(forest.is_visible(2, 3), true);
    }

    #[test]
    fn test_scenic_score() {
        let input="30373
25512
65332
33549
35390";
        let forest = Forest::new(&input);

        // Outer trees
        assert_eq!(forest.scenic_score(1, 2), 4);
        assert_eq!(forest.scenic_score(3, 2), 8);
    }

    #[test]
    fn test_forest_p1() {
        let input="30373
25512
65332
33549
35390";
        assert_eq!(DayEight{}.part_one(&input), "Visible trees: 21");
    }

    #[test]
    fn test_forest_p2() {
        let input="30373
25512
65332
33549
35390";

    assert_eq!(DayEight{}.part_two(&input), "Max scenic score: 8");
    }
}
