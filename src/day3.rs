use std::collections::HashSet;

#[test]
fn rucksacks() {
    let input = include_str!("day3.in");
    let mut common_items = vec![];
    for rsk in input.lines() {
        let sz = rsk.len();
        let (c1, c2) = rsk.split_at(sz / 2);
        let mut b1 = HashSet::new();
        let mut b2 = HashSet::new();
        for (i1, i2) in c1.chars().zip(c2.chars()) {
            b2.insert(i2);
            b1.insert(i1);
            if b2.contains(&i1) {
                common_items.push(i1);
                break;
            }
            if b1.contains(&i2) {
                common_items.push(i2);
                break;
            }
        }
    }
    let priority_sum: usize = common_items
        .iter()
        .map(|c| match c.is_lowercase() {
            true => *c as usize - 'a' as usize + 1,
            false => *c as usize - 'A' as usize + 27,
        })
        .sum();
    println!("{}", priority_sum);
}

#[test]
fn rucksack_badge() {
    let input = include_str!("day3.in");
    let mut it = input.lines();
    let mut badges = vec![];
    while let Some(sack1) = it.next() {
        let sack1 = sack1.chars().collect::<HashSet<_>>();
        let sack2 = it.next().unwrap().chars().collect::<HashSet<_>>();
        let sack3 = it.next().unwrap().chars().collect::<HashSet<_>>();
        let badge = *sack1
            .iter()
            .filter(|c| sack2.contains(c))
            .filter(|c| sack3.contains(c))
            .next()
            .unwrap();
        badges.push(badge)
    }
    let priority_sum: usize = badges
        .iter()
        .map(|c| match c.is_lowercase() {
            true => *c as usize - 'a' as usize + 1,
            false => *c as usize - 'A' as usize + 27,
        })
        .sum();
    println!("{}", priority_sum);
}
