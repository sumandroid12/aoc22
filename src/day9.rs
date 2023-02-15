use std::collections::HashSet;

struct Point(i32, i32);

// impl Add for Point {
//     type Output = Point;

//     fn add(self, rhs: Self) -> Self::Output {
//         Point(self.0 + rhs.0, self.1 + rhs.1)
//     }
// }
enum MoveOnce {
    Up,
    Down,
    Left,
    Right,
}
struct Move(MoveOnce, i32);
struct Rope {
    knots: [(i32, i32); 10],
}
#[test]
fn sim_rope() {
    let moves = include_str!("day9.in").lines().map(parse_line);
    let mut cur_pos = Rope {
        knots: [(0, 0); 10],
    };
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for mv in moves {
        let Move(dir, steps) = mv;
        for _ in 0..steps {
            cur_pos = move_rope(cur_pos, &dir);
            visited.insert(cur_pos.knots[9]);
        }
    }
    dbg!(visited.len());
}

fn move_rope(rope: Rope, mv: &MoveOnce) -> Rope {
    let Rope { mut knots } = rope;
    let head = &mut knots[0];
    match mv {
        MoveOnce::Up => head.0 += 1,
        MoveOnce::Down => head.0 -= 1,
        MoveOnce::Left => head.1 += 1,
        MoveOnce::Right => head.1 -= 1,
    };
    for (head, tail) in (0..10).zip(1..10) {
        let head = knots[head];
        let tail = &mut knots[tail];
        let x_dif = head.0 - tail.0;
        let y_dif = head.1 - tail.1;
        if !(x_dif.abs() <= 1 && y_dif.abs() <= 1) {
            if x_dif == 0 || y_dif == 0 {
                if x_dif == 2 {
                    tail.0 += 1
                } else if x_dif == -2 {
                    tail.0 -= 1
                } else if y_dif == 2 {
                    tail.1 += 1
                } else if y_dif == -2 {
                    tail.1 -= 1
                }
            } else {
                match (x_dif > 0, y_dif > 0) {
                    (true, true) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    (true, false) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (false, true) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (false, false) => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                }
            }
        }
    }
    Rope { knots }
}

fn parse_line(i: &str) -> Move {
    let (dir, step) = i.split_once(' ').unwrap();
    let step: i32 = step.parse().unwrap();
    let res = match dir {
        "L" => MoveOnce::Left,
        "R" => MoveOnce::Right,
        "U" => MoveOnce::Up,
        "D" => MoveOnce::Down,
        _ => panic!(),
    };
    Move(res, step)
}
