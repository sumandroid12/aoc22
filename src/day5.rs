#[test]
fn main() {
    let input = include_str!("day5.in");
    let mut reader = input.lines().skip(10);
    let mut crates = vec![];
    crates.push(vec!['B', 'Z', 'T']);
    crates.push(vec!['V', 'H', 'T', 'D', 'N']);
    crates.push(vec!['B', 'F', 'M', 'D']);
    crates.push(vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L']);
    crates.push(vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M']);
    crates.push(vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S']);
    crates.push(vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W']);
    crates.push(vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M']);
    crates.push(vec!['M', 'Q', 'L', 'F', 'D', 'S']);

    while let Some(inst) = reader.next() {
        let inst = inst.split(' ');
        let qt = inst.clone().nth(1).unwrap().parse::<u32>().unwrap();
        let from_crate = inst.clone().nth(3).unwrap().parse::<usize>().unwrap();
        let to_crate = inst.clone().nth(5).unwrap().parse::<usize>().unwrap();
        let mut temp_stack = vec![];
        for _ in 0..qt {
            let pop = crates[from_crate - 1].pop().unwrap();
            temp_stack.push(pop);
        }
        for _ in 0..qt {
            let pop = temp_stack.pop().unwrap();
            crates[to_crate - 1].push(pop);
        }
    }
    for i in 0..crates.len() {
        println!("{}", crates[i].last().unwrap());
    }
}
