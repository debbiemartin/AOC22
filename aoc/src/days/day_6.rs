use crate::problem::Problem;
use std::collections::HashSet;

pub struct DaySix {}


struct SignalProcessor {
    marker_position: Option<usize>,
    marker_size: usize
}

impl SignalProcessor {
    pub fn new(packet: String, marker_size: usize) -> SignalProcessor {
        let mut i = 0;
        let mut marker_position = None;
        while i + marker_size < packet.len() {
            let string_slice: HashSet<char> = packet[i..i + marker_size].chars().collect();
            if string_slice.len() == marker_size {
                marker_position = Some(i);
                break;
            }
            i += 1
        }
        
        SignalProcessor { marker_position, marker_size }
    }

    pub fn get_marker(&self) -> usize {
        self.marker_position.unwrap() + self.marker_size
    }
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let signal_processor = SignalProcessor::new(String::from(input), 4);
        let packet_start = signal_processor.get_marker();
        format!("Packet start: {packet_start}")
    }

    fn part_two(&self, input: &str) -> String {
        let signal_processor = SignalProcessor::new(String::from(input), 14);
        let message_start = signal_processor.get_marker();
        format!("Start of message: {message_start}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_signal_packet_1() {
        let signal_processor = SignalProcessor::new(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 4);
        assert_eq!(signal_processor.get_marker(), 7);
    }

    #[test]
    fn test_process_signal_packet_2() {
        let signal_processor = SignalProcessor::new(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4);
        assert_eq!(signal_processor.get_marker(), 5);
    }

    #[test]
    fn test_process_signal_packet_3() {
        let signal_processor = SignalProcessor::new(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4);
        assert_eq!(signal_processor.get_marker(), 6);
    }

    #[test]
    fn test_process_signal_packet_4() {
        let signal_processor = SignalProcessor::new(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4);
        assert_eq!(signal_processor.get_marker(), 10);
    }

    #[test]
    fn test_process_signal_packet_5() {
        let signal_processor = SignalProcessor::new(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4);
        assert_eq!(signal_processor.get_marker(), 11);
    }

    #[test]
    fn test_process_signal_message_1() {
        let signal_processor = SignalProcessor::new(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14);
        assert_eq!(signal_processor.get_marker(), 19);
    }

    #[test]
    fn test_process_signal_message_2() {
        let signal_processor = SignalProcessor::new(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14);
        assert_eq!(signal_processor.get_marker(), 23);
    }

    #[test]
    fn test_process_signal_message_3() {
        let signal_processor = SignalProcessor::new(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 14);
        assert_eq!(signal_processor.get_marker(), 23);
    }

    #[test]
    fn test_process_signal_message_4() {
        let signal_processor = SignalProcessor::new(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 14);
        assert_eq!(signal_processor.get_marker(), 29);
    }

    #[test]
    fn test_process_signal_message_5() {
        let signal_processor = SignalProcessor::new(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 14);
        assert_eq!(signal_processor.get_marker(), 26);
    }
}