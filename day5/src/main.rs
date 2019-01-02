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
    Ok(())
}

fn part1(input: &str) {
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

    println!("day5, part1: sequence len: {:?}", seq.len());
}
