use std::{cmp::Ordering, fmt, str::FromStr, vec};

use nom::{
    branch::alt, bytes::streaming::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list0, sequence::tuple, IResult,
};

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(l), Packet::Number(r)) => Some(l.cmp(r)),
            (Packet::Number(l), Packet::List(r)) => {
                Packet::List(vec![Packet::Number(*l)]).partial_cmp(other)
            }
            (Packet::List(l), Packet::Number(r)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Number(*r)]))
            }
            (Packet::List(l), Packet::List(r)) => Some(
                l.iter()
                    .zip(r.iter())
                    .map(|(aa, bb)| aa.cmp(bb))
                    .find(|&ord| ord != Ordering::Equal)
                    .unwrap_or_else(|| l.len().cmp(&r.len())),
            ),
        }
    }
}

#[allow(unused_must_use)]
impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(arg0) => write!(f, "{}", arg0),
            Self::List(arg0) => {
                write!(f, "[");
                arg0.into_iter().for_each(|ele| {
                    write!(f, "{:?},", ele);
                });
                write!(f, "]")
            }
        }
    }
}

impl Packet {
    fn parse_num(i: &str) -> IResult<&str, Self> {
        let (input, num) = map_res(digit1, i32::from_str)(i)?;
        Ok((input, Self::Number(num)))
    }
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (input, (_, num_list, _)) = tuple((
            tag("["),
            separated_list0(tag(","), alt((Self::parse_num, Self::parse))),
            tag("]"),
        ))(i)?;
        Ok((input, Self::List(num_list)))
    }
}
#[test]
fn struct_test() {
    dbg!(Packet::parse(
        "[[4,[[4,0,1,2],[],[6,4,0],[2,3]],8,[[4],[8,1,0,8],2]],[[[],0]],[],[7],[]]"
    ));
}

#[test]
fn distress_signal() {
    let file = include_str!("day13.in");
    let mut sum = 0;
    let mut packets = vec![];
    for (id, group) in file.split("\n\n").enumerate() {
        let mut splitter = group.split("\n");
        let (_, p1) = Packet::parse(splitter.next().unwrap()).unwrap();
        let (_, p2) = Packet::parse(splitter.next().unwrap()).unwrap();
        if p1 < p2 {
            sum += id + 1;
        }
        packets.push(p1);
        packets.push(p2);
    }
    dbg!(sum);
    // part 2
    let dividers = vec![
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    ];
    packets.extend(dividers.iter().cloned());
    packets.sort();

    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    dbg!(decoder_key);
}
