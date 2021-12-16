use std::fmt::Write;

struct BitReader {
    data: String,
    start: usize
}

impl BitReader {
    fn new() -> Self {
        BitReader {data: String::new(), start: 0}
    }

    fn read_bit(&mut self) -> Option<bool> {
        if self.start > self.data.len() {
            return None;
        }

        let bit = &self.data[self.start..self.start+1] == "1";
        self.start += 1;
        Some(bit)
    }

    fn read_i32(&mut self, len: usize) -> Option<i32> {
        if self.start + len > self.data.len() {
            return None;
        }

        let slice = &self.data[self.start..self.start+len];
        self.start += len;
        match i32::from_str_radix(slice, 2) {
            Err(_) => None,
            Ok(i) => Some(i)
        }
    }

    fn read_i64(&mut self, len: usize) -> Option<i64> {
        if self.start + len > self.data.len() {
            return None;
        }

        let slice = &self.data[self.start..self.start+len];
        self.start += len;
        match i64::from_str_radix(slice, 2) {
            Err(_) => None,
            Ok(i) => Some(i)
        }
    }
}

struct Packet {
    version: i32,
    type_id: i32,
    literal: i64,
    sub_packets: Vec<Packet>
}

impl Packet {

    const TYPE_SUM : i32 = 0;
    const TYPE_PRODUCT : i32 = 1;
    const TYPE_MIN : i32 = 2;
    const TYPE_MAX : i32 = 3;
    const TYPE_LITERAL : i32 = 4;
    const TYPE_GREATER_THAN : i32 = 5;
    const TYPE_LESS_THAN : i32 = 6;
    const TYPE_EQUAL_TO : i32 = 7;

    fn parse(bit_stream: &mut BitReader) -> Self {
        let mut packet = Packet{version: 0, type_id: 0, literal: 0, sub_packets: vec![]};

        packet.version = bit_stream.read_i32(3).unwrap();
        packet.type_id = bit_stream.read_i32(3).unwrap();

        if packet.type_id == Packet::TYPE_LITERAL {
            let mut read_literal = true;
            while read_literal {
                read_literal = bit_stream.read_bit().unwrap();
                packet.literal = (packet.literal << 4) | bit_stream.read_i64(4).unwrap();
            }
        } else {
            let length_type_id = bit_stream.read_bit().unwrap();

            if length_type_id {
                let packet_count = bit_stream.read_i32(11).unwrap();
                for _ in 0..packet_count {
                    packet.sub_packets.push(Packet::parse(bit_stream));
                }
            } else {
                let packet_end = bit_stream.start + bit_stream.read_i32(15).unwrap() as usize + 15;
                while bit_stream.start < packet_end {
                    packet.sub_packets.push(Packet::parse(bit_stream));
                }
            }
        }

        packet
    }

    fn value(&self) -> i64 {
        assert!(self.type_id >= 0 && self.type_id <= 7);

        match self.type_id {
            Packet::TYPE_SUM        => { self.sub_packets.iter().map(|p| p.value()).sum::<i64>() }
            Packet::TYPE_PRODUCT    => { self.sub_packets.iter().map(|p| p.value()).product::<i64>() }
            Packet::TYPE_MIN        => { self.sub_packets.iter().map(|p| p.value()).min().unwrap() }
            Packet::TYPE_MAX        => { self.sub_packets.iter().map(|p| p.value()).max().unwrap() }
            Packet::TYPE_LITERAL    => { self.literal }

            Packet::TYPE_GREATER_THAN => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].value() > self.sub_packets[1].value() {1} else {0}
            }

            Packet::TYPE_LESS_THAN => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].value() < self.sub_packets[1].value() {1} else {0}
            }

            Packet::TYPE_EQUAL_TO => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets[0].value() == self.sub_packets[1].value() {1} else {0}
            }

            _ => { 0}
        }
    }

    fn sum_versions(&self) -> i32 {
        let mut result = self.version;

        for p in &self.sub_packets {
            result += p.sum_versions();
        }

        result
    }
}

fn main() {
    let input_iter = include_str!("../../input.txt").trim_end().lines();

    // parse input data
    for line in input_iter {
        let mut binary = BitReader::new();
        for hex in line.chars().map(|c| c.to_digit(16).expect("Invalid hexadecimal")) {
            write!(binary.data, "{:04b}", hex).unwrap();
        }

        let packet = Packet::parse(&mut binary);

        // solve the problems
        println!("Part 1: sum of versions = {}", packet.sum_versions());
        println!("Part 2: value = {}", packet.value())
    }
}
