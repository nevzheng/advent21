use itertools::Itertools;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
type Calls = Vec<u32>;
type Board = Vec<Vec<u32>>;
type Marks = Vec<Vec<bool>>;

struct BingoBoard {
    board: Board,
    marks: Marks,
    win: bool,
}

fn check_row(marks: &Marks, row_id: usize) -> bool {
    for j in 0..WIDTH {
        if !marks[row_id][j] {
            return false;
        }
    }
    true
}

fn check_col(marks: &Marks, col_id: usize) -> bool {
    marks.iter().all(|v| v[col_id])
}

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

    fn check(&mut self) -> bool {
        for i in 0..HEIGHT {
            if check_row(&self.marks, i) {
                self.win = true;
                return true;
            }
        }

        for j in 0..WIDTH {
            if check_col(&self.marks, j) {
                self.win = true;
                return true;
            }
        }
        false
    }

    fn score(&self, just_called: u32) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if !self.marks[i][j] {
                    sum += self.board[i][j]
                }
            }
        }
        just_called * sum
    }

    fn new(board: Board) -> BingoBoard {
        BingoBoard {
            board,
            marks: vec![vec![false; 5]; 5],
            win: false,
        }
    }
}

fn read_calls(csv_calls: &str) -> Calls {
    csv_calls
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

fn read_rows(row: &str) -> Vec<u32> {
    row.split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

fn read_input(input: &str) -> Option<(Calls, Vec<BingoBoard>)> {
    let mut lines = input.lines();

    let calls = read_calls(lines.next()?);

    let mut boards: Vec<BingoBoard> = Vec::new();

    for chunk in &lines.into_iter().chunks(6) {
        let b: Board = chunk.skip(1).map(read_rows).collect();
        boards.push(BingoBoard::new(b));
    }

    Some((calls, boards))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (calls, mut boards) = read_input(input)?;
    for c in calls {
        for b in &mut boards {
            b.call(c);
            if b.check() {
                return Some(b.score(c));
            }
        }
    }
    unreachable!("something went wrong.")
}

pub fn part_two(input: &str) -> Option<u32> {
    let (calls, mut boards) = read_input(input)?;
    let n_boards = boards.len();
    let mut n_wins = 0;

    for c in calls {
        for b in &mut boards {
            // Skip processing boards that already won.
            if b.win {
                continue;
            }
            b.call(c);
            if b.check() {
                n_wins += 1;
                if n_wins == n_boards {
                    return Some(b.score(c));
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
        assert_eq!(part_one(&input), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(1924));
    }
}
