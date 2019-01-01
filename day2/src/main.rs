use std::collections::HashMap;
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("day2, part1: {}", part1(&input));
    println!("day2, part2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> i32 {
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

    nb_doubles * nb_triples
}

fn part2(input: &str) -> Result<String> {
    let mut signatures = HashMap::new();

    for (line_idx, line) in input.lines().enumerate() {
        /* add all signatures of the word, ie all
         * the words created when removing one letter.
         */
        for i in 0..line.len() {
            let mut sig = String::new();
            sig.push_str(&line[0..i]);
            sig.push_str(&line[(i + 1)..line.len()]);

            match signatures.get(&sig) {
                Some(prev_idx) => {
                    /* prevent matching against the same line,
                     * can happen if a letter is repeated */
                    if *prev_idx != line_idx {
                        return Ok(sig);
                    }
                },
                None => {
                    signatures.insert(sig, line_idx);
                },
            }
        }
    }

    Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        "could not find matching IDs",
    )))
}
