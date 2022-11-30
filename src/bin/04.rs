const WIDTH: usize = 5;
const HEIGHT: usize = 5;
type Calls = Vec<u32>;
type Board = [[u32; WIDTH]; HEIGHT];
type Marks = [[bool; WIDTH]; HEIGHT];

struct BingoBoard {
    board: Board,
    marks: Marks,
    win: bool,
}

fn check_row(marks: &Marks, row_id: usize) -> bool {}
fn check_col(marks: &Marks, row_id: usize) -> bool {}
fn check_forward_slash(marks: &Marks) -> bool {}
fn check_back_slash(marks: &Marks) -> bool {}

impl BingoBoard {
    fn call(&mut self, num: u32) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if self.board[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn check(&self) -> bool {}

    fn score(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if !self.marks[i][j] {
                    sum += self.board[i][j]
                }
            }
        }
        sum
    }
}

fn read_calls(csv_calls: &str) -> Calls {
    csv_calls
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

fn read_input(input: &str) -> (Calls, Vec<BingoBoard>) {}

pub fn part_one(input: &str) -> Option<u32> {
    let (calls, boards) = read_input(input);
    for c in calls {
        for b in boards {
            b.call(c);
            if b.check() {
                return Some(b.score());
            }
        }
    }
    unreachable!("something went wrong.")
}

pub fn part_two(input: &str) -> Option<u32> {
    let (calls, boards) = read_input(input);
    let n_boards = boards.len();
    let mut n_wins = 0;

    for c in calls {
        for b in boards {
            // Skip processing boards that already won.
            if b.win {
                continue;
            }
            b.call(c);
            if b.check() {
                n_wins += 1;
                if n_wins == n_boards {
                    return Some(b.score());
                }
            }
        }
    }

    unreachable!("something went wrong")
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
