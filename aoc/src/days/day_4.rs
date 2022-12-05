use crate::problem::Problem;

pub struct DayFour {}


fn check_for_overlap_strict(start_a: i32, end_a: i32, start_b: i32, end_b: i32) -> i32 {
    if start_a <= start_b && end_a >= end_b {
        // A contains B
        return 1
    } else if start_a >= start_b && end_a <= end_b {
        // B contains A
        return 1
    } else {
        return 0
    }
}

fn check_for_overlap_lenient(start_a: i32, end_a: i32, start_b: i32, end_b: i32) -> i32 {
    if end_b < start_a {
        // B is before A - no overlap
        return 0
    } else if end_a < start_b {
        // A is before B - no overlap
        return 0
    } else {
        return 1
    }
}

fn parse_elf(elf: &str) -> (i32, i32) {
    let min_max: Vec<&str> = elf.split("-").collect();
    if min_max.len() != 2 {
        panic!{"Found line without 2 elves"}
    }

    (min_max[0].parse().unwrap(), min_max[1].parse().unwrap())
}

fn get_overlap_count<F: Fn(i32, i32, i32, i32) -> i32>(input: &str, overlap_check: F) -> i32 {
    let mut overlap_count = 0;
    for elf_pair in input.split("\n") {
        let elves: Vec<&str> = elf_pair.trim().split(",").collect();
        if elves.len() != 2 {
            panic!{"Found line without 2 elves"}
        }
        let (elf_a_start, elf_a_end) = parse_elf(elves[0]);
        let (elf_b_start, elf_b_end) = parse_elf(elves[1]);

        overlap_count += overlap_check(elf_a_start, elf_a_end, elf_b_start, elf_b_end);
    }
    
    overlap_count
}


impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let overlap_count = get_overlap_count(input, check_for_overlap_strict);
        format!("Overlap count: {}", overlap_count)
    }

    fn part_two(&self, input: &str) -> String {
        let overlap_count = get_overlap_count(input, check_for_overlap_lenient);
        format!("Overlap count: {}", overlap_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_overlap_strict() {
        assert_eq!(check_for_overlap_strict(1, 4, 2, 3), 1);
        assert_eq!(check_for_overlap_strict(2, 3, 1, 4), 1);
        assert_eq!(check_for_overlap_strict(1, 2, 1, 2), 1);
        assert_eq!(check_for_overlap_strict(1, 2, 3, 4), 0);
        assert_eq!(check_for_overlap_strict(3, 4, 1, 2), 0);
        assert_eq!(check_for_overlap_strict(1, 2, 1, 2), 1);
    }

    #[test]
    fn test_check_overlap_lenient() {
        assert_eq!(check_for_overlap_lenient(1, 4, 2, 3), 1);
        assert_eq!(check_for_overlap_lenient(2, 3, 1, 4), 1);
        assert_eq!(check_for_overlap_lenient(1, 2, 1, 2), 1);
        assert_eq!(check_for_overlap_lenient(1, 2, 3, 4), 0);
        assert_eq!(check_for_overlap_lenient(3, 4, 1, 2), 0);
        assert_eq!(check_for_overlap_lenient(1, 2, 2, 3), 1);
        assert_eq!(check_for_overlap_lenient(2, 3, 1, 2), 1);
        assert_eq!(check_for_overlap_lenient(1, 3, 2, 4), 1);
        assert_eq!(check_for_overlap_lenient(1, 2, 1, 2), 1);
    }

    #[test]
    fn test_parse_elf() {
        let (elf_a_start, elf_a_end) = parse_elf("1-2");
        assert_eq!(elf_a_start, 1);
        assert_eq!(elf_a_end, 2);
    }

    #[test]
    fn test_find_overlap_count_p1() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let result = DayFour{}.part_one(&input);
        assert_eq!(result, "Overlap count: 2");
    }

    #[test]
    fn test_find_overlap_count_p2() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let result = DayFour{}.part_two(&input);
        assert_eq!(result, "Overlap count: 4");
    }
}