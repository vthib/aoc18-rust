use std::error::Error;
use std::io;
use std::io::Read;
use std::iter::Iterator;
use std::str::FromStr;

use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        points.push(line.parse()?);
    }

    part1(&mut points);
    Ok(())
}

/* {{{ part1 */

fn part1(points: &mut Vec<Point>) {
    let mut prev_width = 0;
    let mut prev_height = 0;

    loop {
        let (xmin, xmax, ymin, ymax) = compute_bounding_box_size(points);
        let width = (xmax - xmin) as usize;
        let height = (ymax - ymin) as usize;

        // loop until the bounding box increases
        if prev_width != 0 && width > prev_width && prev_height != 0 && height > prev_height {
            break;
        }
        prev_width = width;
        prev_height = height;

        // if bounding box is small enough, display the points
        if width < 100 && height < 100 {
            display_points(&points, xmin, width, ymin, height);
        }

        for p in points.iter_mut() {
            p.step();
        }
    }
}

fn compute_bounding_box_size(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut iter = points.iter();
    let first_point = iter.next().unwrap();
    let mut xmin = first_point.x;
    let mut xmax = first_point.x;
    let mut ymin = first_point.y;
    let mut ymax = first_point.y;

    while let Some(p) = iter.next() {
        if p.x < xmin {
            xmin = p.x;
        }
        if p.x > xmax {
            xmax = p.x;
        }
        if p.y < ymin {
            ymin = p.y;
        }
        if p.y > ymax {
            ymax = p.y;
        }
    }

    (xmin, xmax, ymin, ymax)
}

fn display_points(points: &Vec<Point>, xmin: i32, width: usize, ymin: i32, height: usize) {
    let mut grid = vec![vec!['.'; width + 1]; height + 1];

    for p in points {
        grid[(p.y - ymin) as usize][(p.x - xmin) as usize] = '#';
    }

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

/* }}} */
/* {{{ Point */

struct Point {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, vel_x, vel_y) =
            scan_fmt!(s, "position=<{}, {}> velocity=<{}, {}>", i32, i32, i32, i32);
        Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
            vel_x: vel_x.unwrap(),
            vel_y: vel_y.unwrap(),
        })
    }
}

impl Point {
    fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

/* }}} */
