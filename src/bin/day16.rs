use std::collections::VecDeque;
use std::str::FromStr;
use thiserror::Error;

const REAL_DATA: &str = include_str!("../../data/day16.txt");

#[derive(Debug, Error, PartialEq)]
enum Oopsie {
    #[error("Bad digit in input: '{0}'")]
    BadDigit(char),
    #[error("Ran out of bits (looking for {1}) pulling value for type {0}")]
    RanOuttaBits(&'static str, usize),
    #[error("Invalid split attempt: stream length is {0}, but requested split at {1}")]
    InvalidSplit(usize, usize),
}

struct BitStream {
    bits: VecDeque<bool>,
}

impl FromStr for BitStream {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = VecDeque::with_capacity(s.len() * 4);

        for c in s.chars() {
            let value = c.to_digit(16).ok_or(Oopsie::BadDigit(c))?;
            bits.push_back(value & 0b1000 > 0);
            bits.push_back(value & 0b0100 > 0);
            bits.push_back(value & 0b0010 > 0);
            bits.push_back(value & 0b0001 > 0);
        }

        Ok(BitStream { bits })
    }
}

macro_rules! next_chunk {
    ($id: ident, $t: ty) => {
        fn $id(&mut self, bits: usize) -> Result<$t, Oopsie> {
            let mut res = 0;

            for _ in 0..bits {
                res <<= 1;
                match self.bits.pop_front() {
                    None => return Err(Oopsie::RanOuttaBits(stringify!($t), bits)),
                    Some(true) => res += 1,
                    Some(false) => {}
                }
            }

            Ok(res)
        }
    };
}

impl BitStream {
    fn next_bit(&mut self) -> Result<bool, Oopsie> {
        self.bits.pop_front().ok_or(Oopsie::RanOuttaBits("bool", 1))
    }

    next_chunk!(next_u8, u8);
    next_chunk!(next_u16, u16);
    next_chunk!(next_u64, u64);

    fn take(&mut self, size: usize) -> Result<BitStream, Oopsie> {
        if self.bits.len() < size {
            Err(Oopsie::InvalidSplit(self.bits.len(), size))
        } else {
            let rest = self.bits.split_off(size);
            let retval = self.bits.clone();
            self.bits = rest;

            Ok(BitStream { bits: retval })
        }
    }

    fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
}

#[derive(Debug, PartialEq)]
enum Message {
    Literal(u8, u64),
    Sum(u8, Vec<Message>),
    Product(u8, Vec<Message>),
    Minimum(u8, Vec<Message>),
    Maximum(u8, Vec<Message>),
    GreaterThan(u8, Vec<Message>),
    LessThan(u8, Vec<Message>),
    EqualTo(u8, Vec<Message>),
    Sequence(u8, u8, Vec<Message>),
}

impl Message {
    fn sequence(version: u8, type_id: u8, sequence: Vec<Message>) -> Message {
        match type_id {
            0 => Message::Sum(version, sequence),
            1 => Message::Product(version, sequence),
            2 => Message::Minimum(version, sequence),
            3 => Message::Maximum(version, sequence),
            5 if sequence.len() == 2 => Message::GreaterThan(version, sequence),
            6 if sequence.len() == 2 => Message::LessThan(version, sequence),
            7 if sequence.len() == 2 => Message::EqualTo(version, sequence),
            _ => Message::Sequence(version, type_id, sequence),
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Message::Literal(x, _) => *x as usize,
            Message::Sum(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::Product(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::Minimum(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::Maximum(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::GreaterThan(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::LessThan(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::EqualTo(x, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
            Message::Sequence(x, _, seq) => {
                (*x as usize) + seq.iter().map(|v| v.version_sum()).sum::<usize>()
            }
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Message::Literal(_, x) => *x,
            Message::Sum(_, seq) => seq.iter().map(|x| x.eval()).sum(),
            Message::Product(_, seq) => seq.iter().map(|x| x.eval()).product(),
            Message::Minimum(_, seq) => seq.iter().map(|x| x.eval()).min().unwrap(),
            Message::Maximum(_, seq) => seq.iter().map(|x| x.eval()).max().unwrap(),
            Message::GreaterThan(_, seq) => {
                if seq[0].eval() > seq[1].eval() {
                    1
                } else {
                    0
                }
            }
            Message::LessThan(_, seq) => {
                if seq[0].eval() < seq[1].eval() {
                    1
                } else {
                    0
                }
            }
            Message::EqualTo(_, seq) => {
                if seq[0].eval() == seq[1].eval() {
                    1
                } else {
                    0
                }
            }
            Message::Sequence(_, _, _) => panic!("Tried to evaluate unknown sequence!"),
        }
    }
}

impl<'a> TryFrom<&'a mut BitStream> for Message {
    type Error = Oopsie;

    fn try_from(value: &mut BitStream) -> Result<Self, Self::Error> {
        let version = value.next_u8(3)?;
        let type_id = value.next_u8(3)?;

        if type_id == 4 {
            let mut literal = 0u64;
            let mut keep_going = true;

            while keep_going {
                keep_going = value.next_bit()?;
                literal = (literal << 4) + value.next_u64(4)?;
            }

            Ok(Message::Literal(version, literal))
        } else {
            let length_type_id = value.next_bit()?;
            let mut seq = Vec::new();

            if length_type_id {
                let subpart_count = value.next_u16(11)?;

                for _ in 0..subpart_count {
                    seq.push(Message::try_from(&mut *value)?);
                }
            } else {
                let subpart_len = value.next_u16(15)? as usize;
                let mut my_bits = value.take(subpart_len)?;

                while !my_bits.is_empty() {
                    seq.push(Message::try_from(&mut my_bits)?);
                }
            }

            Ok(Message::sequence(version, type_id, seq))
        }
    }
}

impl FromStr for Message {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = BitStream::from_str(s)?;
        Message::try_from(&mut bits)
    }
}

#[test]
fn basic_parsing() {
    assert_eq!(
        Ok(Message::Literal(6, 2021)),
        Message::try_from(&mut BitStream::from_str("D2FE28").unwrap())
    );
    assert_eq!(Ok(Message::Literal(6, 2021)), Message::from_str("D2FE28"));
    assert_eq!(
        Ok(Message::LessThan(
            1,
            vec![Message::Literal(6, 10), Message::Literal(2, 20)]
        )),
        Message::from_str("38006F45291200")
    );
    assert_eq!(
        Ok(Message::Maximum(
            7,
            vec![
                Message::Literal(2, 1),
                Message::Literal(4, 2),
                Message::Literal(1, 3)
            ]
        )),
        Message::from_str("EE00D40C823060")
    );
}

#[test]
fn example_tests() {
    assert_eq!(
        16,
        Message::from_str("8A004A801A8002F478")
            .unwrap()
            .version_sum()
    );
    assert_eq!(
        12,
        Message::from_str("620080001611562C8802118E34")
            .unwrap()
            .version_sum()
    );
    assert_eq!(
        23,
        Message::from_str("C0015000016115A2E0802F182340")
            .unwrap()
            .version_sum()
    );
    assert_eq!(
        31,
        Message::from_str("A0016C880162017C3686B18A3D4780")
            .unwrap()
            .version_sum()
    );
}

fn main() -> Result<(), Oopsie> {
    let message = Message::from_str(REAL_DATA)?;
    println!("Version sum: {}", message.version_sum());
    println!("Input computed value: {}", message.eval());
    Ok(())
}
