use crate::Problem;
use std::collections::VecDeque;

pub struct DayTwenty;

struct Cipher {
    content: VecDeque<(i32, usize)>
}

impl Cipher {   
    fn decrypt(&mut self) {
        for i in 0..self.content.len() {
            let index = self.content.iter().position(|&r| r.1 == i).unwrap();
            let element = self.content.remove(index).unwrap();
            let new_index = (index as i32 + element.0).rem_euclid(self.content.len() as i32) as usize;
            self.content.insert(new_index, element)
        }
    }

    fn get_sum(&self) -> i32 {
        let zero_index = self.content.iter().position(|&r| r.0 == 0).unwrap();
        
        let mut sum = 0;
        for index in [1000, 2000, 3000] {
            let val = self.content.get((zero_index + index) % self.content.len()).unwrap().0;
            sum += val;
        }

        sum
    }

    pub fn new(input: &str) -> Cipher {
        let mut content = VecDeque::new();
        for (i, numstr) in input.lines().enumerate() {
            let num: i32 = numstr.parse().unwrap();
            content.push_back((num, i));
        }

        Cipher { content }
    }
}


impl Problem for DayTwenty {
    fn part_one(&self, input: &str) -> String {
        let mut cipher = Cipher::new(&input);
        cipher.decrypt();
        let sum = cipher.get_sum();
        format!("Sum: {sum}")
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_cipher_p1() {
        let output = DayTwenty{}.part_one(&INPUT);
        assert_eq!(output, "Sum: 3")
    }
}
