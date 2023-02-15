use std::{collections::VecDeque, str::FromStr, time::Instant};

use nom::{
    bytes::complete::{tag, take, take_until},
    character::{
        complete::{digit1, newline},
        streaming::space1,
    },
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type Id = u64;
#[derive(Debug)]
struct Monkey {
    id: Id,
    items: Vec<u64>,
    operation: (String, String),
    test: u64,
    test_true: Id,
    test_false: Id,
}
pub fn parse_numbers(input: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(input)
}

impl Monkey {
    fn process(&mut self) -> Vec<(u64, Id)> {
        let list = self
            .items
            .iter()
            .map(|&item| {
                // LCM of all divisibility factors
                let item = item % 9699690;
                let item = match (self.operation.0.as_ref(), self.operation.1.as_ref()) {
                    ("+", "old") => item + item,
                    ("+", operand) => item + operand.parse::<u64>().unwrap(),
                    ("*", "old") => item * item,
                    ("*", operand) => item * operand.parse::<u64>().unwrap(),
                    _ => panic!(),
                };
                let item = item % 9699690;
                if item % self.test == 0 {
                    (item, self.test_true)
                } else {
                    (item, self.test_false)
                }
            })
            .collect();
        self.items.clear();
        list
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (_, id, _)) = tuple((tag("Monkey "), parse_numbers, tag(":\n")))(input)?;
        let (input, (_, _, items, _)) = tuple((
            space1,
            tag("Starting items: "),
            separated_list1(tag(", "), parse_numbers),
            newline,
        ))(input)?;
        let (input, (_, _, operator, operand, _)) = tuple((
            space1,
            tag("Operation: new = old "),
            take(1usize),
            take_until("\n"),
            newline,
        ))(input)?;
        let (input, (_, _, div_by, _)) =
            tuple((space1, tag("Test: divisible by "), parse_numbers, newline))(input)?;
        let (input, (_, _, true_throw, _)) = tuple((
            space1,
            tag("If true: throw to monkey "),
            parse_numbers,
            newline,
        ))(input)?;
        let (input, (_, _, false_throw, _)) = tuple((
            space1,
            tag("If false: throw to monkey "),
            parse_numbers,
            newline,
        ))(input)?;
        Ok((
            input,
            Monkey {
                id,
                items,
                operation: (operator.to_owned(), operand.trim().to_owned()),
                test: div_by,
                test_true: true_throw,
                test_false: false_throw,
            },
        ))
    }
}

#[test]
fn monkey_business() {
    let before = Instant::now();
    let file = include_str!("day11.in");
    let (_input, mut monkeys) = separated_list1(newline, Monkey::parse)(file).unwrap();

    let mut inspect_counter = [0; 8];
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let inspected = monkeys[m].process();
            inspect_counter[m] += inspected.len();
            inspected.iter().for_each(|(item, id)| {
                monkeys[*id as usize].items.push(*item);
            })
        }
    }
    println!("Elapsed: {:.2?}", before.elapsed());
    dbg!(&inspect_counter);
}
