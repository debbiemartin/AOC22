use crate::problem::Problem;

pub struct DayTwo {}

fn modulo(val: isize, modulus: isize) -> u8 {
    (((val % modulus) + modulus) % modulus) as u8
}
    

fn comparison_score(opponent: u8, player: u8) -> u32 {
    let comparison = modulo(player as isize - opponent as isize, 3);
    match comparison {
        0 => 3, // draw
        1 => 6, // win
        2 => 0, // loss 
        _ => panic!("Unknown round outcome: {}", comparison)
    }
}

fn player_score(player: u8) -> u32 {
    match player {
        1 => 1, 
        2 => 2,
        3 => 3,
        _ => panic!("Unknown player type: {}", player)
    } 
}

fn player_as_action(_opponent: u8, player: &str) -> u8 {
   match player {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => panic!("Opponent not recognised")
    }
}

fn player_as_result(opponent: u8, player: &str) -> u8 {
    match player {
        "X" => modulo(opponent as isize - 2, 3) + 1, // lose
        "Y" => opponent, // draw
        "Z" => modulo(opponent as isize, 3) + 1, // win
        &_ => panic!("Opponent not recognised")
    }
}

fn parse<F: Fn(u8, &str) -> u8>(input: &str, player_infer: F) -> Vec<u32> {  
    let mut rounds = Vec::new();
    for line in input.split("\n") {
        if line == "" {
            continue
        }
        let opponent_player: Vec<&str> = line.trim().split(" ").collect();
        let opponent = match opponent_player[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            &_ => panic!("Opponent not recognised")
        };
        let player = player_infer(opponent, opponent_player[1]);
        let round_score: u32 = comparison_score(opponent, player) + player_score(player);
        
        rounds.push(round_score);
    }

    return rounds
}


impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let rounds: Vec<u32> = parse(input, player_as_action);
        let total: u32 = rounds.iter().sum();
        format!("Total score: {}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let rounds: Vec<u32> = parse(input, player_as_result);
        let total: u32 = rounds.iter().sum();
        format!("Total score: {}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_scores_pt1() {
        let input = "A Y
        B X
        C Z";
        let result: Vec<u32> = parse(input, player_as_action);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 8);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 6);
    }

    #[test]
    fn find_scores_pt2() {
        let input = "A Y
        B X
        C Z";
        let result: Vec<u32> = parse(input, player_as_result);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 4);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 7);
    }

    #[test]
    fn sum_p1() {
        let input = "A Y
        B X
        C Z";
        let result = DayTwo{}.part_one(&input);
        assert_eq!(result, "Total score: 15");
    }

    #[test]
    fn sum_p2() {
        let input = "A Y
        B X
        C Z";
        let result = DayTwo{}.part_two(&input);
        assert_eq!(result, "Total score: 12");
    }
}