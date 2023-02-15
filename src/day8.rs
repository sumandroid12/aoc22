#[test]
fn visible_trees() {
    let input = include_str!("day8.in");
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    let (m, n) = (forest.len(), forest[0].len());
    let mut visible = vec![vec![false; n]; m];
    for i in 0..m {
        let mut prev_l = -1;
        let mut prev_r = -1;
        let mut prev_t = -1;
        let mut prev_b = -1;
        for j in 0..n {
            let left = forest[i][j];
            let right = forest[i][n - 1 - j];
            let top = forest[j][i];
            let bot = forest[n - 1 - j][i];
            if left > prev_l {
                visible[i][j] = true;
                prev_l = left;
            }
            if right > prev_r {
                visible[i][n - 1 - j] = true;
                prev_r = right;
            }
            if top > prev_t {
                visible[j][i] = true;
                prev_t = top;
            }
            if bot > prev_b {
                visible[n - 1 - j][i] = true;
                prev_b = bot;
            }
        }
    }
    let visible_count = visible.iter().flatten().filter(|&&v| v).count();
    dbg!(visible_count);
}
fn main() {
    scenic_score()
}
// #[test]
fn scenic_score() {
    // O(n^2*h) runtime.
    let input = include_str!("day8.in");
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    let (m, n) = (forest.len(), forest[0].len());

    let mut scores = vec![vec![(0, 0, 0, 0); n]; m];

    for i in 0..n {
        let mut last_seen_pos_l = vec![0; 10];
        let mut last_seen_pos_r = vec![n - 1; 10];
        let mut last_seen_pos_t = vec![0; 10];
        let mut last_seen_pos_b = vec![n - 1; 10];
        for j in 0..n {
            // left
            let height = forest[i][j] as usize;
            let score_l = j - last_seen_pos_l[height..].iter().max().unwrap().clone();
            scores[i][j].0 = score_l;
            last_seen_pos_l[height] = j;
            // right
            let height = forest[i][n - 1 - j] as usize;
            let score_r = last_seen_pos_r[height..].iter().min().unwrap().clone() - (n - 1 - j);
            scores[i][n - 1 - j].1 = score_r;
            last_seen_pos_r[height] = n - 1 - j;
            // top
            let height = forest[j][i] as usize;
            let score_t = j - last_seen_pos_t[height..].iter().max().unwrap().clone();
            scores[j][i].2 = score_t;
            last_seen_pos_t[height] = j;
            // bottom
            let height = forest[n - 1 - j][i] as usize;
            let score_b = last_seen_pos_b[height..].iter().min().unwrap().clone() - (n - 1 - j);
            scores[n - 1 - j][i].3 = score_b;
            last_seen_pos_b[height] = n - 1 - j;
        }
    }

    let maxscore = scores
        .iter()
        .flatten()
        .map(|s| s.0 * s.1 * s.2 * s.3)
        .max()
        .unwrap();
    dbg!(maxscore);
}
