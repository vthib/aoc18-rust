use std::collections::HashMap;
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)
}

fn part1(input: &str) -> Result<()> {
    let mut nb_doubles = 0;
    let mut nb_triples = 0;
    let mut nb_occurs = HashMap::new();

    for line in input.lines() {
        for letter in line.chars() {
            let counter = nb_occurs.entry(letter).or_insert(0);
            *counter += 1;
        }
        for nb_occur in nb_occurs.values() {
            if *nb_occur == 2 {
                nb_doubles += 1;
                break;
            }
        }
        for nb_occur in nb_occurs.values() {
            if *nb_occur == 3 {
                nb_triples += 1;
                break;
            }
        }
        nb_occurs.clear();
    }

    println!("day2, part1: {}", nb_doubles * nb_triples);
    Ok(())
}
