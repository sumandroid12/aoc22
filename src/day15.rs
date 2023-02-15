use nom::{bytes::complete::tag, sequence::tuple, IResult};

use crate::lib::parse_number;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    location: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn radius(&self) -> i32 {
        (self.location.0 - self.beacon.0).abs() + (self.location.1 - self.beacon.1).abs()
    }
    fn parse(i: &str) -> IResult<&str, Self> {
        let (input, (_, sx, _, sy, _, bx, _, by)) = tuple((
            tag("Sensor at x="),
            parse_number,
            tag(", y="),
            parse_number,
            tag(": closest beacon is at x="),
            parse_number,
            tag(", y="),
            parse_number,
        ))(i)?;
        Ok((
            input,
            Sensor {
                location: (sx, sy),
                beacon: (bx, by),
            },
        ))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Interval(i32, i32);

impl Interval {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.0 <= other.0 {
            if other.0 <= self.1 {
                return Some(Interval(self.0, other.1.max(self.1)));
            } else {
                return None;
            }
        } else {
            if self.0 <= other.1 {
                return Some(Interval(other.0, self.1.max(other.1)));
            } else {
                return None;
            }
        }
    }
}

#[test]
fn interval_test() {
    assert_eq!(Interval(1, 5).merge(&Interval(2, 7)), Some(Interval(1, 7)));
    assert_eq!(Interval(3, 9).merge(&Interval(4, 5)), Some(Interval(3, 9)));
    assert_eq!(Interval(4, 5).merge(&Interval(3, 9)), Some(Interval(3, 9)));
    assert_eq!(Interval(4, 5).merge(&Interval(5, 9)), Some(Interval(4, 9)));
}

#[test]
fn main() {
    let file = include_str!("day15.in");
    let mut sensors = file
        .lines()
        .map(|l| Sensor::parse(l).unwrap().1)
        .collect::<Vec<_>>();
    for row in 0..=4000000 {
        let mut coverage = vec![];
        sensors.iter().for_each(|s| {
            let row: i32 = row;
            let rad = s.radius();
            let rem = rad - row.abs_diff(s.location.1) as i32;
            if rem >= 0 {
                let reachable = Interval(
                    // (s.location.0 - rem),
                    // (s.location.0 + rem)
                    (s.location.0 - rem).clamp(0, 4000000),
                    (s.location.0 + rem).clamp(0, 4000000),
                );
                coverage.push(reachable);
            }
        });
        coverage.sort();
        let mut acc = coverage[0];
        coverage
            .iter()
            .find(|&i| {
                if let Some(int) = acc.merge(i) {
                    acc = int;
                    false
                } else {
                    println!("{:?}", (acc, i));
                    true
                }
            })
            .unwrap_or(&Interval(0, 20));
        if acc != Interval(0, 4000000) {
            println!("{:?}", (row, acc));
            return;
        }
    }
}
