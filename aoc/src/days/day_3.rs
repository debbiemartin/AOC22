use crate::problem::Problem;
use std::collections::HashSet;

pub struct DayThree {}

fn convert_char_to_int(character: char) -> u32 {
    match character {
        'a'..='z' => character as u32 - ('a' as u32) + 1,
        'A'..='Z' => character as u32 - ('A' as u32) + 27,
        _ => panic!("Don't recognise character {character}")
    }
}

fn split_string_in_half(line: &str) -> (&str, &str) {
    let strlen = line.len();
    if strlen % 2 != 0 {
        panic!("String length {} can't be split in half", strlen)
    };
    line.split_at(strlen / 2)
}

fn get_doubles_line(line: &str) -> Vec<char> {
    let (split1, split2) = split_string_in_half(line.trim());

    let set1: HashSet<char> = split1.chars().collect();
    let set2: HashSet<char> = split2.chars().collect();

    set1.intersection(&set2).into_iter().map(|x| *x).collect()

}

fn get_doubles_input(input: &str) -> Vec<char> { 
    let mut doubles: Vec<char> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            continue
        }
        let doubles_line = get_doubles_line(line);

        for d in doubles_line {
            doubles.push(d)
        }
    }

    return doubles
}

fn get_priorities_input(input: &str) -> Vec<u32> {
    get_doubles_input(input).iter().map(|x| convert_char_to_int(*x)).collect()
}

fn get_badges_group(group: Vec<&str>) -> char {
    let group_size = group.len();
    if group_size != 3 {
        panic!("Got group of len {group_size}")
    };

    let set1: HashSet<char> = group[0].chars().collect();
    let set2: HashSet<char> = group[1].chars().collect();
    let set3: HashSet<char> = group[2].chars().collect();

    let badges: Vec<char> = set1.iter().filter(|k| set2.contains(k)).filter(|k| set3.contains(k)).map(|x| *x).collect();

    let badge_count = badges.len();
    if badge_count != 1 {
        panic!("found {badge_count} badges")
    };

    badges[0]
}

fn get_badges_input(input: &str) -> Vec<char> {
    let mut badges: Vec<char> = Vec::new();

    for group in input.split("\n").collect::<Vec<&str>>().chunks(3) {
        let badge = get_badges_group(group.iter().map(|x| x.trim()).collect());

        badges.push(badge)
    }
    
    badges
}


fn get_badge_priorities_input(input: &str) -> Vec<u32> {
    get_badges_input(input).iter().map(|x| convert_char_to_int(*x)).collect()
}


impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let priorities = get_priorities_input(input);
        let total: u32 = priorities.iter().sum();
        format!("Total score: {}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let badges = get_badge_priorities_input(input);
        let total: u32 = badges.iter().sum();
        format!("Total score: {}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_char_to_int () {
        assert_eq!(convert_char_to_int('a'), 1);
        assert_eq!(convert_char_to_int('A'), 27);
    }

    #[test]
    fn test_split_string_in_half () {
        let (str1, str2) = split_string_in_half("abcdef");
        assert_eq!(str1, "abc");
        assert_eq!(str2, "def");
    }

    #[test]
    fn test_get_doubles_line () {
        let doubles_line: Vec<char> = get_doubles_line("abcade");
        assert_eq!(doubles_line.len(), 1);
        assert_eq!(doubles_line[0], 'a');
    }

    #[test]
    fn test_get_doubles_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result: Vec<char> = get_doubles_input(&input);
        //assert_eq!(result.len(), 6);
        assert_eq!(result[0], 'p');
        assert_eq!(result[1], 'L');
        assert_eq!(result[2], 'P');
        assert_eq!(result[3], 'v');
        assert_eq!(result[4], 't');
        assert_eq!(result[5], 's');
    }

    #[test]
    fn test_get_priorities_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result: Vec<u32> = get_priorities_input(&input);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 16);
        assert_eq!(result[1], 38);
        assert_eq!(result[2], 42);
        assert_eq!(result[3], 22);
        assert_eq!(result[4], 20);
        assert_eq!(result[5], 19);
    }

    #[test]
    fn test_get_badges_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result: Vec<char> = get_badges_input(&input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 'r');
        assert_eq!(result[1], 'Z');
    }

    #[test]
    fn test_get_badge_priorities_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result: Vec<u32> = get_badge_priorities_input(&input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 18);
        assert_eq!(result[1], 52);
    }

    #[test]
    fn test_find_priorities_p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = DayThree{}.part_one(&input);
        assert_eq!(result, "Total score: 157");
    }

    #[test]
    fn test_find_badges_p2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = DayThree{}.part_two(&input);
        assert_eq!(result, "Total score: 70");
    }
}