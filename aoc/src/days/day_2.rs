use crate::problem::Problem;

pub struct DayTwo {}

fn comparison_score(opponent: u8, player: u8) -> u32 {
    // Rust gives remainder like C from % NOT modulus
    let comparison = (((player as isize - opponent as isize) % 3) + 3) % 3;
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
        _ => panic!("Unknown player type")
    } 
}

fn parse(input: &str) -> Vec<u32> {  
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
        let player = match opponent_player[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            &_ => panic!("Player not recognised")
        };
        let round_score: u32 = comparison_score(opponent, player) + player_score(player);
        
        rounds.push(round_score);
    }

    return rounds
}


impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let rounds: Vec<u32> = parse(input);
        let total: u32 = rounds.iter().sum();
        format!("Total score: {}", total)
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_scores() {
        let input = "A Y
        B X
        C Z";
        let result: Vec<u32> = parse(input);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 8);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 6);
    }

    #[test]
    fn sum_p1() {
        let input = "A Y
        B X
        C Z";
        let result = DayTwo{}.part_one(&input);
        assert_eq!(result, "Total score: 15");
    }
}