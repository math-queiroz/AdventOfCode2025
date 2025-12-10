use hashbrown::HashSet;
use itertools::Itertools;

type Coordinates = (i64, i64, i64);

const LINK_COUNT: usize = 1000;

#[aoc::day(08, "Playground")]
#[aoc::asserts("4896 24455520 80109", "0")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let points: Vec<Coordinates> = input
        .split(line_ending)
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().expect("bad input"))
                .collect_tuple()
                .expect("bad input")
        })
        .collect();

    let mut distances_sqr: Vec<(Coordinates, Coordinates, i64)> = vec![];

    for p1 in &points {
        for p2 in &points {
            if p1 == p2 {
                continue;
            }
            let distance_sqr = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2);
            distances_sqr.push((*p1, *p2, distance_sqr));
        }
    }

    distances_sqr.sort_by_key(|v| v.2);
    distances_sqr = distances_sqr
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();

    // println!("{:?}", distances_sqr);

    let mut circuits: Vec<HashSet<Coordinates>> = vec![];
    let mut link_count = 0;
    let mut distances_iter = distances_sqr.iter();
    while link_count < LINK_COUNT {
        let Some((p1, p2, _)) = distances_iter.next() else {
            break;
        };

        let mut found_in = vec![];
        let mut found_coords = vec![];
        for i in 0..circuits.len() {
            let (has_p1, has_p2) = (circuits[i].contains(p1), circuits[i].contains(p2));
            if has_p1 || has_p2 {
                if has_p1 && has_p2 {
                    link_count -= 1;
                } else if circuits[i].contains(p2) {
                    found_coords.push(p2);
                    found_in.push(i);
                } else {
                    found_coords.push(p1);
                    found_in.push(i);
                }
            }
        }

        if found_in.len() == 2 {
            for node in circuits[found_in[1]].clone() {
                circuits[found_in[0]].insert(node);
            }
            circuits.remove(found_in[1]);
        } else if found_in.len() == 1 {
            if found_coords[0] == p1 {
                circuits[found_in[0]].insert(*p2);
            } else {
                circuits[found_in[0]].insert(*p1);
            }
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(*p1);
            new_set.insert(*p2);
            circuits.push(new_set);
        }

        link_count += 1;
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    println!(
        "{:?}",
        &circuits[..3]
            .iter()
            .map(|s| s.len())
            .collect::<Vec<usize>>()
    );

    (circuits[..3].iter().fold(1, |acc, set| acc * set.len()), 0)
}
