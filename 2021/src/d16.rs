use crate::input;

use bitvec::prelude::*;
use bitvec::mem::BitMemory;
use itertools::Itertools;

use std::fmt;

const DAY: usize = 16;

pub fn run() -> String {
    let input = input(DAY);
    let mut output = String::new();
    let time = std::time::Instant::now();
    output += &format!("d{:02}p1: {} in {:?}\n", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    output += &format!("d{:02}p2: {} in {:?}\n", DAY, p2(&input), time.elapsed());
    output
}

#[derive(Debug, Clone)]
struct Input {
    bytes: Vec<u8>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let nibbles = s
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect::<Vec<u8>>();

        let bytes = nibbles
            .chunks(2)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0u8, |byte, (i, nibble)| {
                        let nibble_idx = 1 - i;
                        byte | nibble << (nibble_idx * 4)
                    })
            })
            .collect::<Vec<u8>>();

        Self { bytes }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    typ: Type,
}

struct Parser {
    bits: BitVec::<Msb0, u8>,
    index: usize,
}

impl From<&Input> for Parser {
    fn from(input: &Input) -> Self {
        let bits = BitVec::<Msb0, _>::from_slice(&input.bytes).unwrap();

        Self { bits, index: 0 }
    }
}

impl Parser {
    fn take<I>(&mut self, nbits: usize) -> I
    where
        I: BitMemory
    {
        let value = self.bits[self.index..(self.index + nbits)].load_be::<I>();
        self.index += nbits;
        value
    }
}

#[derive(Debug, Clone)]
enum TypeId {
    Literal,
    Operator,
}

impl From<u8> for TypeId {
    fn from(typeid: u8) -> Self {
        match typeid {
            4 => Self::Literal,
            _ => Self::Operator,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl From<&mut Parser> for Packet {
    fn from(parser: &mut Parser) -> Self {
        let version_width = 3;
        let typeid_width = 3;
        let length_typeid_width = 1;
        let total_length_width = 15;
        let num_sub_pkt_width = 11;

        let version = parser.take(version_width);
        let typeid: u8 = parser.take(typeid_width);
        let typeid = TypeId::from(typeid);
        let typ =
            match typeid {
                TypeId::Literal => {
                    let mut nibbles: Vec<u8> = Vec::new();

                    loop {
                        let more = parser.take::<u8>(1) == 1;
                        let nibble = parser.take(4);
                        nibbles.push(nibble);
                        if !more { break; }
                    }

                    let n_nibbles = nibbles
                        .iter()
                        .count();
                    let literal = nibbles
                        .iter()
                        .enumerate()
                        .fold(0u64, |literal, (i, nibble)| {
                            let nibble_idx = n_nibbles - i - 1;
                            literal | (*nibble as u64) << (4 * nibble_idx)
                        });

                    Type::Literal(literal)
                }
                TypeId::Operator => {
                    let length_typeid = parser.take::<u8>(length_typeid_width) == 1;
                    match length_typeid {
                        false => {
                            let total_length: usize = parser.take(total_length_width);

                            let mut parsed_length = 0;
                            let mut packets: Vec<Packet> = Vec::new();
                            loop {
                                let index = parser.index;
                                packets.push(Packet::from(&mut *parser));
                                let packet_length = parser.index - index;
                                parsed_length += packet_length;
                                if parsed_length == total_length {
                                    break;
                                }
                            }

                            Type::Operator(packets)
                        }
                        true => {
                            let num_sub_pkt: u16 = parser.take(num_sub_pkt_width);
                            let packets = (0..num_sub_pkt)
                                .map(|_| Packet::from(&mut *parser))
                                .collect::<Vec<Packet>>();

                            Type::Operator(packets)
                        }
                    }
                }
            };

        Self { version, typ }
    }
}

fn version_sum(packet: &Packet) -> usize {
    let version: usize = packet.version.into();

    version +
        match &packet.typ {
            Type::Literal(_) => 0,
            Type::Operator(children) => {
                children
                    .iter()
                    .map(|child| version_sum(child))
                    .sum()
            }
        }
}

fn p1(s: &str) -> usize {
    let input = Input::from(s);
    let mut parser = Parser::from(&input);
    let packet = Packet::from(&mut parser);
    version_sum(&packet)
}

fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn p1() {
        assert_eq!(Packet::from(&mut Parser::from(&Input::from("D2FE28"))),
            Packet {
                version: 6,
                typ: Type::Literal(2021),
            }
        );
        assert_eq!(super::p1("8A004A801A8002F478"), 16);
        assert_eq!(super::p1("620080001611562C8802118E34"), 12);
        assert_eq!(super::p1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::p1("A0016C880162017C3686B18A3D4780"), 31);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 860);
    }

    #[test]
    #[ignore]
    fn p2() {
        // assert_eq!(super::p2(INPUT), 2188189693529);

        let input = input(DAY);
        // assert_eq!(super::p2(&input), 4110215602456);
    }
}
