#[aoc::day(06, "Trash Compactor")]
#[aoc::asserts("4412382293768", "7858808482092")]
fn main(input: String, line_ending: &str) -> (i64, i64) {
    let parsed_data: Vec<&str> = input.split(line_ending).collect();
    let column_count = parsed_data[0].split_whitespace().count();
    let line_count = parsed_data.len() - 1;

    let operations: Vec<&str> = parsed_data[line_count].split_whitespace().collect();
    let mut problems: Vec<Vec<&str>> = vec![vec![""; line_count]; column_count];

    let mut column_counter = 0;
    let mut last_sep_idx = 0;
    for i in 0..parsed_data[0].len() {
        if parsed_data[..line_count]
            .iter()
            .all(|row| row.chars().nth(i).unwrap() == ' ')
        {
            for l in 0..line_count {
                problems[column_counter][l] = &parsed_data[l][last_sep_idx..i];
            }
            last_sep_idx = i + 1;
            column_counter += 1;
        };
    }

    for l in 0..line_count {
        problems[column_counter][l] = &parsed_data[l][last_sep_idx..];
    }

    let p1: Vec<Vec<i64>> = problems.iter().map(|s| s.iter().map(|n| n.trim().parse::<i64>().expect("bad input")).collect()).collect();
    let p2: Vec<Vec<i64>> = problems.into_iter().map(|arr| {
        let mut tmp = Vec::with_capacity(arr[0].len());
        for i in (0..arr[0].len()).rev() {
            let mut pow = 0;
            let mut n = 0;
            for l in (0..arr.len()).rev() {
                let char = arr[l].chars().nth(i).unwrap();
                if char == ' ' { continue }
                n += (char as u8 - b'0') as i64 * 10_i64.pow(pow as u32);
                pow += 1;
            }
            tmp.push(n);
        }
        tmp
    }).collect();

    let results: Vec<i64> = vec![p1, p2]
        .iter()
        .map(|p| {
            operations
                .iter()
                .enumerate()
                .map(|(i, op)| match op {
                    &"*" => p[i].iter().fold(1, |acc, n| acc * n), 
                    &"+" => p[i].iter().fold(0, |acc, n| acc + n),
                    _ => unreachable!("bad input"),
                })
                .fold(0, |acc, n| acc + n)
        })
        .collect();

    (results[0], results[1])
