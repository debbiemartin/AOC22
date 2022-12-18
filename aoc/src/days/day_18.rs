use crate::Problem;

pub struct DayEighteen;


struct Lava {
    droplets: Vec<(i32,i32,i32)>
}

impl Lava {
    fn get_surface_area(&self) -> usize {
        let mut touching_count = 0;
        let droplet_count = self.droplets.len();

        for i in 0..droplet_count {
            for j in 0..i {
                let diff = [
                    self.droplets[i].0 - self.droplets[j].0,
                    self.droplets[i].1 - self.droplets[j].1,
                    self.droplets[i].2 - self.droplets[j].2
                ];
                let distance: i32 = diff.iter().map(|x| x.abs()).sum();
                if distance == 1 {
                    touching_count += 1;
                }
            }
        } 

        return self.droplets.len() * 6 - touching_count * 2
    }

    fn new(input: &str) -> Lava {
        let mut droplets = Vec::new();

        for line in input.lines() {
            let coords: Vec<i32> = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
            droplets.push((coords[0], coords[1], coords[2]));
        }

        Lava { droplets }
    }
}

impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let lava = Lava::new(&input);
        let area = lava.get_surface_area();
        format!("Surface area: {area}")
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_lava_simple() {
        let input = "1,1,1
2,1,1";
        let output = DayEighteen{}.part_one(&input);
        assert_eq!(output, "Surface area: 10")
    }

    #[test]
    fn test_lava_complex() {
        let output = DayEighteen{}.part_one(&INPUT);
        assert_eq!(output, "Surface area: 64")
    }
}
