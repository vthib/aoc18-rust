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

    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let react_len = react(&input.as_bytes());
    println!("day5, part1: sequence len: {:?}", react_len);
}

fn react(input: &[u8]) -> usize {
    let mut seq = Vec::new();
    for c in input {
        if seq.len() > 0 {
            let prevc = seq[seq.len() - 1];
            let diff = if c > prevc { c - prevc } else { prevc - c };
            if diff == 32 {
                seq.pop();
                continue;
            }
        }
        seq.push(c);
    }
    seq.len()
}

fn part2(input: &str) {
    let mut size_without = HashMap::new();

    for c in input.bytes() {
        let c = c.to_ascii_lowercase();
        if size_without.contains_key(&c) {
            continue;
        }
        let polymer = polymer_without_unit(input, c);
        let score = react(&polymer);
        size_without.insert(c, score);
    }

    let min = size_without.values().min().unwrap();
    println!("day5, part2: min is {}", min);
}

fn polymer_without_unit(input: &str, unit: u8) -> Vec<u8> {
    let mut polymer = Vec::new();

    for c in input.bytes() {
        if c.to_ascii_lowercase() != unit {
            polymer.push(c);
        }
    }
    polymer
}
