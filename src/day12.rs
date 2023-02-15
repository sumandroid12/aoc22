use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    time::Instant,
};

use nom::{
    bytes::{complete::take_until, streaming::tag},
    multi::separated_list1,
    IResult,
};

fn parse_grid(file: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(tag("\n"), parse_row)(file)
}
fn parse_row(row: &str) -> IResult<&str, Vec<char>> {
    let (input, row) = take_until("\n")(row)?;
    Ok((input, row.chars().collect()))
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    distance: u32,
    location: (usize, usize),
}

#[test]
fn hill_climb() {
    let before = Instant::now();
    let (_, mut grid) = parse_grid(include_str!("day12.in")).unwrap();
    let (r, c) = (grid.len(), grid[0].len());
    let start = grid.iter().flatten().position(|c| *c == 'E').unwrap();
    let start = (start / c, start % c);
    let end = grid.iter().flatten().position(|c| *c == 'S').unwrap();
    let end = (end / c, end % c);
    grid[start.0][start.1] = 'z';
    grid[end.0][end.1] = 'a';

    let mut visited = vec![vec![false; c]; r];
    let mut distances = vec![vec![u32::MAX; c]; r];
    let mut pq = BinaryHeap::new();
    distances[start.0][start.1] = 0;
    pq.push(Reverse(Vertex {
        distance: 0,
        location: start,
    }));
    while let Some(Reverse(Vertex {
        distance: curr_dist,
        location,
    })) = pq.pop()
    {
        if !visited[location.0][location.1] {
            let curr = location;
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(i, j)| (curr.0 as i32 + *i, curr.1 as i32 + *j))
                .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < r as i32 && *j < c as i32)
                .filter(|(i, j)| {
                    let curr_elevation = grid[curr.0][curr.1] as i32;
                    let dest_elevation = grid[*i as usize][*j as usize] as i32;
                    curr_elevation - dest_elevation <= 1
                })
                .for_each(|(i, j)| {
                    let new_dist = curr_dist + 1;
                    let old_dist = &mut distances[i as usize][j as usize];
                    if !visited[i as usize][j as usize] && *old_dist > new_dist {
                        *old_dist = new_dist;
                        pq.push(Reverse(Vertex {
                            distance: new_dist,
                            location: (i as usize, j as usize),
                        }));
                    }
                });
            visited[curr.0][curr.1] = true;
        }
    }
    let min_step = grid
        .iter()
        .flatten()
        .enumerate()
        .filter(|(id, elev)| **elev == 'a')
        .map(|(id, _)| {
            let location = (id / c, id % c);
            distances[location.0][location.1]
        })
        .min()
        .unwrap();
    println!("Elapsed: {:.2?}", before.elapsed());
    dbg!(min_step);
}
