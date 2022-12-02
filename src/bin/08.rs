fn read_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    // Split into lines.
    let lines: Vec<&str> = input.lines().collect();
    // Split Each Line into the input_str, and output_str sections.
    let split_lines: Vec<(&str, &str)> = lines
        .into_iter()
        .map(|l| l.split_once(" | ").unwrap())
        .collect();
    // Split each input_str, output_str into vectors.
    split_lines
        .into_iter()
        .map(|(signal_str, output_str)| {
            (
                signal_str.split_whitespace().collect::<Vec<&str>>(),
                output_str.split_whitespace().collect::<Vec<&str>>(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let inputs_outputs = read_input(input);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(61229));
    }
}
