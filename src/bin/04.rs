use std::collections::VecDeque;

#[aoc::day(04, "Printing Department")]
#[aoc::asserts("1384", "0")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let parsed_data = input.split(line_ending);
    let grid: Vec<Vec<char>> = parsed_data.map(|l| l.chars().collect()).collect();
    let mut adj_grid: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut rolls: VecDeque<(usize, usize)> = VecDeque::new();

    let neighbors: Vec<(isize, isize)> = (-1..2)
        .map(|dy| (-1..2).map(move |dx| (dx, dy)))
        .flatten()
        .filter(|(x, y)| !(*y == 0 && *x == 0))
        .collect();

    let (h, w) = (grid.len(), grid[0].len());
    for (x, y) in (0..h).map(|y| (0..w).map(move |x| (x, y))).flatten() {
        if grid[y][x] == '@' {
            rolls.push_front((x, y));
            for (dx, dy) in neighbors.clone() {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= h as isize {
                    continue;
                }
                adj_grid[ny as usize][nx as usize] += 1
            }
        }
    }

    let initial_rolls = rolls.len();
    let mut removables = rolls
        .iter()
        .enumerate()
        .filter_map(|(i, (x, y))| if adj_grid[*y][*x] < 4 { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let first_removables = removables.len();

    while removables.len() > 0 {
        for roll_idx in removables.iter().rev() {
            let roll = rolls[*roll_idx];
            for (dx, dy) in neighbors.clone() {
                let (nx, ny) = (roll.0 as isize + dx, roll.1 as isize + dy);
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= h as isize {
                    continue;
                }
                adj_grid[ny as usize][nx as usize] -= 1;
            }
            rolls.swap_remove_back(*roll_idx);
        }
        removables = rolls
            .iter()
            .enumerate()
            .filter_map(|(i, (x, y))| if adj_grid[*y][*x] < 4 { Some(i) } else { None })
            .collect::<Vec<usize>>();
    }

    (first_removables, (initial_rolls - rolls.len()))
}
