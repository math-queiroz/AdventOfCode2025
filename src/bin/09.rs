use itertools::Itertools;

#[aoc::day(09, "Movie Theater")]
#[aoc::asserts("4715966250", "0")]
fn main(input: String, line_ending: &str) -> (i64, i64) {
    let tiles: Vec<(i64, i64)> = input
        .split(line_ending)
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i64>().expect("bad input"))
                .collect_tuple()
                .expect("bad input")
        })
        .collect();

    let mut max_area = 0;
    for (x1, y1) in &tiles {
        for (x2, y2) in &tiles {
            let (dx, dy) = ((x2-x1).max(x1-x2) + 1, (y1-y2).max(y2-y1) + 1);
            max_area = max_area.max(dx*dy);
        }
    }

    (max_area, 0)
}
