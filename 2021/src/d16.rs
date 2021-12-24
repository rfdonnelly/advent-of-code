use crate::input;

#[cfg(feature = "bitvec")]
use bitvec::{
    mem::BitMemory,
    prelude::*,
};

#[cfg(feature = "nom")]
use nom::{
    IResult,
    bits::{
        bits,
        complete::{
            take,
            tag,
        },
    },
    sequence::tuple,
};

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
                chunk.iter().enumerate().fold(0u8, |byte, (i, nibble)| {
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

const WIDTH_VERSION: usize = 3;
const WIDTH_TYPEID: usize = 3;
const WIDTH_LENGTH_TYPEID: usize = 1;
const WIDTH_TOTAL_LENGTH: usize = 15;
const WIDTH_NUM_SUB_PKT: usize = 11;

#[cfg(feature = "bitvec")]
struct Bits {
    bits: BitVec<Msb0, u8>,
    index: usize,
}

#[cfg(feature = "bitvec")]
impl From<&Input> for Bits {
    fn from(input: &Input) -> Self {
        let bits = BitVec::<Msb0, _>::from_slice(&input.bytes).unwrap();

        Self { bits, index: 0 }
    }
}

#[cfg(feature = "bitvec")]
impl Bits {
    fn take<I>(&mut self, nbits: usize) -> I
    where
        I: BitMemory,
    {
        let value = self.bits[self.index..(self.index + nbits)].load_be::<I>();
        self.index += nbits;
        value
    }
}

#[derive(Debug, Clone)]
enum TypeId {
    Literal,
    Operator(Op),
}

impl From<u8> for TypeId {
    fn from(typeid: u8) -> Self {
        match typeid {
            4 => Self::Literal,
            _ => Self::Operator(Op::from(typeid)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Literal(u64),
    Operator(Op, Vec<Packet>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u8> for Op {
    fn from(opid: u8) -> Self {
        match opid {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => unreachable!(),
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "bitvec")]
impl From<&mut Bits> for Packet {
    fn from(bits: &mut Bits) -> Self {
        let version = bits.take(WIDTH_VERSION);
        let typeid = bits.take::<u8>(WIDTH_TYPEID);
        let typeid = TypeId::from(typeid);
        let typ = match typeid {
            TypeId::Literal => {
                let mut nibbles: Vec<u8> = Vec::new();

                loop {
                    let more = bits.take::<u8>(1) == 1;
                    let nibble = bits.take(4);
                    nibbles.push(nibble);
                    if !more {
                        break;
                    }
                }

                let n_nibbles = nibbles.iter().count();
                let literal = nibbles
                    .iter()
                    .enumerate()
                    .fold(0u64, |literal, (i, nibble)| {
                        let nibble_idx = n_nibbles - i - 1;
                        literal | (*nibble as u64) << (4 * nibble_idx)
                    });

                Type::Literal(literal)
            }
            TypeId::Operator(op) => {
                let length_typeid = bits.take::<u8>(WIDTH_LENGTH_TYPEID) == 1;
                match length_typeid {
                    false => {
                        let total_length: usize = bits.take(WIDTH_TOTAL_LENGTH);

                        let mut parsed_length = 0;
                        let mut packets: Vec<Packet> = Vec::new();
                        loop {
                            let index = bits.index;
                            packets.push(Packet::from(&mut *bits));
                            let packet_length = bits.index - index;
                            parsed_length += packet_length;
                            if parsed_length == total_length {
                                break;
                            }
                        }

                        Type::Operator(op, packets)
                    }
                    true => {
                        let num_sub_pkt: u16 = bits.take(WIDTH_NUM_SUB_PKT);
                        let packets = (0..num_sub_pkt)
                            .map(|_| Packet::from(&mut *bits))
                            .collect::<Vec<Packet>>();

                        Type::Operator(op, packets)
                    }
                }
            }
        };

        Self { version, typ }
    }
}

#[cfg(feature = "nom")]
fn parse_bytes(i: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(i)
}

#[cfg(feature = "nom")]
fn parse_packet_type_literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), Type> {
    let mut nibbles: Vec<u8> = Vec::new();

    let mut i = i;
    loop {
        let (more, nibble): (u8, u8);
        (i, (more, nibble)) = tuple((take(1usize), take(4usize)))(i)?;
        nibbles.push(nibble);
        let more = more == 1;
        if !more {
            break;
        }
    }

    let n_nibbles = nibbles.iter().count();
    let literal = nibbles
        .iter()
        .enumerate()
        .fold(0u64, |literal, (i, nibble)| {
            let nibble_idx = n_nibbles - i - 1;
            literal | (*nibble as u64) << (4 * nibble_idx)
        });

    Ok((i, Type::Literal(literal)))
}

#[cfg(feature = "nom")]
fn parse_packet_type_operator(i: (&[u8], usize), op: Op) -> IResult<(&[u8], usize), Type> {
    let (i, length_typeid): (_, u16) = take(WIDTH_LENGTH_TYPEID)(i)?;
    let length_typeid = length_typeid == 1;

    match length_typeid {
        false => {
            let (i, total_length) = take(WIDTH_TOTAL_LENGTH)(i)?;

            let mut parsed_length = 0;
            let mut packets: Vec<Packet> = Vec::new();
            let mut i = i;
            loop {
                let bit_remaining_before = 8 * i.0.len() - i.1;
                let packet;
                (i, packet) = parse_packet_bits(i)?;
                packets.push(packet);
                let bits_remaining_after = 8 * i.0.len() - i.1;
                let packet_length = bit_remaining_before - bits_remaining_after;
                parsed_length += packet_length;
                if parsed_length == total_length {
                    break;
                }
            }

            Ok((i, Type::Operator(op, packets)))
        }
        true => {
            let (i, num_sub_pkt): (_, u16) = take(WIDTH_NUM_SUB_PKT)(i)?;
            let mut i = i;
            let mut packets: Vec<Packet> = Vec::new();
            for _ in 0..num_sub_pkt {
                let packet;
                (i, packet) = parse_packet_bits(i)?;
                packets.push(packet);
            }

            Ok((i, Type::Operator(op, packets)))
        }
    }
}

#[cfg(feature = "nom")]
fn parse_packet_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (i, version) = take(WIDTH_VERSION)(i)?;
    let (i, typeid): (_, u8) = take(WIDTH_TYPEID)(i)?;
    let typeid = TypeId::from(typeid);
    let (i, typ) = match typeid {
        TypeId::Literal => parse_packet_type_literal(i)?,
        TypeId::Operator(op) => parse_packet_type_operator(i, op)?,
    };

    let packet = Packet { version, typ };
    Ok((i, packet))
}

#[cfg(feature = "nom")]
impl From<&[u8]> for Packet {
    fn from(bytes: &[u8]) -> Self {
        let (_, packet) = parse_bytes(bytes).unwrap();
        packet
    }
}

impl Packet {
    fn version_sum(&self) -> usize {
        let version: usize = self.version.into();

        version
            + match &self.typ {
                Type::Literal(_) => 0,
                Type::Operator(_, children) => children.iter().map(Packet::version_sum).sum(),
            }
    }

    fn eval(&self) -> usize {
        match &self.typ {
            Type::Literal(v) => *v as usize,
            Type::Operator(op, children) => {
                let children = children.iter().map(Packet::eval);
                match op {
                    Op::Sum => children.sum(),
                    Op::Product => children.product(),
                    Op::Minimum => children.min().unwrap(),
                    Op::Maximum => children.max().unwrap(),
                    Op::GreaterThan => children.greater_than() as usize,
                    Op::LessThan => children.less_than() as usize,
                    Op::EqualTo => children.equal_to() as usize,
                }
            }
        }
    }
}

pub trait ComparisonIteratorAdapter
where
    Self: Sized + Iterator,
{
    fn greater_than(self) -> bool;
    fn less_than(self) -> bool;
    fn equal_to(self) -> bool;
}

impl<I> ComparisonIteratorAdapter for I
where
    I: Iterator,
    I::Item: PartialOrd + PartialEq,
{
    fn greater_than(mut self) -> bool {
        self.next().unwrap() > self.next().unwrap()
    }

    fn less_than(mut self) -> bool {
        self.next().unwrap() < self.next().unwrap()
    }

    fn equal_to(mut self) -> bool {
        self.next().unwrap() == self.next().unwrap()
    }
}

fn p1(s: &str) -> usize {
    let input = Input::from(s);

    #[cfg(feature = "bitvec")]
    {
        let mut bits = Bits::from(&input);
        Packet::from(&mut bits).version_sum()
    }

    #[cfg(feature = "nom")]
    Packet::from(input.bytes.as_ref()).version_sum()
}

fn p2(s: &str) -> usize {
    let input = Input::from(s);

    #[cfg(feature = "bitvec")]
    {
        let mut bits = Bits::from(&input);
        Packet::from(&mut bits).eval()
    }

    #[cfg(feature = "nom")]
    Packet::from(input.bytes.as_ref()).eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        #[cfg(feature = "bitvec")]
        assert_eq!(
            Packet::from(&mut Bits::from(&Input::from("D2FE28"))),
            Packet {
                version: 6,
                typ: Type::Literal(2021),
            }
        );
        #[cfg(feature = "nom")]
        assert_eq!(
            Packet::from(Input::from("D2FE28").bytes.as_ref()),
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
    fn p2() {
        assert_eq!(super::p2("C200B40A82"), 3);
        assert_eq!(super::p2("04005AC33890"), 54);
        assert_eq!(super::p2("880086C3E88112"), 7);
        assert_eq!(super::p2("CE00C43D881120"), 9);
        assert_eq!(super::p2("D8005AC2A8F0"), 1);
        assert_eq!(super::p2("F600BC2D8F"), 0);
        assert_eq!(super::p2("9C005AC2F8F0"), 0);
        assert_eq!(super::p2("9C0141080250320F1802104A08"), 1);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 470949537659);
    }
}
