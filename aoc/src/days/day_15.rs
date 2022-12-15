use crate::Problem;
use regex::Regex;
use std::cmp::{min, max};

pub struct DayFifteen;

struct Sensor {
    sensor: (i32, i32),
    nearest_beacon: (i32, i32),
    distance: u32
}

impl Sensor {
    fn get_x_not_possible_range(&self, y: i32, extent: Option<u32>) -> Option<(i32, i32)> {
        let y_diff = (y - self.sensor.1).abs() as u32; 
        if y_diff > self.distance {
            return None;
        }
        
        let x_range = self.distance - y_diff;
        let mut not_possible = ((self.sensor.0 - x_range as i32), (self.sensor.0 + x_range as i32));
        if self.nearest_beacon.1 == y {
            if self.nearest_beacon.0 == not_possible.0 {
                not_possible = (not_possible.0 + 1, not_possible.1);
            } else if self.nearest_beacon.0 == not_possible.1 {
                not_possible = (not_possible.0, not_possible.1 - 1);
            } else {
                dbg!(&self.nearest_beacon);
                dbg!(&not_possible);
                dbg!(&self.sensor);
                panic!("Beacon should be on outer edge");
            }
        }

        if let Some(x) = extent {
            not_possible = (max(0, not_possible.0), min(x as i32, not_possible.1));
        }

        Some(not_possible)
    }

    fn new(sensor: (i32, i32), nearest_beacon: (i32, i32)) -> Sensor {
        let distance = (sensor.0-nearest_beacon.0).abs() as u32 + (sensor.1-nearest_beacon.1).abs() as u32;
        
        Sensor { sensor, nearest_beacon, distance }
    }
}

struct Map {
    sensors: Vec<Sensor>,
    line_to_search: i32, 
    extent: u32
}

impl Map {
    fn merge_ranges(&self, ranges: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        ranges.sort_by(|a, b| (a.0).cmp(&b.0));

        let mut merged = Vec::from([ranges[0]]);
        for (c, d) in &ranges[1..] {
            let (a, b) = merged.pop().unwrap(); 
            if *c <= b + 1 {
                merged.push((a, max(b, *d)));
            } else {
                // No overlap
                merged.push((a, b));
                merged.push((*c, *d));
            }
        }
        
        merged
    }

    fn get_merged_ranges(&self, y: i32, extent: Option<u32>) -> Vec<(i32, i32)> {
        let mut not_possible_ranges: Vec<(i32, i32)> = Vec::new();
        for sensor in &self.sensors {
            match sensor.get_x_not_possible_range(y, extent) {
                Some(x) => {not_possible_ranges.push(x);}, //@@@ this isn't taking the extent into account
                None => continue,
            }
        }

        self.merge_ranges(&mut not_possible_ranges)
    }

    fn count_beacons_not_possible(&self) -> u32 {
        let merged_ranges = self.get_merged_ranges(self.line_to_search, None);

        let mut total = 0;
        for (range_from, range_to) in merged_ranges {
            total += (range_to-range_from + 1) as u32;
        }
        total
    }

    fn not_a_beacon_or_sensor(&mut self, coord: (i32, i32)) -> bool {
        ! self.sensors.iter().any(|x| x.sensor == coord || x.nearest_beacon == coord)
    }


    fn get_distress_beacon(&mut self) -> (i32, i32) {
        for y in 0..self.extent + 1 {
            let not_poss = self.get_merged_ranges(y as i32, Some(self.extent));
            let poss_coord = match not_poss.len() {
                1 => {continue;},
                2 => (not_poss[0].1 + 1, y as i32),
                _ => {panic!("Shouldn't have 3+ ranges");},
            };
            if self.not_a_beacon_or_sensor(poss_coord) {
                return poss_coord;
            }
        }
        panic!("Didn't find beacon");
    }

    fn new(input: &str) -> Map {
        let mut sensors = Vec::new();
        let mut lines = input.lines();
        let line_to_search = lines.next().unwrap().parse().unwrap();
        let extent = lines.next().unwrap().parse().unwrap();

        for line in lines {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^Sensor\sat\sx=(?P<x_sensor>[\-0-9]+),\sy=(?P<y_sensor>[\-0-9]+):\sclosest beacon\sis\sat\sx=(?P<x_beacon>[\-0-9]+),\sy=(?P<y_beacon>[\-0-9]+)$").unwrap();
            }
        
            let cap = RE.captures(line.trim()).unwrap();
            let x_sensor: i32 = cap.name("x_sensor").unwrap().as_str().parse::<i32>().unwrap();
            let y_sensor: i32 = cap.name("y_sensor").unwrap().as_str().parse::<i32>().unwrap();
            let x_beacon: i32 = cap.name("x_beacon").unwrap().as_str().parse::<i32>().unwrap();
            let y_beacon: i32 = cap.name("y_beacon").unwrap().as_str().parse::<i32>().unwrap();

            let sensor = Sensor::new((x_sensor, y_sensor), (x_beacon, y_beacon));
            sensors.push(sensor);
        }

        Map { sensors, line_to_search, extent }
    }
}



impl Problem for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let map = Map::new(&input);
        let count = map.count_beacons_not_possible();
        format!("Not possible beacon locations: {count}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = Map::new(&input);
        let distress_beacon = map.get_distress_beacon();
        dbg!(&distress_beacon);
        let tuning_frequency: u64 = distress_beacon.0 as u64 * 4000000 + distress_beacon.1 as u64; 
        format!("Tuning frequency of distress signal: {tuning_frequency}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "10
20
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_beacons_p1() {
        let output = DayFifteen{}.part_one(&INPUT);
        assert_eq!(output, "Not possible beacon locations: 26")
    }

    #[test]
    fn test_beacons_p2() {
        let output = DayFifteen{}.part_two(&INPUT);
        assert_eq!(output, "Tuning frequency of distress signal: 56000011")
    }

}
