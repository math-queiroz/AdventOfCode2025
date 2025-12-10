use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

fn insert_interval(intervals: &mut Vec<Interval>, new: Interval) {
    intervals.push(new);
    intervals.sort_by_key(|r| r.start);

    let mut merged: Vec<Interval> = Vec::new();

    for i in intervals.drain(..) {
        if let Some(last) = merged.last_mut() {
            if i.start <= last.end {
                last.end = last.end.max(i.end);
            } else {
                merged.push(i);
            }
        } else {
            merged.push(i);
        }
    }

    *intervals = merged;
}

#[aoc::day(05, "Cafeteria")]
#[aoc::asserts("615", "353716783056994")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let parsed_data: Vec<Vec<&str>> = input
        .split(&line_ending.repeat(2))
        .map(|i| i.split(line_ending).collect())
        .collect();

    let mut intervals: Vec<Interval> = vec![];
    parsed_data[0].iter().for_each(|l| {
        let (start, end) = l
            .split('-')
            .map(|n| n.parse::<i64>().expect("bad input"))
            .collect_tuple()
            .expect("bad input");
        insert_interval(&mut intervals, Interval { start, end });
    });

    let ids: Vec<i64> = parsed_data[1]
        .iter()
        .map(|l| l.parse().expect("bad input"))
        .collect();

    let fresh: Vec<_> = ids
        .iter()
        .filter(|&&id| {
            if let Some(i) = intervals.iter().position(|i| i.end >= id) {
                id >= intervals[i].start
            } else {
                false
            }
        })
        .collect();

    let fresh_domain_count = intervals
        .iter()
        .fold(0_usize, |acc, i| acc + i.end as usize - i.start as usize + 1);

    (fresh.len(), fresh_domain_count)
}
