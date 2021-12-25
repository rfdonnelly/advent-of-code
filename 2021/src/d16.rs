use crate::input;

#[cfg(feature = "nom")]
mod d16_nom;
#[cfg(feature = "nom")]
use d16_nom::*;

#[cfg(feature = "bitvec")]
mod d16_bitvec;
#[cfg(feature = "bitvec")]
use d16_bitvec::*;

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
