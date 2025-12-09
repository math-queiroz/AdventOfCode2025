#[aoc::day(07, "Laboratories")]
#[aoc::asserts("1703", "171692855075500")]
fn main(input: String, line_ending: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.split(line_ending).collect();
    let mut beams_arrays = vec![vec![0; lines[0].len()]; 2];

    let start_index = lines[0].chars().position(|c| c == 'S').expect("bad input");
    beams_arrays[0][start_index] = 1;
    let mut split_counter = 0;

    for i in 1..lines[0].len() {
        let prev_index = (i % 2 == 0) as usize;
        let curr_index = (i % 2 != 0) as usize;
        beams_arrays[curr_index].fill(0);
        for (j, c) in lines[i].chars().enumerate() {
            if beams_arrays[prev_index][j] > 0 {
                if c == '^' {
                    split_counter += 1;
                    beams_arrays[curr_index][j - 1] =
                        beams_arrays[prev_index][j] + beams_arrays[curr_index][j - 1];
                    beams_arrays[curr_index][j + 1] =
                        beams_arrays[prev_index][j] + beams_arrays[curr_index][j + 1];
                } else {
                    beams_arrays[curr_index][j] += beams_arrays[prev_index][j];
                }
            }
        }
    }

    let reality_count = beams_arrays[(lines.len() % 2 != 0) as usize].iter().sum();

    (split_counter, reality_count)
}
