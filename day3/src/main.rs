use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut claims: Vec<Claim> = Vec::new();

    for line in input.lines() {
        claims.push(line.parse()?);
    }

    println!("day3, part1: {}", part1(&claims));
    Ok(())
}

fn part1(claims: &Vec<Claim>) -> usize {
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in claims {
        for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                *grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    grid.values().filter(|x| **x > 1).count()
}

struct Claim {
    _id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        /* Parse "#{id} @ {x},{y}: {width}x{height}" */
        let (id, x, y, w, h) = scan_fmt!(s, "#{d} @ {d},{d}: {d}x{d}", u32, u32, u32, u32, u32);

        Ok(Claim {
            _id: id.unwrap(),
            x: x.unwrap(),
            y: y.unwrap(),
            width: w.unwrap(),
            height: h.unwrap(),
        })
    }
}
