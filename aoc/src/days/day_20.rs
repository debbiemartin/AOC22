use crate::Problem;
use std::collections::VecDeque;

pub struct DayTwenty;

struct Cipher {
    content: VecDeque<(i64, usize)>
}

impl Cipher {   
    fn decrypt(&mut self, run_count: u32) {
        for _ in 0..run_count {
            for i in 0..self.content.len() {
                let index = self.content.iter().position(|&r| r.1 == i).unwrap();
                let element = self.content.remove(index).unwrap();
                let new_index = (index as i64 + element.0).rem_euclid(self.content.len() as i64) as usize;
                self.content.insert(new_index, element)
            }
        }
    }

    fn get_sum(&self) -> i64 {
        let zero_index = self.content.iter().position(|&r| r.0 == 0).unwrap();
        
        let mut sum = 0;
        for index in [1000, 2000, 3000] {
            sum += self.content.get((zero_index + index) % self.content.len()).unwrap().0;
        }

        sum as i64
    }

    pub fn new(input: &str, decryption_key: i64) -> Cipher {
        let mut content = VecDeque::new();
        for (i, numstr) in input.lines().enumerate() {
            let num: i64 = numstr.parse().unwrap();
            content.push_back(((num * decryption_key), i));
        }

        Cipher { content }
    }
}


impl Problem for DayTwenty {
    fn part_one(&self, input: &str) -> String {
        let mut cipher = Cipher::new(&input, 1);
        cipher.decrypt(1);
        let sum = cipher.get_sum();
        format!("Sum: {sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut cipher = Cipher::new(&input, 811589153);
        cipher.decrypt(10);
        let sum = cipher.get_sum();
        format!("Sum: {sum}")
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

    #[test]
    fn test_cipher_p2() {
        let output = DayTwenty{}.part_two(&INPUT);
        assert_eq!(output, "Sum: 1623178306")
    }
}
