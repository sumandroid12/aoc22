
use bit_set::BitSet;

enum Rock {
    R1,
    R2,
    R3,
    R4,
    R5,
}
#[derive(Debug, PartialEq, Eq)]
struct RockPoints(Vec<(i32, i32)>);
impl From<&Rock> for RockPoints {
    fn from(r: &Rock) -> Self {
        match r {
            Rock::R1 => RockPoints(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            Rock::R2 => RockPoints(vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)]),
            Rock::R3 => RockPoints(vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]),
            Rock::R4 => RockPoints(vec![(0, 3), (0, 2), (0, 1), (0, 0)]),
            Rock::R5 => RockPoints(vec![(0, 1), (1, 1), (0, 0), (1, 0)]),
        }
    }
}
impl RockPoints {
    fn set_start_position(&mut self, pos: (i32, i32)) {
        self.0.iter_mut().for_each(|r| {
            r.0 += pos.0;
            r.1 += pos.1;
        })
    }
    fn push_right(&mut self, room: &Room) {
        let changed: Vec<_> = self.0.iter().map(|p| (p.0 + 1, p.1)).collect();
        if changed.iter().any(|p| p.0 > 6) {
            return;
        }
        if changed
            .iter()
            .any(|p| room.contains_rock(p.0 as usize, p.1 as usize))
        {
            return;
        }
        self.0 = changed;
    }
    fn push_left(&mut self, room: &Room) {
        let changed: Vec<_> = self.0.iter().map(|p| (p.0 - 1, p.1)).collect();
        if changed.iter().any(|p| p.0 < 0) {
            return;
        }
        if changed
            .iter()
            .any(|p| room.contains_rock(p.0 as usize, p.1 as usize))
        {
            return;
        }
        self.0 = changed;
    }
    /**
    Try dropping the rock by 1 unit
    # Errors
    This function will return an error if collision occurs
    On collision return topmost point from new room with jjjj
    */
    fn try_drop(mut self, room: &mut Room) -> Result<RockPoints, usize> {
        let changed: Vec<_> = self.0.iter().map(|rp| (rp.0, rp.1 - 1)).collect();
        if changed
            .iter()
            .any(|p| room.contains_rock(p.0 as usize, p.1 as usize))
        {
            // solidify and then stop
            return Err(room.top(self));
        }
        self.0 = changed;
        Ok(self)
    }
}

struct Room {
    bitset: BitSet,
    width: usize,
}

impl Room {
    fn new() -> Self {
        Room {
            bitset: BitSet::new(),
            width: 7,
        }
    }
    fn insert_rock(&mut self, i: usize, j: usize) -> bool {
        self.bitset.insert(j * self.width + i)
    }
    fn contains_rock(&self, i: usize, j: usize) -> bool {
        self.bitset.contains(j * self.width + i)
    }
    /// Inserts a new rock and returns the top most point
    fn top(&mut self, new_rock: RockPoints) -> usize {
        new_rock.0.iter().for_each(|(i, j)| {
            self.insert_rock(*i as usize, *j as usize);
        });
        new_rock.0.iter().max().unwrap().0 as usize
    }
}

struct RoomState {
    room: Room,
    spawn_pt: (i32, i32),
    rock: char,
    jet: char,
}
use Rock::*;
impl RoomState {
    const ALL_ROCKS: [Rock; 5] = [R1, R2, R3, R4, R5];
}

fn main() {
    const ALL_ROCKS: [Rock; 5] = [R1, R2, R3, R4, R5];

    let input = include_str!("day17.in");
    let mut jet = input.chars().cycle();
    let mut next_rock = ALL_ROCKS.iter().cycle();
    let mut room = Room::new();
    (0..2022).for_each(|_| {});
}
