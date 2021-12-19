use std::fs::File;
use std::io::{BufRead, BufReader};

use bitvec::prelude::*;

#[allow(dead_code)]
pub fn solution() {
    let packet = load("input/day16.txt");

    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.value());
}

fn load(filename: &str) -> Packet {
    let f = File::open(filename).unwrap();
    let mut f = BufReader::new(f);

    // Read hex digits out of the file.
    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    Packet::parse(&s)
}

/// Parses a packet out of the given bits that starts at index i, returning the packet and the
/// next index.
fn parse_packet(bits: &BitVec<Msb0, usize>, i: usize) -> (Packet, usize) {
    // First three bits are the version
    let version = bits[i + 0..i + 3].load_be::<u8>();

    // Next three bits are the type id
    let type_id = bits[i+3..i+6].load_be::<u8>();

    if type_id == 4 {
        // Type 4 is a literal value
        let (num, index) = parse_packet_number(bits, i + 6);

        return (Packet::Literal { version, num }, index);
    }

    // Other types are operators on one or more sub-packets.
    // The next bit is the length_type_id, which determines the number of sub-packets
    let length_type_id = bits[i+6];

    let mut packets = Vec::new();
    let mut index;
    if length_type_id {
        // The next 11 bits are a number that represents the number of sub-packets immediately
        // contained by this packet.
        let length = bits[i + 7..i + 18].load_be::<u16>();

        index = i + 18;
        for _ in 0..length {
            let (packet, new_index) = parse_packet(bits, index);
            packets.push(packet);
            index = new_index;
        }
    } else {
        // The next 15 bits are a number that represents the total length in bits of
        // sub-packets contained by this packet.
        let length = bits[i + 7..i + 22].load_be::<u16>();

        index = i + 22;
        while index < i + 22 + length as usize {
            let (packet, new_index) = parse_packet(bits, index);
            packets.push(packet);
            index = new_index;
        }
    };

    (Packet::Operator { version, type_id, packets }, index)
}

/// Parses a number starting at the given index in the bits.
fn parse_packet_number(bits: &BitVec<Msb0, usize>, i: usize) -> (u64, usize) {
    let mut index = i;
    let mut num = 0;
    loop {
        // Numbers are 5 bit chunks, where a 1 in the first bit means there's more chunks.
        // The remaining 4 bits are part of the number.
        num = (num << 4) + bits[index + 1..index + 5].load_be::<u8>() as u64;
        index += 5;

        if !bits[index - 5] {
            break;
        }
    }

    (num, index)
}

#[derive(Debug)]
enum Packet {
    Literal{version: u8, num: u64},
    Operator{version: u8, type_id: u8, packets: Vec<Packet>},
}

impl Packet {
    /// Parses a packet out of the given hex string.  The packet may contain one or more sub-packets.
    fn parse(s: &str) -> Self {
        // Translate s from hex digits into bits.
        let bits: BitVec<Msb0, usize> = s.chars()
            .flat_map(|c| c.to_digit(16))
            .fold(BitVec::new(), |mut bits, num| {
                for i in 0..4 {
                    bits.push(num & (1 << (3 - i)) != 0);
                }
                bits
            });

        // Parse packets.
        parse_packet(&bits, 0).0
    }

    /// Sums the versions of this packet and it's sub-packets.
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, num: _ } => *version as u64,
            Packet::Operator { version, type_id: _, packets} => {
                *version as u64 + packets.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    /// Returns the value of this packet and its sub-packets.
    fn value(&self) -> u64 {
        match self {
            // Type 0: sum
            Packet::Operator { version: _, type_id, packets } if *type_id == 0 =>
                packets.iter().map(|packet| packet.value()).sum(),
            // Type 1: product
            Packet::Operator { version: _, type_id, packets } if *type_id == 1 =>
                packets.iter().map(|packet| packet.value()).product(),
            // Type 2: minimum
            Packet::Operator { version: _, type_id, packets } if *type_id == 2 =>
                packets.iter().map(|packet| packet.value()).min().unwrap(),
            // Type 3: maximum
            Packet::Operator { version: _, type_id, packets } if *type_id == 3 =>
                packets.iter().map(|packet| packet.value()).max().unwrap(),
            // Type 4: literal
            Packet::Literal { version: _, num } => *num,
            // Type 5: greater than.  1 if the first value is greater than the second, 0 otherwise.
            Packet::Operator { version: _, type_id, packets } if *type_id == 5 =>
                if packets[0].value() > packets[1].value() { 1 } else { 0 },
            // Type 6: less than.
            Packet::Operator { version: _, type_id, packets } if *type_id == 6 =>
                if packets[0].value() < packets[1].value() { 1 } else { 0 },
            // Type 7: equal to
            Packet::Operator { version: _, type_id, packets } if *type_id == 7 =>
                if packets[0].value() == packets[1].value() { 1 } else { 0 },
            _ => panic!("Invalid packet type: {:?}", self),
        }
    }
}

#[test]
fn version_sum_sample() {
    assert_eq!(6, Packet::parse("D2FE28").version_sum());
    assert_eq!(9, Packet::parse("38006F45291200").version_sum());
    assert_eq!(14, Packet::parse("EE00D40C823060").version_sum());
    assert_eq!(16, Packet::parse("8A004A801A8002F478").version_sum());
    assert_eq!(12, Packet::parse("620080001611562C8802118E34").version_sum());
    assert_eq!(23, Packet::parse("C0015000016115A2E0802F182340").version_sum());
    assert_eq!(31, Packet::parse("A0016C880162017C3686B18A3D4780").version_sum());
}

#[test]
fn value_sample() {
    assert_eq!(3, Packet::parse("C200B40A82").value());
    assert_eq!(54, Packet::parse("04005AC33890").value());
    assert_eq!(7, Packet::parse("880086C3E88112").value());
    assert_eq!(9, Packet::parse("CE00C43D881120").value());
    assert_eq!(1, Packet::parse("D8005AC2A8F0").value());
    assert_eq!(0, Packet::parse("F600BC2D8F").value());
    assert_eq!(0, Packet::parse("9C005AC2F8F0").value());
    assert_eq!(1, Packet::parse("9C0141080250320F1802104A08").value());
}
