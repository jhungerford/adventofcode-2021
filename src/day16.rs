use std::fs::File;
use std::io::{BufRead, BufReader};

use bitvec::prelude::*;

#[allow(dead_code)]
pub fn solution() {
    let transmission = load("input/day16.txt");

    println!("Part 1: {}", version_sum(&transmission));
}

fn load(filename: &str) -> String {
    let f = File::open(filename).unwrap();
    let mut f = BufReader::new(f);

    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    s
}

/// Decodes the given hexadecimal transmission, returning the sum of the version numbers
/// in all of the packets.
fn version_sum(s: &str) -> u32 {
    // Parse s from hex digits into bits.
    // let bits: BitVec = s.chars()
    let bits: BitVec<Msb0, usize> = s.chars()
        .flat_map(|c| c.to_digit(16))
        .fold(BitVec::new(), |mut bits, num| {
            for i in 0..4 {
                bits.push(num & (1 << (3 - i)) != 0);
            }
            bits
        });

    println!("s: {}, bits: {:?}", s, &bits);

    // Parse packets.
    let (packet, _) = parse_packet(&bits, 0);

    packet.version_sum()
}

/// Parses a packet out of the given bits that starts at index i, returning the packet and the
/// next index.
fn parse_packet(bits: &BitVec<Msb0, usize>, i: usize) -> (Packet, usize) {
    // First three bits are the version
    let version = bits[i + 0..i + 3].load::<u8>();

    // Next three bits are the type id
    let type_id = bits[i+3..i+6].load::<u8>();

    println!("Version: {}, type_id: {}", version, type_id);

    if type_id == 4 {
        // Type 4 is a literal value
        let (num, index) = parse_packet_number(bits, i + 6);
        println!("Literal - num: {}, index: {}", num, index);

        return (Packet::Literal { version, type_id, num }, index);
    }

    // Other types are operators on one or more sub-packets.
    // The next bit is the length_type_id, which determines the number of sub-packets
    let length_type_id = bits[i+6];

    println!("Length type id: {}", length_type_id);

    let mut length;
    let mut packets = Vec::new();
    let mut index;
    if length_type_id {
        // The next 11 bits are a number that represents the number of sub-packets immediately
        // contained by this packet.
        length = bits[i + 7..i + 18].load::<u16>();
        println!("Sub-packets - Length: {}", length);

        index = i + 18;
        for _ in 0..length {
            let (packet, new_index) = parse_packet(bits, index);
            packets.push(packet);
            index = new_index;
        }
    } else {
        // The next 15 bits are a number that represents the total length in bits of
        // sub-packets contained by this packet.
        length = bits[i + 7..i + 22].load::<u16>();
        println!("Bits Length: {} - {:?}", length, &bits[i+7..i+22]);

        index = i + 22;
        while index < i + 22 + length as usize {
            println!("Index: {}, limit: {}", index, i + 22 + length as usize);
            let (packet, new_index) = parse_packet(bits, index);
            packets.push(packet);
            index = new_index;
        }

        println!("Index: {}, limit: {}", index, i + 22 + length as usize);
    };

    (Packet::Operator { version, type_id, length_type_id, length, packets }, index)
}

/// Parses a number starting at the given index in the bits.  Numbers are made up of 5-bit chunks,
/// where 1 in the first bit means there are additional chunks.
fn parse_packet_number(bits: &BitVec<Msb0, usize>, i: usize) -> (u32, usize) {
    let mut index = i;

    let mut num = 0;
    loop {
        println!("Section: {:?}", &bits[index + 1..index + 5].load::<u8>());

        num = (num << 4) + bits[index + 1..index + 5].load::<u8>() as u32;
        index += 5;

        if !bits[index - 5] {
            break;
        }
    }

    println!("Packet #: {}", num);

    (num, index)
}

enum Packet {
    Literal{version: u8, type_id: u8, num: u32},
    Operator{version: u8, type_id: u8, length_type_id: bool, length: u16, packets: Vec<Packet>},
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            &Packet::Literal {version, type_id: _, num: _} => version as u32,
            Packet::Operator {version, type_id: _, length_type_id: _, length: _, packets} => {
                let packet_sum = packets.iter().map(|p| p.version_sum()).sum::<u32>();
                *version as u32 + packet_sum
            }
        }
    }
}

#[test]
fn version_sum_sample() {
    assert_eq!(6, version_sum("D2FE28"));
    assert_eq!(9, version_sum("38006F45291200"));
    assert_eq!(14, version_sum("EE00D40C823060"));
    assert_eq!(16, version_sum("8A004A801A8002F478"));
    assert_eq!(12, version_sum("620080001611562C8802118E34"));
    assert_eq!(23, version_sum("C0015000016115A2E0802F182340"));
    assert_eq!(31, version_sum("A0016C880162017C3686B18A3D4780"));
}