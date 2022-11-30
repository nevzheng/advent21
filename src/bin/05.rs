use std::{
    cmp::{max, min},
    convert::TryInto,
};

use itertools::equal;

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> Self {
        LineSegment { a, b }
    }
}

fn parse_point(point: &str) -> Option<Point> {
    let (a, b) = point.split_once(',')?;
    Some(Point::new(a.parse().ok()?, b.parse().ok()?))
}

fn read_segment(line: &str) -> Option<LineSegment> {
    let (p1, p2) = line.split_once(" -> ")?;

    Some(LineSegment::new(parse_point(p1)?, parse_point(p2)?))
}

fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

fn intersects_points(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}

fn intersects(x: &LineSegment, y: &LineSegment) -> bool {
    intersects_points(&x.a, &x.b, &y.a, &y.b)
}

pub fn part_one(input: &str) -> Option<isize> {
    let segments: Vec<LineSegment> = input.lines().filter_map(read_segment).collect();

    // Determine bounds of grid.
    let mut max_x = 0;
    let mut max_y = 0;
    for s in &segments {
        max_x = max(max_x, s.a.x);
        max_x = max(max_x, s.b.x);
        max_y = max(max_y, s.a.y);
        max_y = max(max_y, s.b.y);
    }

    let mut grid: Vec<Vec<_>> =
        vec![vec![0; (max_y + 1).try_into().unwrap()]; (max_x + 1).try_into().unwrap()];

    for s in segments {
        // Consider only vertical and horizontal lines where  x1=x2 or y1=y2 for segment x1,y1 -> x2,y2
        if s.a.x == s.b.x {
            // Vertical Line
            let start: usize = min(s.a.y, s.b.y).try_into().ok()?;
            let end: usize = max(s.a.y, s.b.y).try_into().ok()?;
            for i in start..=end {
                grid[TryInto::<usize>::try_into(s.a.x).ok()?][i] += 1;
            }
        } else if s.a.y == s.b.y {
            // Horizontal Line
            let start: usize = min(s.a.x, s.b.x).try_into().ok()?;
            let end: usize = max(s.a.x, s.b.x).try_into().ok()?;
            for i in start..=end {
                grid[i][TryInto::<usize>::try_into(s.a.y).ok()?] += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..=TryInto::<usize>::try_into(max_x).ok()? {
        for j in 0..=(TryInto::<usize>::try_into(max_y).ok()?) {
            if grid[i][j] >= 2 {
                ans += 1;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<isize> {
    let segments: Vec<LineSegment> = input.lines().filter_map(read_segment).collect();

    // Determine bounds of grid.
    let mut max_x = 0;
    let mut max_y = 0;
    for s in &segments {
        max_x = max(max_x, s.a.x);
        max_x = max(max_x, s.b.x);
        max_y = max(max_y, s.a.y);
        max_y = max(max_y, s.b.y);
    }

    let mut grid: Vec<Vec<_>> =
        vec![vec![0; (max_y + 1).try_into().unwrap()]; (max_x + 1).try_into().unwrap()];

    for s in segments {
        // Consider only vertical and horizontal lines where  x1=x2 or y1=y2 for segment x1,y1 -> x2,y2
        if s.a.x == s.b.x {
            // Vertical Line
            let start: usize = min(s.a.y, s.b.y).try_into().ok()?;
            let end: usize = max(s.a.y, s.b.y).try_into().ok()?;
            for i in start..=end {
                grid[TryInto::<usize>::try_into(s.a.x).ok()?][i] += 1;
            }
        } else if s.a.y == s.b.y {
            // Horizontal Line
            let start: usize = min(s.a.x, s.b.x).try_into().ok()?;
            let end: usize = max(s.a.x, s.b.x).try_into().ok()?;
            for i in start..=end {
                grid[i][TryInto::<usize>::try_into(s.a.y).ok()?] += 1;
            }
        } else {
            // 45 degree line
            let mut cur = s.a.to_owned();
            let dx: isize = if (s.b.x as isize) - (s.a.x as isize) > 0 {
                1
            } else {
                -1
            };
            let dy: isize = if (s.b.y as isize) - (s.a.y as isize) > 0 {
                1
            } else {
                -1
            };

            while !same_point(&cur, &s.b) {
                grid[cur.x as usize][cur.y as usize] += 1;
                cur.x += dx;
                cur.y += dy;
            }
            grid[cur.x as usize][cur.y as usize] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..=TryInto::<usize>::try_into(max_x).ok()? {
        for j in 0..=(TryInto::<usize>::try_into(max_y).ok()?) {
            if grid[i][j] >= 2 {
                ans += 1;
            }
        }
    }

    Some(ans)
}

fn same_point(a: &Point, b: &Point) -> bool {
    (a.x == b.x) && (a.y == b.y)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(12));
    }
}
