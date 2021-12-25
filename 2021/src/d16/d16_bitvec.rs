use bitvec::{
    mem::BitMemory,
    prelude::*,
};

use super::*;

pub struct Bits {
    bits: BitVec<Msb0, u8>,
    index: usize,
}

impl From<&Input> for Bits {
    fn from(input: &Input) -> Self {
        let bits = BitVec::<Msb0, _>::from_slice(&input.bytes).unwrap();

        Self { bits, index: 0 }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Packet::from(&mut Bits::from(&Input::from("D2FE28"))),
            Packet {
                version: 6,
                typ: Type::Literal(2021),
            }
        );
    }
}
