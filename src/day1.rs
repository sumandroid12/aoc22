#[test]
fn max_calorie() {
    let input = include_str!("day1.in");
    let mut max_cal = 0;
    let mut cur_cal = 0;
    let mut calories: Vec<i32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            max_cal = max_cal.max(cur_cal);
            calories.push(cur_cal);
            cur_cal = 0;
            continue;
        }
        cur_cal += line.parse::<i32>().unwrap();
    }
    calories.sort();
    let sum: i32 = calories.iter().rev().take(3).sum();
    println!("{}", sum);
}
