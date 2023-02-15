use nom::{
    bytes::streaming::tag,
    character::complete::digit1,
    combinator::{complete, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const MAX_X: usize = 511;
const MIN_X: usize = 458;
const MAX_Y: usize = 166;
#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);
pub fn parse_numbers(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}
impl Path {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (input, points) = separated_list1(complete(tag(" -> ")), Point::parse)(i)?;
        Ok((input, Path { points }))
    }
    fn draw(&self, grid: &mut Vec<Vec<char>>) {
        let points = &self.points;
        points
            .iter()
            .zip(points.iter().skip(1))
            .for_each(|(&Point(x1, y1), &Point(x2, y2))| {
                if x1 == x2 {
                    if y1 <= y2 {
                        (y1..=y2).for_each(|y| grid[y][x1] = '#');
                    } else {
                        (y2..=y1).for_each(|y| grid[y][x1] = '#');
                    }
                } else {
                    if x1 <= x2 {
                        (x1..=x2).for_each(|x| grid[y1][x] = '#');
                    } else {
                        (x2..=x1).for_each(|x| grid[y1][x] = '#');
                    }
                }
            })
    }
}

impl Point {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (input, (x, y)) = separated_pair(parse_numbers, tag(","), parse_numbers)(i)?;
        Ok((input, Point(x, y)))
    }
}

#[test]
fn regolith_reseervoir() {
    let file = include_str!("day14.in");
    let mut caves = vec![vec!['.'; 2 * MAX_X + 10]; MAX_Y + 10];
    let paths = file
        .lines()
        .map(|l| Path::parse(l).unwrap().1)
        .collect::<Vec<_>>();
    paths.iter().for_each(|pt| pt.draw(&mut caves));

    // base floor
    caves[MAX_Y + 2].iter_mut().for_each(|c| *c = '#');

    let source = Point(500, 0);
    let mut count = 0;
    while let Some(Point(x, y)) = sand_rest_position(&caves, &source) {
        count += 1;
        if y == 0 {
            break;
        }
        caves[y][x] = '#';
    }
    dbg!(count);
}

type Grid = Vec<Vec<char>>;

fn sand_rest_position(caves: &Grid, source: &Point) -> Option<Point> {
    let mut pos = source.to_owned();
    loop {
        let Point(x, y) = pos;
        // check down
        if caves[y + 1][x] == '.' {
            pos = Point(x, y + 1);
        // check down left
        } else if caves[y + 1][x - 1] == '.' {
            pos = Point(x - 1, y + 1);
        // check down right
        } else if caves[y + 1][x + 1] == '.' {
            pos = Point(x + 1, y + 1);
        // rest
        } else {
            return Some(pos);
        }
    }
}
