#[test]
fn total_score() {
    let input = include_str!("day2.in");
    let mut score = 0;
    for round in input.lines() {
        let mut c = round.split(" ");
        let opp = c.next().unwrap();
        let me = c.next().unwrap();
        score += calculate_score(opp, me);
    }
    println!("{}", score);
}

fn calculate_score(opp: &str, me: &str) -> i32 {
    return match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    } + match (opp, me) {
        ("A", "Y") | ("C", "Z") | ("B", "X") => 1,
        ("B", "Y") | ("A", "Z") | ("C", "X") => 2,
        ("C", "Y") | ("B", "Z") | ("A", "X") => 3,
        _ => panic!(),
    };
}
