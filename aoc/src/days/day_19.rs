use crate::Problem;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::max;

pub struct DayNineteen;

#[derive(Debug)]
struct Recipe {
    ore: u32, 
    clay: u32,
    obsidian: u32
}

impl Recipe {
    fn new(ore: u32, clay: u32, obsidian: u32) -> Recipe {
        Recipe { ore, clay, obsidian }
    }
}

struct Blueprint {
    blueprint_number: u8,
    ore_robot: Recipe,
    clay_robot: Recipe,
    obsidian_robot: Recipe,
    geode_robot: Recipe,
    memoize: HashMap<(u32, u32, u32, u32, u32, u32, u32, u32, u32), u32>,
    minutes: u32
}


impl Blueprint {
    fn _mins_to_get(existing_quantity: u32, needed_quantity: u32, robots: u32) -> u32 {
        if needed_quantity <= existing_quantity {
            return 1
        }

        return ((needed_quantity-existing_quantity) + robots - 1) / robots + 1;      
    }

    pub fn choose_route(&mut self, minute: u32, ore: u32, clay: u32, obsidian: u32, geodes: u32, ore_robots: u32, clay_robots: u32, obsidian_robots: u32, geode_robots: u32) -> u32 {       
        if minute == self.minutes {
            return geodes + geode_robots;
        }
        
        let args = (minute, ore, clay, obsidian, geodes, ore_robots, clay_robots, obsidian_robots, geode_robots);
        if self.memoize.contains_key(&args) {
            return *self.memoize.get(&args).unwrap();
        }
        
        if minute > self.minutes {
            panic!("Should be handled within individual steppers");
        } else if minute == self.minutes {
            return geodes 
        }

        let max_ore = *[self.ore_robot.ore, self.clay_robot.ore, self.obsidian_robot.ore, self.geode_robot.ore].iter().max().unwrap();

        let mut possible_routes = Vec::new();

        if ore >= self.geode_robot.ore && obsidian >= self.geode_robot.obsidian {
            possible_routes.push(
                self.choose_route(
                    minute+1, 
                    ore+ore_robots-self.geode_robot.ore, clay+clay_robots, obsidian+obsidian_robots-self.geode_robot.obsidian, geodes+geode_robots, 
                    ore_robots, clay_robots, obsidian_robots, geode_robots+1
                )
            );
        } else {
            // Next robot is ore robot
            if ore_robots < max_ore {
                let mins_for_ore = Blueprint::_mins_to_get(ore, self.ore_robot.ore, ore_robots);
 
                if minute + mins_for_ore < self.minutes {
                    possible_routes.push(
                        self.choose_route(
                            minute+mins_for_ore, 
                            ore+ore_robots*mins_for_ore-self.ore_robot.ore, clay+clay_robots*mins_for_ore, obsidian+obsidian_robots*mins_for_ore, geodes+geode_robots*mins_for_ore, 
                            ore_robots+1, clay_robots, obsidian_robots, geode_robots
                        )
                    ); 
                }
            }
            // Next robot is clay robot
            if obsidian_robots < self.geode_robot.obsidian && clay_robots < self.obsidian_robot.clay {
                let mins_for_ore = Blueprint::_mins_to_get(ore, self.clay_robot.ore, ore_robots);

                if minute + mins_for_ore < self.minutes {
                    possible_routes.push(
                        self.choose_route(
                            minute+mins_for_ore, 
                            ore+ore_robots*mins_for_ore-self.clay_robot.ore, clay+clay_robots*mins_for_ore, obsidian+obsidian_robots*mins_for_ore, geodes+geode_robots*mins_for_ore, 
                            ore_robots, clay_robots+1, obsidian_robots, geode_robots
                        )
                    );
                }
            } 
            // Next robot is obsidian robot
            if obsidian_robots < self.geode_robot.obsidian && clay_robots > 0 {
                let mins_for_ore = Blueprint::_mins_to_get(ore, self.obsidian_robot.ore, ore_robots);
                let mins_for_clay = Blueprint::_mins_to_get(clay, self.obsidian_robot.clay, clay_robots);
                let mins = max(mins_for_clay, mins_for_ore);

                if minute + mins < self.minutes {
                    possible_routes.push(
                        self.choose_route(
                            minute+mins, 
                            ore+ore_robots*mins-self.obsidian_robot.ore, clay+clay_robots*mins-self.obsidian_robot.clay, obsidian+obsidian_robots*mins, geodes+geode_robots*mins, 
                            ore_robots, clay_robots, obsidian_robots+1, geode_robots
                        )
                    );
                }
            }

            // Next robot is geode robot
            if obsidian_robots > 0 {
                let mins_for_ore = Blueprint::_mins_to_get(ore, self.geode_robot.ore, ore_robots);
                let mins_for_obsidian = Blueprint::_mins_to_get(obsidian, self.geode_robot.obsidian, obsidian_robots);
                let mins = max(mins_for_obsidian, mins_for_ore);
    
                if minute + mins < self.minutes {
                    possible_routes.push(
                        self.choose_route(
                            minute+mins, 
                            ore+ore_robots*mins-self.geode_robot.ore, clay+clay_robots*mins, obsidian+obsidian_robots*mins-self.geode_robot.obsidian, geodes+geode_robots*mins, 
                            ore_robots, clay_robots, obsidian_robots, geode_robots+1
                        )
                    );
                }
            }

            // No next robot
            possible_routes.push(
                self.choose_route(
                    self.minutes, 
                    ore+ore_robots*(self.minutes-minute), clay+clay_robots*(self.minutes-minute), obsidian+obsidian_robots*(self.minutes-minute), geodes+geode_robots*(self.minutes-minute), 
                    ore_robots, clay_robots, obsidian_robots, geode_robots
                )
            );
        }


        let max = *possible_routes.iter().max().unwrap();
        self.memoize.insert(args, max);
        max
    }

    pub fn quality_level(&mut self) -> u32 {
        let geodes = self.choose_route(1, 0, 0, 0, 0, 1, 0, 0, 0);
        geodes as u32 * self.blueprint_number as u32
    }

    pub fn max_geodes(&mut self) -> u32 {
        let geodes = self.choose_route(1, 0, 0, 0, 0, 1, 0, 0, 0);
        geodes as u32
    }

    pub fn new(input: &str, minutes: u32) -> Blueprint {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Blueprint (?P<blueprint_number>[0-9]+): Each ore robot costs (?P<ore_ore>[0-9]+) ore. Each clay robot costs (?P<clay_ore>[0-9]+) ore. Each obsidian robot costs (?P<obsidian_ore>[0-9]+) ore and (?P<obsidian_clay>[0-9]+) clay. Each geode robot costs (?P<geode_ore>[0-9]+) ore and (?P<geode_obsidian>[0-9]+) obsidian.$").unwrap();
        }
    
        let cap = RE.captures(input.trim()).unwrap();
        let blueprint_number: u8 = cap.name("blueprint_number").unwrap().as_str().parse::<u8>().unwrap();
        let ore_ore: u32 = cap.name("ore_ore").unwrap().as_str().parse::<u32>().unwrap();
        let ore_robot = Recipe::new(ore_ore, 0, 0);

        let clay_ore: u32 = cap.name("clay_ore").unwrap().as_str().parse::<u32>().unwrap();
        let clay_robot = Recipe::new(clay_ore, 0, 0);

        let obsidian_ore: u32 = cap.name("obsidian_ore").unwrap().as_str().parse::<u32>().unwrap();
        let obsidian_clay: u32 = cap.name("obsidian_clay").unwrap().as_str().parse::<u32>().unwrap();
        let obsidian_robot = Recipe::new(obsidian_ore, obsidian_clay, 0);

        let geode_ore: u32 = cap.name("geode_ore").unwrap().as_str().parse::<u32>().unwrap();
        let geode_obsidian: u32 = cap.name("geode_obsidian").unwrap().as_str().parse::<u32>().unwrap();
        let geode_robot = Recipe::new(geode_ore, 0, geode_obsidian);

        let memoize = HashMap::new();

        Blueprint { blueprint_number, ore_robot, clay_robot, obsidian_robot, geode_robot, memoize, minutes }
    }
}

struct Blueprints {
    blueprints: Vec<Blueprint>
}

impl Blueprints {
    fn get_quality_score(&mut self) -> u32 {
        let mut score: u32 = 0;
        for blueprint in &mut self.blueprints {
            score += blueprint.quality_level() as u32;
        } 

        score
    }

    fn get_score_product(&mut self) -> u32 {
        let mut score: u32 = 1;
        for i in 0..3 {
            let blueprint = &mut self.blueprints[i];
            score *= blueprint.max_geodes();
        }

        score
    }

    fn new(input: &str, minutes: u32) -> Blueprints {
        let mut blueprints = Vec::new();

        for blueprint in input.lines() {
            blueprints.push(Blueprint::new(blueprint, minutes))
        }

        Blueprints{ blueprints }
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let mut robots = Blueprints::new(&input, 24);
        let score = robots.get_quality_score();
        format!("Quality levels: {score}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut robots = Blueprints::new(&input, 32);
        let score = robots.get_score_product();
        format!("Quality levels: {score}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_geodes_p1() {
        let output = DayNineteen{}.part_one(&INPUT);
        assert_eq!(output, "Quality levels: 33")
    }

    #[test]
    fn test_geodes_p2_blueprint_1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

        let mut blueprint = Blueprint::new(&input, 32);
        assert_eq!(blueprint.max_geodes(), 56);
    }

    #[test]
    fn test_geodes_p2_blueprint_2() {
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        
        let mut blueprint = Blueprint::new(&input, 32);
        assert_eq!(blueprint.max_geodes(), 62);
    }
}
