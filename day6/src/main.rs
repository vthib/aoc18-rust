use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::u32::MAX as u32_MAX;

use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        points.push(line.parse()?);
    }

    part1(&points);
    part2(&points);
    Ok(())
}

/* 2d grid, storing the index of the closest point
 * (or None if multiple points are closest) */
type Grid = Vec<Vec<Option<usize>>>;

fn part1(points: &Vec<Point>) {
    let (max_x, max_y) = compute_bounds(&points);
    let mut grid = Grid::new();

    /* build grid of closest points */
    for x in 0..max_x {
        let mut row = Vec::new();
        for y in 0..max_y {
            row.push(get_closest_point(&points, x, y));
        }
        grid.push(row);
    }

    /* count areas */
    let mut areas = HashMap::new();
    for row in &grid {
        for elem in row {
            if let Some(idx) = elem {
                *areas.entry(idx).or_insert(0) += 1;
            }
        }
    }

    /* build set of excluded points: if a point on the boundary is closest to a point, this
     * point's voronoi diagram is infinite */
    let mut exclude = HashSet::new();
    for x in &[0, max_x - 1] {
        for y in 0..max_y {
            if let Some(idx) = grid[*x as usize][y as usize] {
                exclude.insert(idx);
            }
        }
    }
    for y in &[0, max_y - 1] {
        for x in 0..max_x {
            if let Some(idx) = grid[x as usize][*y as usize] {
                exclude.insert(idx);
            }
        }
    }

    /* find max */
    let mut max_area = 0;
    for (idx, area) in areas {
        if !exclude.contains(&idx) {
            max_area = std::cmp::max(max_area, area);
        }
    }
    println!("day6, part1: max area: {}", max_area);
}

fn part2(points: &Vec<Point>) {
    let (max_x, max_y) = compute_bounds(&points);
    let mut grid = Vec::new();

    /* build grid of sum of manhattan distances */
    for x in 0..max_x {
        let mut row: Vec<u32> = Vec::new();
        for y in 0..max_y {
            row.push(
                points
                    .iter()
                    .map(|p| p.get_manhattan_distance(x, y))
                    .sum()
            );
        }
        grid.push(row);
    }

    let mut nb_safe = 0;
    for row in &grid {
        for sum_dist in row {
            if *sum_dist < 10_000 {
                nb_safe += 1;
            }
        }
    }
    println!("day6, part2: safe area: {}", nb_safe);
}

fn compute_bounds(points: &Vec<Point>) -> (u32, u32) {
    (
        points.iter().max_by_key(|p| p.x).unwrap().x + 1,
        points.iter().max_by_key(|p| p.y).unwrap().y + 1,
    )
}

fn get_closest_point(points: &Vec<Point>, x: u32, y: u32) -> Option<usize> {
    let mut closest = None;
    let mut min_dist = u32_MAX;

    for (index, point) in points.iter().enumerate() {
        let dist = point.get_manhattan_distance(x, y);

        if dist == min_dist {
            closest = None;
        } else if dist < min_dist {
            min_dist = dist;
            closest = Some(index);
        }
    }

    closest
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn get_manhattan_distance(&self, x: u32, y: u32) -> u32 {
        (if x > self.x { x - self.x } else { self.x - x })
            + (if y > self.y { y - self.y } else { self.y - y })
    }
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = scan_fmt!(s, "{d}, {d}", u32, u32);

        Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}
