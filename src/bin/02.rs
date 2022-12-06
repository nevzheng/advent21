pub fn part_one(input: &str) -> Option<u32> {
    let mut x = 0;
    let mut y = 0;

    let commands: Vec<(&str, u32)> = input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(x, y_s)| {
            let y_i = y_s.parse::<u32>().ok()?;
            Some((x, y_i))
        })
        .collect();

    for (direction, count) in commands {
        match direction {
            "forward" => x += count,
            "up" => y -= count,
            "down" => y += count,
            _ => panic!(),
        }
    }

    Some(x * y)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    let commands: Vec<(&str, u32)> = input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(x, y_s)| {
            let y_i = y_s.parse::<u32>().ok()?;
            Some((x, y_i))
        })
        .collect();

    for (direction, count) in commands {
        match direction {
            "forward" => {
                x += count;
                y += aim * count;
            }
            "up" => aim -= count,
            "down" => aim += count,
            _ => panic!(),
        }
    }

    Some(x * y)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
