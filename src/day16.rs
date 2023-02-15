use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

use crate::lib::parse_number;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::complete,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct ValveRaw<'a> {
    flow_rate: usize,
    name: &'a str,
    tunnels: Rc<Vec<&'a str>>,
}
struct Valve {
    flow_rate: usize,
    id: usize,
    tunnels: Vec<usize>,
}

impl<'a> ValveRaw<'a> {
    fn parse(i: &'a str) -> IResult<&str, Self> {
        let (input, (_, name, _, flow_rate, _, tunnels)) = tuple((
            tag("Valve "),
            take(2usize),
            tag(" has flow rate="),
            parse_number,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(complete(tag(", ")), take(2usize)),
        ))(i)?;
        Ok((
            input,
            ValveRaw {
                flow_rate,
                tunnels: Rc::new(tunnels),
                name,
            },
        ))
    }
}

#[test]
fn volcanic_emergency() {
    let file = include_str!("day16.in");
    let mut valves = file
        .lines()
        .map(|l| ValveRaw::parse(l).unwrap().1)
        .collect::<Vec<_>>();

    valves.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate));
    valves.reverse();

    let mut get_id = HashMap::new();
    valves.iter().enumerate().for_each(|(id, v)| {
        get_id.insert(v.name, id);
    });

    let valves = valves
        .iter()
        .map(|v| {
            let id = get_id[v.name];
            let tunnels = v.tunnels.iter().map(|t| get_id[t]).collect::<Vec<_>>();
            let flow_rate = v.flow_rate;
            Valve {
                id,
                tunnels,
                flow_rate,
            }
        })
        .collect::<Vec<_>>();
    let n = valves.len();
    let mut distance_all_pairs = vec![];
    for i in 0..n {
        let mut distances = vec![usize::MAX; n];
        distances[i] = 0;
        dijkstra(i, &mut distances, &valves);
        distance_all_pairs.push(distances);
    }

    let n = valves.iter().filter(|v| v.flow_rate > 0).count();
    let mut visited = HashSet::new();

    let ans = max_pressure(
        n,
        get_id["AA"],
        30,
        &mut visited,
        &distance_all_pairs,
        0,
        &valves,
    );
    dbg!(ans);
}

fn max_pressure(
    n: usize,
    curr: usize,
    time: usize,
    visited: &mut HashSet<usize>,
    distance_all_pairs: &Vec<Vec<usize>>,
    pressure_released: usize,
    valves: &Vec<Valve>,
) -> usize {
    if time == 0 {
        return pressure_released;
    }
    if visited.len() == n {
        return pressure_released;
    }
    let mut maxp = pressure_released;
    for i in 0..n {
        if !visited.contains(&i) {
            let time_left = time as i32 - distance_all_pairs[curr][i] as i32 - 1;
            if time_left <= 0 {
                continue;
            }
            let time_left = time_left as usize;
            let pressurereleased = pressure_released + time_left * valves[i].flow_rate;
            visited.insert(i);
            let pressure = max_pressure(
                n,
                i,
                time_left,
                visited,
                distance_all_pairs,
                pressure_released,
                valves,
            );
            maxp = maxp.max(pressure);
            visited.remove(&i);
        }
    }
    maxp
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    distance: usize,
    id: usize,
}

fn dijkstra(src: usize, distances: &mut [usize], valves: &Vec<Valve>) {
    let n = distances.len();
    let mut visited = vec![false; n];
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(Vertex {
        id: src,
        distance: 0,
    }));
    while let Some(Reverse(Vertex {
        id,
        distance: curr_dist,
    })) = pq.pop()
    {
        let neighbors = &valves[id].tunnels;
        for &n in neighbors {
            if !visited[n] && distances[n] > curr_dist + 1 {
                distances[n] = curr_dist + 1;
                pq.push(Reverse(Vertex {
                    distance: distances[n],
                    id: n,
                }))
            }
        }
        visited[id] = true;
    }
}
