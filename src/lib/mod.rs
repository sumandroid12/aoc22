pub mod parser;
use std::str::FromStr;
use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::pair,
    IResult,
};

pub fn parse_number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(pair(opt(char('-')), digit1)), str::parse::<T>)(input)
}
