use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    /* pop extra newline */
    input.pop();

    let react = react(&input);
    println!("day5, part1: sequence len: {:?}", react.len());

    part2(&input);
    Ok(())
}

fn react(input: &str) -> String {
    let mut seq = String::new();
    for c in input.bytes() {
        if seq.len() > 0 {
            let prevc = seq.as_bytes()[seq.len() - 1];
            let diff = if c > prevc { c - prevc } else { prevc - c };
            if diff == 32 {
                seq.pop();
                continue;
            }
        }
        seq.push(c as char);
    }
    seq
}

fn part2(input: &str) {
    let mut size_without = HashMap::new();

    for c in input.bytes() {
        let c = c.to_ascii_lowercase();
        if size_without.contains_key(&c) {
            continue;
        }
        let polymer = polymer_without_unit(input, c);
        let score = react(&polymer).len();
        size_without.insert(c, score);
    }

    let min = size_without.values().min().unwrap();
    println!("day5, part2: min is {}", min);
}

fn polymer_without_unit(input: &str, unit: u8) -> String {
    let mut polymer = String::new();

    for c in input.bytes() {
        if c.to_ascii_lowercase() != unit {
            polymer.push(c as char);
        }
    }
    polymer
}
