pub fn part_one(input: &str) -> Option<i32> {
    // Parse into Vec
    let crab_pos: Vec<_> = input
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    // Get largest position
    let max = crab_pos.iter().max()?.to_owned();

    let mut costs = vec![0; (max + 1) as usize];

    // for each crab
    for p in &crab_pos {
        // for each possible position the crab could move to
        for i in 0..=max {
            // add the cost of moving p to i
            costs[i as usize] += (i - p).abs();
        }
    }
    // Return the min cost
    costs.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<i32> {
    // Parse into Vec
    let crab_pos: Vec<_> = input
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    // Get largest position
    let max = crab_pos.iter().max()?.to_owned();

    let mut costs = vec![0; (max + 1) as usize];

    // for each crab
    for p in &crab_pos {
        // for each possible position the crab could move to
        for i in 0..=max {
            let dist = (i - p).abs();
            costs[i as usize] += ((dist) * (dist + 1)) / 2;
        }
    }
    // Return the min cost
    costs.iter().min().copied()
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(206));
    }
}
