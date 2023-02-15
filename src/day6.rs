use std::collections::HashSet;

#[test]
fn start_of_packet() {
    let input = include_str!("day6.in");
    let stream = input.chars().collect::<Vec<_>>();
    for (id, w) in stream.windows(14).enumerate() {
        if w.iter().collect::<HashSet<_>>().len() == 14 {
            println!("{}", id + 14);
            break;
        }
    }
}
