fn most_common_bit(nums: &Vec<u32>, idx: usize) -> Option<u32> {
    let mut n_zeros = 0;
    let mut n_ones = 0;

    let mask = 1 << idx;
    for num in nums {
        if (num & mask) == mask {
            n_ones += 1
        } else {
            n_zeros += 1
        }
    }

    if n_ones == n_zeros {
        None
    } else if n_ones < n_zeros {
        Some(0)
    } else {
        Some(1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().nth(1)?.len();
    let nums = input
        .lines()
        .filter_map(|x| u32::from_str_radix(x, 2).ok())
        .collect::<Vec<u32>>();

    let mut eps = 0;
    for i in 0..=width {
        eps |= most_common_bit(&nums, i)? << i;
    }
    // Use xor to flip the bits, but only the first `width` bits.
    let gam = eps ^ ((1 << width) - 1);

    Some(eps * gam)
}

fn find_oxygen_rating(nums: &Vec<u32>, width: usize) -> u32 {
    let mut current_nums = nums.clone();

    for idx in (0..width).rev() {
        // oxygen uses 1 as tie_breaker
        let most_common_digit = most_common_bit(&current_nums, idx).unwrap_or(1);
        let next_nums: Vec<u32> = current_nums
            .into_iter()
            .filter(|x| (x & (1 << idx)) == (most_common_digit << idx))
            .collect();

        current_nums = next_nums.to_owned();

        if current_nums.len() == 1 {
            break;
        }
    }

    *current_nums.first().unwrap()
}

fn find_scrubber_rating(nums: &Vec<u32>, width: usize) -> u32 {
    let mut current_nums = nums.clone();

    for idx in (0..width).rev() {
        // scrubber uses 0 as tie breaker. we need to invert it though so use 1.
        let most_common_digit = most_common_bit(&current_nums, idx).unwrap_or(1);
        let least_common_digit = 1 - most_common_digit;
        let next_nums: Vec<u32> = current_nums
            .into_iter()
            .filter(|x| (x & (1 << idx)) == (least_common_digit << idx))
            .collect();

        current_nums = next_nums.to_owned();

        if current_nums.len() == 1 {
            break;
        }
    }

    *current_nums.first().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().nth(1)?.len();
    let nums = input
        .lines()
        .filter_map(|x| u32::from_str_radix(x, 2).ok())
        .collect::<Vec<u32>>();

    let oxy = find_oxygen_rating(&nums, width);
    let scrub = find_scrubber_rating(&nums, width);

    Some(oxy * scrub)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(230));
    }
}
