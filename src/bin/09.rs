use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;

    Some(
        (0..height)
            .cartesian_product(0..width)
            .filter_map(|(x, y)| find_low_point_score(&grid, x, y, height, width))
            .map(|x| x as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let _grid = parse_grid(input);
    None
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect::<Vec<_>>())
        .collect()
}

fn find_low_point_score(
    grid: &[Vec<u8>],
    x: i32,
    y: i32,
    x_bound: i32,
    y_bound: i32,
) -> Option<u8> {
    let cur = grid[x as usize][y as usize];

    if (x - 1) >= 0 && grid[x as usize - 1][y as usize] <= cur {
        return None;
    }
    if (x + 1) < x_bound && grid[x as usize + 1][y as usize] <= cur {
        return None;
    }
    if (y - 1) >= 0 && grid[x as usize][y as usize - 1] <= cur {
        return None;
    }
    if (y + 1) < y_bound && grid[x as usize][y as usize + 1] <= cur {
        return None;
    }

    Some(cur + 1)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1134));
    }
}
