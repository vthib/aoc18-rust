use std::io;
use std::io::Read;

use crate::utils::Result;

pub fn run() -> Result<i32> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut res = 0;
    for line in input.lines() {
        res += line.parse::<i32>()?;
    }

    Ok(res)
}
