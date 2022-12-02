pub fn part_one(input: &str) -> Option<u32> {
    let depths: Vec<u32> = input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut n_rises = 0;

    for w in depths.windows(2) {
        if w[0] < w[1] {
            n_rises += 1;
        }
    }

    Some(n_rises)
}

pub fn part_two(input: &str) -> Option<u32> {
    let depths: Vec<u32> = input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut last_sum = u32::MAX;
    let mut n_greater = 0;

    for w in depths.windows(3) {
        if last_sum < w.iter().sum() {
            n_greater += 1;
        }
        last_sum = w.iter().sum();
    }

    Some(n_greater)
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
