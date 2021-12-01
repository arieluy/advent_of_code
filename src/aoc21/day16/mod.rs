use std::fs;
use to_binary::BinaryString;

struct State {
    sum: usize
}

struct Return {
    end_index: usize,
    value: usize
}

impl State {
    // returns end index of packet
    fn parse_packet(&mut self, packet: &str) -> usize {
        let version = usize::from_str_radix(&packet[0..3], 2).unwrap();
        let p_type = usize::from_str_radix(&packet[3..6], 2).unwrap();
        self.sum += version;
        match p_type {
            4 => {
                // literal
                let mut pad_index = 6;
                while &packet[pad_index..pad_index+1] == "1" {
                    pad_index += 5;
                }
                return pad_index + 5;
            }, 
            _ => {
                // operator
                match &packet[6..7] {
                    "0" => {
                        // next 15 bits are total length of subpackets
                        let subpackets_len = usize::from_str_radix(&packet[7..22], 2).unwrap();
                        let subpackets_end_index = 22 + subpackets_len;
                        let mut curr_index = 22 + self.parse_packet(&packet[22..]);
                        while curr_index < subpackets_end_index {
                            curr_index += self.parse_packet(&packet[curr_index..]);
                        }
                        assert_eq!(curr_index, subpackets_end_index);
                        return subpackets_end_index;
                    },
                    _ => {
                        // next 11 bits are number of subpackets
                        let subpackets_num = u32::from_str_radix(&packet[7..18], 2).unwrap();
                        let mut curr_index = 18;
                        for _ in 0..subpackets_num {
                            curr_index += self.parse_packet(&packet[curr_index..]);
                        }
                        return curr_index;
                    }
                }
            }
        }
    }

    // returns end index of packet
    fn parse_packet_calc(&mut self, packet: &str) -> Return {
        let p_type = usize::from_str_radix(&packet[3..6], 2).unwrap();
        match p_type {
            4 => {
                // literal
                let mut pad_index = 6;
                let mut value = 0;
                while &packet[pad_index..pad_index+1] == "1" {
                    value <<= 4;
                    value += usize::from_str_radix(&packet[pad_index+1..pad_index+5], 2).unwrap();
                    pad_index += 5;
                }
                value <<= 4;
                value += usize::from_str_radix(&packet[pad_index+1..pad_index+5], 2).unwrap();
                return Return{ end_index: pad_index + 5, value: value };
            }, 
            _ => {
                let mut subpackets_end_index = 0;
                let mut subpacket_values = Vec::new();
                // operator
                match &packet[6..7] {
                    "0" => {
                        // next 15 bits are total length of subpackets
                        let subpackets_len = usize::from_str_radix(&packet[7..22], 2).unwrap();
                        subpackets_end_index = 22 + subpackets_len;

                        let mut subp = self.parse_packet_calc(&packet[22..]);
                        let mut curr_index = 22 + subp.end_index;
                        subpacket_values.push(subp.value);

                        while curr_index < subpackets_end_index {
                            subp = self.parse_packet_calc(&packet[curr_index..]);
                            curr_index += subp.end_index;
                            subpacket_values.push(subp.value);
                        }
                        assert_eq!(curr_index, subpackets_end_index);
                    },
                    _ => {
                        // next 11 bits are number of subpackets
                        let subpackets_num = u32::from_str_radix(&packet[7..18], 2).unwrap();
                        let mut curr_index = 18;

                        for _ in 0..subpackets_num {
                            let subp = self.parse_packet_calc(&packet[curr_index..]);
                            curr_index += subp.end_index;
                            subpacket_values.push(subp.value);
                        }
                        subpackets_end_index = curr_index;
                    }
                }

                let value = match p_type {
                    0 => subpacket_values.iter().sum::<usize>(),
                    1 => subpacket_values.iter().product(),
                    2 => *subpacket_values.iter().min().unwrap(),
                    3 => *subpacket_values.iter().max().unwrap(),
                    5 => if subpacket_values[0] > subpacket_values[1] { 1 } else { 0 },
                    6 => if subpacket_values[0] < subpacket_values[1] { 1 } else { 0 },
                    _ => if subpacket_values[0] == subpacket_values[1] { 1 } else { 0 },
                };

                return Return { end_index: subpackets_end_index, value: value };
            }
        }
    }
}

pub fn day16_part1() {
    let file_string = fs::read_to_string("day16/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let binary = BinaryString::from_hex(lines.next().unwrap()).unwrap();

    let mut tmp = State { sum: 0 };
    
    tmp.parse_packet(&binary.0);
    println!("{}", tmp.sum);
} 


pub fn day16_part2() {
    let file_string = fs::read_to_string("day16/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let binary = BinaryString::from_hex(lines.next().unwrap()).unwrap();

    let mut tmp = State { sum: 0 };
    
    let ret = tmp.parse_packet_calc(&binary.0);
    println!("{}", ret.value);
}