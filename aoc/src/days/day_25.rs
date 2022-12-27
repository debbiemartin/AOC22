use crate::Problem;
use std::collections::HashMap;

pub struct DayTwentyFive;

struct SnafuDecoder {
    snafu: HashMap<String, i64>
}

impl SnafuDecoder {
    fn sum(&self) -> String {
        let sum = self.snafu.values().sum();

        SnafuDecoder::encode(sum)
    }
    
    fn encode(snafu: i64) -> String {
        let mut val = snafu.clone();
        let mut str = String::new();

        loop {
            let digit = match (val + 2) % 5 {
                4 => "2",
                3 => "1",
                2 => "0",
                1 => "-",
                0 => "=",
                _ => panic!("Don't expect val"),
            };
            str = format!("{digit}{str}");
            val = (val + 2) / 5;
            if val == 0 {
                break;
            }
        }
        
        return str
    }

    fn decode(snafu: &str) -> i64 {
        let mut val: i64 = 0;
        for (i, character) in snafu.chars().enumerate() {
            let multiplier: i64 = (5 as i64).pow(snafu.len() as u32 - i as u32 - 1);
            val += match character {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Don't recognise {character}"),
            } * multiplier;
        }

        return val;
    }

    fn new(input: &str) -> SnafuDecoder {
        let mut snafu = HashMap::new();
        for line in input.lines() {
            let val = SnafuDecoder::decode(line.trim());

            snafu.insert(String::from(line.trim()), val);
        }

        SnafuDecoder { snafu }
    }
}


impl Problem for DayTwentyFive {
    fn part_one(&self, input: &str) -> String {
        let snafu_decoder = SnafuDecoder::new(&input);
        let sum = snafu_decoder.sum();
        format!("Snafu sum: {sum}")
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122";

    #[test]
    fn test_snafu_decodes() {
        assert_eq!(SnafuDecoder::decode("1"), 1);
        assert_eq!(SnafuDecoder::decode("2"), 2);
        assert_eq!(SnafuDecoder::decode("1="), 3);
        assert_eq!(SnafuDecoder::decode("1-"), 4);
        assert_eq!(SnafuDecoder::decode("10"), 5);
        assert_eq!(SnafuDecoder::decode("11"), 6);
        assert_eq!(SnafuDecoder::decode("12"), 7);
        assert_eq!(SnafuDecoder::decode("2="), 8);
        assert_eq!(SnafuDecoder::decode("2-"), 9);
        assert_eq!(SnafuDecoder::decode("20"), 10);
        assert_eq!(SnafuDecoder::decode("1=0"), 15);
        assert_eq!(SnafuDecoder::decode("1-0"), 20);
        assert_eq!(SnafuDecoder::decode("1=11-2"), 2022);
        assert_eq!(SnafuDecoder::decode("1-0---0"), 12345);
        assert_eq!(SnafuDecoder::decode("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_snafu_encodes() {
        assert_eq!(SnafuDecoder::encode(1), "1");
        assert_eq!(SnafuDecoder::encode(2), "2");
        assert_eq!(SnafuDecoder::encode(3), "1=");
        assert_eq!(SnafuDecoder::encode(4), "1-");
        assert_eq!(SnafuDecoder::encode(5), "10");
        assert_eq!(SnafuDecoder::encode(6), "11");
        assert_eq!(SnafuDecoder::encode(7), "12");
        assert_eq!(SnafuDecoder::encode(8), "2=");
        assert_eq!(SnafuDecoder::encode(9), "2-");
        assert_eq!(SnafuDecoder::encode(10), "20");
        assert_eq!(SnafuDecoder::encode(15), "1=0");
        assert_eq!(SnafuDecoder::encode(20), "1-0");
        assert_eq!(SnafuDecoder::encode(2022), "1=11-2");
        assert_eq!(SnafuDecoder::encode(12345), "1-0---0");
        assert_eq!(SnafuDecoder::encode(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_map_p1() {
        let output = DayTwentyFive{}.part_one(&INPUT);
        assert_eq!(output, "Snafu sum: 2=-1=0")
    }
}
