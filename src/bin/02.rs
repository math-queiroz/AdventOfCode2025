use itertools::{self, Itertools};

#[aoc::day(02, "Gift Shop")]
#[aoc::asserts("56660955519", "79183223243")]
fn main(input: String, _line_ending: &str) -> (usize, usize) {
    let ranges = input.split(',');
    let mut invalid_id_acc = (0, 0);
    for range in ranges {
        let (start, end) = range
            .split('-')
            .map(|n| n.parse::<usize>().expect("bad input"))
            .collect_tuple()
            .expect("bad input");
        invalid_id_acc.0 += (start..=end).filter(is_single_rep).sum::<usize>();
        invalid_id_acc.1 += (start..=end).filter(has_multi_rep).sum::<usize>();
    }
    invalid_id_acc
}

fn is_single_rep(num: &usize) -> bool {
    let digit_count = num.checked_ilog10().expect("bad input") + 1;
    let half_factor = 10_usize.pow(digit_count / 2);
    (num / half_factor) == (num % half_factor)
}

fn has_multi_rep(num: &usize) -> bool {
    let chars = num.to_string().chars().collect::<Vec<_>>();
    for i in 0..chars.len() / 2 {
        if chars[i + 1..].chunks(i + 1).all(|c| c == &chars[..=i]) {
            return true;
        }
    }
    false
}
