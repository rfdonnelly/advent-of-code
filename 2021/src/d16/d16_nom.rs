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

use super::*;

fn parse_bytes(input: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(input)
}

fn parse_packet_type_literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), Type> {
    let mut nibbles: Vec<u8> = Vec::new();

    let mut input = input;
    loop {
        let (more, nibble): (u8, u8);
        (input, (more, nibble)) = tuple((take(1usize), take(4usize)))(input)?;
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

    Ok((input, Type::Literal(literal)))
}

fn parse_packet_type_operator(input: (&[u8], usize), op: Op) -> IResult<(&[u8], usize), Type> {
    let (input, length_typeid): (_, u16) = take(WIDTH_LENGTH_TYPEID)(input)?;
    let length_typeid = length_typeid == 1;

    match length_typeid {
        false => {
            let (input, total_length) = take(WIDTH_TOTAL_LENGTH)(input)?;

            let mut parsed_length = 0;
            let mut packets: Vec<Packet> = Vec::new();
            let mut input = input;
            loop {
                let bit_remaining_before = 8 * input.0.len() - input.1;
                let packet;
                (input, packet) = parse_packet_bits(input)?;
                packets.push(packet);
                let bits_remaining_after = 8 * input.0.len() - input.1;
                let packet_length = bit_remaining_before - bits_remaining_after;
                parsed_length += packet_length;
                if parsed_length == total_length {
                    break;
                }
            }

            Ok((input, Type::Operator(op, packets)))
        }
        true => {
            let (input, num_sub_pkt): (_, u16) = take(WIDTH_NUM_SUB_PKT)(input)?;
            let mut input = input;
            let mut packets: Vec<Packet> = Vec::new();
            for _ in 0..num_sub_pkt {
                let packet;
                (input, packet) = parse_packet_bits(input)?;
                packets.push(packet);
            }

            Ok((input, Type::Operator(op, packets)))
        }
    }
}

fn parse_packet_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (input, version) = take(WIDTH_VERSION)(input)?;
    let (input, typeid): (_, u8) = take(WIDTH_TYPEID)(input)?;
    let typeid = TypeId::from(typeid);
    let (input, typ) = match typeid {
        TypeId::Literal => parse_packet_type_literal(input)?,
        TypeId::Operator(op) => parse_packet_type_operator(input, op)?,
    };

    let packet = Packet { version, typ };
    Ok((input, packet))
}

impl From<&[u8]> for Packet {
    fn from(bytes: &[u8]) -> Self {
        let (_, packet) = parse_bytes(bytes).unwrap();
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Packet::from(Input::from("D2FE28").bytes.as_ref()),
            Packet {
                version: 6,
                typ: Type::Literal(2021),
            }
        );
    }
}
