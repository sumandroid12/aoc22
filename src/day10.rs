use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res, rest},
    sequence::tuple,
    IResult,
};
#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_x| Instruction::Noop)(i)
}
fn addx(i: &str) -> IResult<&str, Instruction> {
    let (num_str, _) = tuple((tag("addx"), char(' ')))(i)?;
    let (_, num) = map_res(rest, FromStr::from_str)(num_str)?;
    Ok(("", Instruction::AddX(num)))
}

fn parse_line(i: &str) -> Option<Instruction> {
    alt((addx, noop))(i).ok().map(|(_, ins)| ins)
}

struct CPU {
    reg: i32,
    remaining: usize,
    curr_inst: Instruction,
}

#[test]
fn crt() {
    let mut instructions = include_str!("day10.in").lines().flat_map(parse_line);

    let mut cpu = CPU {
        reg: 1,
        remaining: 0,
        curr_inst: Instruction::Noop,
    };
    let mut screen = vec![vec!['.'; 40]; 6];

    for i in 0..240 {
        if cpu.remaining == 0 {
            // update register
            cpu.reg += match cpu.curr_inst {
                Instruction::Noop => 0,
                Instruction::AddX(num) => num,
            };
            if let Some(ins) = instructions.next() {
                match ins {
                    Instruction::Noop => {
                        cpu.curr_inst = ins;
                        cpu.remaining = 0
                    }
                    Instruction::AddX(_) => {
                        cpu.curr_inst = ins;
                        cpu.remaining = 1
                    }
                }
            } else {
                cpu.curr_inst = Instruction::Noop;
                cpu.remaining = 0
            }
        } else {
            cpu.remaining -= 1;
        }
        let row = i / 40;
        let col = i % 40;
        screen[row][col] = {
            if cpu.reg.abs_diff(col as i32) <= 1 {
                '#'
            } else {
                '.'
            }
        }
    }
    for ele in screen {
        for c in ele {
            print!("{c}");
        }
        println!();
    }
}
