#[aoc::day(01, "Secret Entrance")]
#[aoc::asserts("969", "5887")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let parsed_data = input.split(line_ending);
    let (mut dial, mut zero_count, mut flip_count): (isize, usize, usize) = (50, 0, 0);
    for line in parsed_data {
        let (dir, shift) = line.split_at(1);
        let shift = shift.parse::<isize>()
            .map(|n| if dir == "L" { -n } else { n })
            .expect("bad input");
        let target_dial = dial + shift;

        flip_count += target_dial.abs() as usize / 100;
        if dial != 0 && target_dial <= 0 { flip_count += 1 }

        dial = target_dial.rem_euclid(100);
        if dial == 0 { zero_count += 1 }
    }
    (zero_count, flip_count)
}
