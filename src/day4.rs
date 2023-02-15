#[test]
fn pair_overlap() {
    let input = include_str!("day4.in");
    let mut count = 0;
    for pairs in input.lines() {
        let (p1, p2) = pairs.split_once(',').unwrap();
        let (p1s, p1e) = p1.split_once('-').unwrap();
        let p1 = (p1s.parse::<i32>().unwrap(), p1e.parse::<i32>().unwrap());
        let (p2s, p2e) = p2.split_once('-').unwrap();
        let p2 = (p2s.parse::<i32>().unwrap(), p2e.parse::<i32>().unwrap());
        if (p2.0 <= p1.0 && p1.0 <= p2.1)
            || (p2.0 <= p1.1 && p1.1 <= p2.1)
            || (p2.0 >= p1.0 && p1.1 >= p2.1)
        {
            // dbg!(p1,p2);
            count += 1;
        }
    }
    println!("{}", count);
}
