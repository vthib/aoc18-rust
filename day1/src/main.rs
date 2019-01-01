use std::collections::HashSet;
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)
}

fn part1(input: &str) -> Result<()> {
    let mut res = 0;

    for line in input.lines() {
        res += line.parse::<i32>()?;
    }

    println!("day1, part1: {}", res);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut acc = 0;
    let mut map = HashSet::new();

    'outer: loop {
        for line in input.lines() {
            acc += line.parse::<i32>()?;
            if !map.insert(acc) {
                break 'outer;
            }
        }
    }

    println!("day1, part2: {}", acc);
    Ok(())
}

