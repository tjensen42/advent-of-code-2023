use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let universe = universe_from_str(input);
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, galaxy)| if *galaxy > 0 { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        .sum()
}

fn universe_from_str(s: &str) -> Vec<Vec<usize>> {
    let mut counter = 1;
    let mut universe = Vec::new();
    for (i, line) in s.lines().enumerate() {
        universe.push(vec![0; line.chars().count()]);
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                universe[i][j] = counter;
                counter += 1;
            }
        }
        assert_eq!(universe[i].len(), universe[0].len());
    }

    let mut inserted = 0;
    for i in 0..universe.len() {
        let pos = i + inserted;
        if universe[pos].iter().all(|&x| x == 0) {
            universe.insert(pos + 1, universe[pos].clone());
            inserted += 1;
        }
    }

    let mut inserted = 0;
    for i in 0..universe[0].len() {
        let pos = i + inserted;
        if (0..universe.len()).all(|j| universe[j][pos] == 0) {
            universe.iter_mut().for_each(|row| row.insert(pos + 1, 0));
            inserted += 1;
        }
    }

    universe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 374);
    }
}
