use crate::Problem;
use std::collections::{HashSet, VecDeque};

pub struct DayEighteen;


struct Lava {
    droplets: HashSet<(i32,i32,i32)>,
    x_extent: (i32, i32),
    y_extent: (i32, i32),
    z_extent: (i32, i32),
}

impl Lava {
    fn _get_exterior_air(&self) -> HashSet<(i32, i32, i32)> {
        let mut queue = VecDeque::from([(self.x_extent.0, self.y_extent.0, self.z_extent.0)]);
        let mut exterior_air = HashSet::new();
        
        loop {
            let current = match queue.pop_front() {
                None => {break;},
                Some(x) => x,
            };

            let trials = [
                (current.0+1, current.1, current.2), 
                (current.0-1, current.1, current.2), 
                (current.0, current.1+1, current.2), 
                (current.0, current.1-1, current.2), 
                (current.0, current.1, current.2+1), 
                (current.0, current.1, current.2-1)
            ];

            for trial in trials {
                if trial.0 >= self.x_extent.0 && trial.0 <= self.x_extent.1 && 
                    trial.1 >= self.y_extent.0 && trial.1 <= self.y_extent.1 && 
                    trial.2 >= self.z_extent.0 && trial.2 <= self.z_extent.1 &&
                    ! self.droplets.contains(&trial) && ! exterior_air.contains(&trial) {
                    exterior_air.insert(trial);
                    queue.push_back(trial);                                  
                }
            }
        }

        exterior_air
    }

    pub fn get_interior_area(&self) -> usize {
        let exterior_air = self._get_exterior_air();
        let mut interior_air = HashSet::new();
        for i in self.x_extent.0..self.x_extent.1+1 {
            for j in self.y_extent.0..self.y_extent.1+1 {
                for k in self.z_extent.0..self.z_extent.1+1 {
                    let coord = (i,j,k);
                    if ! exterior_air.contains(&coord) && ! self.droplets.contains(&coord) {
                        interior_air.insert(coord);
                    }
                }
            }
        }
        self._get_surface_area(&interior_air)
    }

    fn _get_surface_area(&self, coords: &HashSet<(i32,i32,i32)>) -> usize {
        let coords_vec: Vec<(i32, i32, i32)> = coords.iter().map(|x| *x).collect();
        let mut touching_count = 0;
        let droplet_count = coords_vec.len();

        for i in 0..droplet_count {
            for j in 0..i {
                let diff = [
                    coords_vec[i].0 - coords_vec[j].0,
                    coords_vec[i].1 - coords_vec[j].1,
                    coords_vec[i].2 - coords_vec[j].2
                ];
                let distance: i32 = diff.iter().map(|x| x.abs()).sum();
                if distance == 1 {
                    touching_count += 1;
                }
            }
        } 

        return coords_vec.len() * 6 - touching_count * 2
    }

    pub fn get_surface_area(&self) -> usize {
        self._get_surface_area(&self.droplets)
    }

    fn new(input: &str) -> Lava {
        let mut droplets = HashSet::new();

        for line in input.lines() {
            let coords: Vec<i32> = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
            droplets.insert((coords[0], coords[1], coords[2]));
        }

        let x_extent = (droplets.iter().map(|x| x.0).min().unwrap() - 1, droplets.iter().map(|x| x.0).max().unwrap() + 1);
        let y_extent = (droplets.iter().map(|x| x.1).min().unwrap() - 1, droplets.iter().map(|x| x.1).max().unwrap() + 1);
        let z_extent = (droplets.iter().map(|x| x.2).min().unwrap() - 1, droplets.iter().map(|x| x.2).max().unwrap() + 1);

        Lava { droplets, x_extent, y_extent, z_extent }
    }
}

impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let lava = Lava::new(&input);
        let area = lava.get_surface_area();
        format!("Surface area: {area}")
    }

    fn part_two(&self, input: &str) -> String {
        let lava = Lava::new(&input);
        let exterior_area = lava.get_surface_area() - lava.get_interior_area();
        format!("Outer surface area: {exterior_area}")
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

    #[test]
    fn test_lava_exterior_area() {
        let output = DayEighteen{}.part_two(&INPUT);
        assert_eq!(output, "Outer surface area: 58")
    }
}
