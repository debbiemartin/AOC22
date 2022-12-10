use crate::problem::Problem;

pub struct DayTen {}

struct CathodeRay {
    sum: i32,
    cycle_count: u32,
    register: i32,
    render_buf: String
}

impl CathodeRay {
    pub fn print_display(&self) -> &String {
        return &self.render_buf
    }

    fn render(&mut self) {
        let pixel_position = self.cycle_count as i32 - 1;
        if (pixel_position % 40 - self.register).abs() <= 1 {
            self.render_buf.push_str("#");
        } else {
            self.render_buf.push_str(".");
        }
        
        if self.cycle_count % 40 == 0 {
            self.render_buf.push_str("\n");
        }
    }

    fn increment_cycle_count(&mut self, register: Option<i32>) {
        self.render();
        self.cycle_count += 1;
        if let Some(x) = register {
            self.register += x;
        }
        if self.cycle_count % 40 == 20 {
            self.sum += self.register * self.cycle_count as i32;
        }
    }

    pub fn process_signal(&mut self, input: &str) -> i32 {
        for line in input.lines() {
            if line == "noop" {
                self.increment_cycle_count(None);
            } else if line.starts_with("addx") {
                let register = line[5..].parse().unwrap();
                self.increment_cycle_count(None);
                self.increment_cycle_count(Some(register));
            } else {
                panic!("Can't parse line {line}");
            }
        }

        self.sum
    }

    pub fn new() -> CathodeRay {
        let render_buf = String::new();
        CathodeRay { sum:0, cycle_count:1, register:1, render_buf:render_buf }
    }
}


impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let mut cathode_ray = CathodeRay::new();
        let register_sum = cathode_ray.process_signal(&input);
        format!("Sum of registers: {register_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut cathode_ray = CathodeRay::new();
        let _ = cathode_ray.process_signal(&input);
        let display = cathode_ray.print_display();
        format!("Display:\n{display}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cathode_ray_p1() {
        let input="addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let output = DayTen{}.part_one(&input);
        assert_eq!(output, "Sum of registers: 13140")
    }

    #[test]
    fn test_cathode_ray_p2() {
        let input="addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let output = DayTen{}.part_two(&input);
        assert_eq!(output, "Display:
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
