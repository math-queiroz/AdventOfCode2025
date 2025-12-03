#[aoc::day(03, "Lobby")]
#[aoc::asserts("17408", "172740584266849")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let parsed_data = input
        .split(line_ending)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (get_joltage(&parsed_data, 2), get_joltage(&parsed_data, 12))
}

fn get_joltage(battery_banks: &Vec<Vec<char>>, digits: usize) -> usize {
    battery_banks
        .iter()
        .fold(0, |acc, line| acc + find_biggest(line, digits))
}

fn find_biggest(str: &[char], n: usize) -> usize {
    let (mut biggest, mut start_from) = (0, 0);
    for offset in (0..n).rev() {
        let mut max_idx = start_from;
        for i in start_from..str.len() - offset {
            if str[i] > str[max_idx] {
                max_idx = i
            }
        }
        start_from = max_idx + 1;
        biggest += 10_usize.pow(offset as u32) * (str[max_idx] as u8 - b'0') as usize;
    }
    biggest
}
