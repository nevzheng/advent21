#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use itertools::Itertools;

lazy_static! {
    static ref UNIQUE_LENGTH_DIGITS: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        // Two segments is a "1".
        m.insert(2, 1);
        // Three Segments is a "7";
        m.insert(3,7);
        // Four Segments is a "4".
        m.insert(4,4);
        // Seven Segments is an "8".
        m.insert(7,8);
        m
    };
}

fn read_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .into_iter()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(signal_str, output_str)| {
            (
                signal_str
                    .split_whitespace()
                    .map(|s| s.chars().sorted().collect::<String>())
                    .collect::<Vec<String>>(),
                output_str
                    .split_whitespace()
                    .map(|s| s.chars().sorted().collect::<String>())
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

fn contains(src: &str, query: &str) -> bool {
    query.chars().all(|c| src.contains(c))
}

fn decode_line(input: &[String], output: &[String]) -> usize {
    let mut decoder: HashMap<String, char> = HashMap::new();

    // 1, 4, 7, 8 have a unique number of segments.
    let one = input.iter().find(|s| s.len() == 2).unwrap();
    decoder.insert(one.to_owned(), '1');

    let seven = input.iter().find(|s| s.len() == 3).unwrap();
    decoder.insert(seven.to_owned(), '7');

    let four = input.iter().find(|s| s.len() == 4).unwrap();
    decoder.insert(four.to_owned(), '4');

    let eight = input.iter().find(|s| s.len() == 7).unwrap();
    decoder.insert(eight.to_owned(), '8');

    // 2, 3, 5 have 5 segments.
    // A 3 has 5 segments and contains all segments 7 has. AND 2 & 5, DO NOT have all segments have.
    let three = input
        .iter()
        .find(|s| s.len() == 5 && contains(s, seven))
        .unwrap();
    decoder.insert(three.to_owned(), '3');

    // 0, 6, 9 have 6 segments. 6 DOES contains all segments 7 has, while 0, 9 do not.
    let six = input
        .iter()
        .find(|s| s.len() == 6 && !contains(s, seven))
        .unwrap();
    decoder.insert(six.to_owned(), '6');

    // Segments B & E.
    let segments_be = eight
        .chars()
        .filter(|c| !three.chars().contains(c))
        .collect::<String>();

    // 0, 6, 9 have 6 segments. We deduced 6 && 0 contains BE while 9 does not.
    let zero = input
        .iter()
        .find(|s| s.len() == 6 && s != &six && contains(s, &segments_be))
        .unwrap();
    decoder.insert(zero.to_owned(), '0');

    // The remaining 6-len character digit is 9.
    let nine = input
        .iter()
        .find(|s| s.len() == 6 && s != &six && s != &zero)
        .unwrap();
    decoder.insert(nine.to_owned(), '9');

    // Segments EG = 8 diff 4 diff 7.
    let segments_eg = eight
        .chars()
        .filter(|c| !four.chars().contains(c))
        .filter(|c| !seven.chars().contains(c))
        .collect::<String>();

    // Two has 5 segments, is not equal to 3, and contains EG, while 5 does not.
    let two = input
        .iter()
        .find(|s| s.len() == 5 && s != &three && contains(s, &segments_eg))
        .unwrap();
    decoder.insert(two.to_owned(), '2');

    let five = input
        .iter()
        .find(|s| s.len() == 5 && s != &three && s != &two)
        .unwrap();
    decoder.insert(five.to_owned(), '5');

    output
        .iter()
        .map(|code| decoder.get(code).unwrap())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let inputs_outputs = read_input(input);

    Some(
        inputs_outputs
            .iter()
            .map(|(_, outputs)| {
                outputs
                    .iter()
                    .filter_map(|digit| {
                        UNIQUE_LENGTH_DIGITS.contains_key(&digit.len()).then_some(1)
                    })
                    .count()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    // Each String is sorted.
    let inputs_outputs = read_input(input);

    Some(inputs_outputs.iter().map(|(i, o)| decode_line(i, o)).sum())
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
