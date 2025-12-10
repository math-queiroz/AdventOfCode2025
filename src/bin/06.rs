#[aoc::day(906, "Trash Compactor")]
#[aoc::asserts("4412382293768", "0")]
fn main(input: String, line_ending: &str) -> (i64, i64) {
    let parsed_data: Vec<&str> = input.split(line_ending).collect();
    let column_count = parsed_data[0].split_whitespace().count();
    let line_count = parsed_data.len() - 1;

    let operations: Vec<&str> = parsed_data[line_count].split_whitespace().collect();
    let mut problems: Vec<Vec<&str>> = vec![vec![""; line_count]; column_count];

    for c in 0..line_count {
        /* if parsed_data[..line_count].iter().all(|row| row.char_at(c) == ' ') {
            problems[column_index][i] = &parsed_data[i][last_sep_idx..(if j == last_char_index { j } else { j - 1 })];
        }; 
        last_sep_idx = j + 1;
        column_index += 1; */
    }

    println!("{:?}",problems);

    let results: Vec<i64> = operations
        .iter()
        .enumerate()
        .map(|(i, op)| match op {
            &"*" => problems[i].iter().map(|n| { println!("{n}"); n.parse::<i64>().expect("bad input") }).fold(1, |acc, n| acc * n),
            &"+" => problems[i].iter().map(|n| { println!("{n}"); n.parse::<i64>().expect("bad input") }).fold(0, |acc, n| acc + n),
            _ => unreachable!("bad input"),
        })
        .collect();

    (results.iter().sum(), 0)
}
