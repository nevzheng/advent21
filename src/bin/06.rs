use std::collections::HashMap;

fn read_input(input: &str) -> Option<Vec<usize>> {
    Some(
        input
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>(),
    )
}

pub fn simulate(input: &str, days: usize) -> Option<usize> {
    let mut buckets: HashMap<usize, usize> = HashMap::new();
    let fish = read_input(input)?;

    for f in fish {
        *buckets.entry(f).or_insert(0) +=1;
    }

    for d in 0..days {
        if let Some(todays_fish) = buckets.get(&d).cloned(){
            // Move all parent fish into future.
            *buckets.entry(d+7).or_insert(0) += todays_fish;
            // Spawn the new fish
            *buckets.entry(d+7+2).or_insert(0) += todays_fish;
            // Delete todays bucket to avoid double counting.
            buckets.remove(&d);
        }
    }

    Some(buckets.into_values().sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    println!("{}", simulate(input, 80).unwrap());
    println!("{}", simulate(input, 256).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(simulate(&input, 18), Some(26));
        assert_eq!(simulate(&input, 80), Some(5934));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(simulate(&input, 256), Some(26984457539));
    }
}
